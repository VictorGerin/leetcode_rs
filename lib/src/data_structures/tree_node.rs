use std::{rc::Rc, cell::RefCell, collections::VecDeque};
use crate::parser::Val;

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn as_post_ordem_vec(&self) -> Vec<i32> {
        TreeNodeIterPostOrder::new(Some(Rc::new(RefCell::new(self.clone())))).map(|x| x.borrow().val).collect()
    }
}

// Implementação de Drop iterativo para evitar stack overflow em árvores profundas
impl Drop for TreeNode {
    fn drop(&mut self) {
        // Usar uma stack na heap para processar os nós iterativamente
        // ao invés de recursivamente, evitando stack overflow
        let mut stack: Vec<TreeNodeRef> = Vec::new();
        
        // Extrair os filhos do nó atual antes do drop padrão
        // Isso evita que o drop padrão cause recursão
        if let Some(left) = self.left.take() {
            stack.push(left);
        }
        
        if let Some(right) = self.right.take() {
            stack.push(right);
        }
        
        // Processar todos os nós na stack iterativamente
        while let Some(node_ref) = stack.pop() {
            // Tentar obter o RefCell interno se este for o último Rc
            // Se houver outras referências, apenas decrementa o contador
            if let Ok(cell) = Rc::try_unwrap(node_ref) {
                // Este é o último Rc, podemos dropar o RefCell
                let mut node = cell.into_inner();
                
                // Extrair os filhos antes do drop recursivo
                if let Some(left) = node.left.take() {
                    stack.push(left);
                }
                
                if let Some(right) = node.right.take() {
                    stack.push(right);
                }
                
                // O drop padrão do TreeNode será chamado automaticamente aqui
                // Mas como já removemos left e right, não haverá recursão
            }
            // Se houver outras referências ao Rc, ele apenas decrementa o contador
            // e não faz drop do conteúdo, então não há problema de recursão
        }
        
        // O drop padrão do TreeNode será chamado automaticamente
        // Mas como já removemos left e right com take(), não há mais referências
        // para causar recursão
    }
}

impl FromIterator<Val> for TreeNodeRef {
    fn from_iter<I: IntoIterator<Item=Val>>(iter: I) -> TreeNodeRef {
        let mut iter = iter.into_iter();

        // Função helper: converte Val em Option<TreeNodeRef>
        let val_to_node = |val: Val| -> Option<TreeNodeRef> {
            match val {
                Val::Int(v) => Some(Rc::new(RefCell::new(TreeNode::new(v as i32)))),
                Val::None => None,
                _ => panic!("Invalid input: expected integer or null")
            }
        };

        // Criar a raiz a partir do primeiro elemento
        let root = match iter.next() {
            None => Rc::new(RefCell::new(TreeNode::new(0))),
            Some(val) => val_to_node(val).unwrap_or_else(|| Rc::new(RefCell::new(TreeNode::new(0)))),
        };
        
        // Fila para processar nós em nível (level-order)
        let mut queue: VecDeque<Option<TreeNodeRef>> = VecDeque::new();
        queue.push_back(Some(root.clone()));

        // Processar os elementos restantes do iterador
        // No formato LeetCode level-order:
        // - Apenas nós não-null na fila consomem elementos (2 por nó: left e right)
        // - Nós null na fila são ignorados (não consomem elementos)
        while let Some(parent_opt) = queue.pop_front() {
            // Se o pai é None, ignoramos (não consome elementos do iterador)
            let Some(parent) = parent_opt else {
                continue;
            };
        
            // Pegar os próximos 2 elementos: left e right
            // Se o iterador se esgotar, paramos
            let left_val = match iter.next() {
                Some(val) => val,
                None => break,
            };

            // Processar filho esquerdo
            let left_node = val_to_node(left_val);
            parent.borrow_mut().left = left_node.clone();
            queue.push_back(left_node);

            let right_val = match iter.next() {
                Some(val) => val,
                None => break
            };

            // Processar filho direito
            let right_node = val_to_node(right_val);
            parent.borrow_mut().right = right_node.clone();
            queue.push_back(right_node);
        }

        root
    }
} 



enum StackItem {
    Unvisited(TreeNodeRef),
    Visited(TreeNodeRef),
}

pub struct TreeNodeIterPostOrder {
    stack: Vec<StackItem>,
}

impl Iterator for TreeNodeIterPostOrder {
    type Item = TreeNodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            match item {
                StackItem::Visited(node) => {
                    // Já processamos os filhos, retornar o nó
                    return Some(node);
                }
                StackItem::Unvisited(node) => {
                    // Marcar como visitado e empilhar filhos
                    // Empilhamos na ordem: pai (visited), direito, esquerdo
                    // Assim processamos: esquerdo, direito, pai (pós-ordem)
                    self.stack.push(StackItem::Visited(node.clone()));
                    
                    if let Some(right) = node.borrow().right.clone() {
                        self.stack.push(StackItem::Unvisited(right));
                    }
                    
                    if let Some(left) = node.borrow().left.clone() {
                        self.stack.push(StackItem::Unvisited(left));
                    }
                }
            }
        }
        None
    }
}

impl TreeNodeIterPostOrder {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            stack: match root {
                Some(root) => vec![StackItem::Unvisited(root)],
                None => vec![]
            }
        }
    }
}
# 1339. Maximum Product of Splitted Binary Tree

## Descrição

Dada a raiz de uma árvore binária, divida a árvore em duas subárvores removendo uma aresta de forma que o produto das somas das duas subárvores seja maximizado. Retorne este produto máximo módulo \(10^9 + 7\).

**Exemplo 1:**

```
Input: root = [1,2,3,4,5,6]
Output: 110
```

Explicação: Remover a aresta entre os nós 1 e 2 resulta em duas subárvores com somas 11 e 10. Seu produto é \(11 \times 10 = 110\).

**Exemplo 2:**

```
Input: root = [1,null,2,3,4,null,null,5,6]
Output: 90
```

Explicação: Remover a aresta entre os nós 2 e 3 resulta em duas subárvores com somas 15 e 6. Seu produto é \(15 \times 6 = 90\).

**Restrições:**

- O número de nós na árvore está no intervalo \([2, 5 \times 10^4]\).
- O valor de cada nó está entre 1 e \(10^4\).

## Link

https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/

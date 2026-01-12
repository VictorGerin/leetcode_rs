# 865. Smallest Subtree with all the Deepest Nodes

## Descrição

Dada a raiz de uma árvore binária, onde a profundidade de cada nó é definida como a menor distância até a raiz, a tarefa é identificar a menor subárvore que contém todos os nós mais profundos na árvore original.

**Definições:**

- **Nó Mais Profundo:** Um nó é considerado mais profundo se tiver a maior profundidade possível entre qualquer nó na árvore inteira.

- **Subárvore:** A subárvore de um nó consiste desse nó e todos os seus descendentes.

**Exemplo:**

Considere a árvore binária representada como:

```
        3
       / \
      5   1
     /|   |\
    6 2   0 8
      |\
      7 4
```

Nesta árvore, os nós mais profundos são 7 e 4, ambos na profundidade 4. A menor subárvore contendo ambos esses nós está enraizada no nó 2.

**Restrições:**

- O número de nós na árvore está entre 1 e 500.
- O valor de cada nó é único e varia de 0 a 500.

Este problema é equivalente ao LeetCode Problem 1123: "Lowest Common Ancestor of Deepest Leaves."

## Link

https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/

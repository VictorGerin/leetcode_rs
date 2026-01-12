# 1577. Number of Ways Where Square of Number Is Equal to Product of Two Numbers

## Descrição

Você recebe dois arrays de inteiros, `nums1` e `nums2`. Sua tarefa é contar o número total de triplas válidas que podem ser formadas sob duas condições específicas:

- **Tipo 1 de Tripla:** Uma tripla `(i, j, k)` onde:
  - `i` é um índice de `nums1`.
  - `j` e `k` são índices de `nums2` com `j < k`.
  - A relação `nums1[i]² == nums2[j] * nums2[k]` é válida.

- **Tipo 2 de Tripla:** Uma tripla `(i, j, k)` onde:
  - `i` é um índice de `nums2`.
  - `j` e `k` são índices de `nums1` com `j < k`.
  - A relação `nums2[i]² == nums1[j] * nums1[k]` é válida.

Em termos mais simples:

- Para Tipo 1: Pegue um elemento de `nums1`, eleve ao quadrado e verifique se é igual ao produto de quaisquer dois elementos de `nums2` (onde o segundo elemento vem depois do primeiro).
- Para Tipo 2: Pegue um elemento de `nums2`, eleve ao quadrado e verifique se é igual ao produto de quaisquer dois elementos de `nums1` (onde o segundo elemento vem depois do primeiro).

A função deve retornar a contagem total de todas as triplas válidas (Tipo 1 e Tipo 2 combinadas).

**Exemplo:**

Se `nums1 = [7, 4]` e `nums2 = [5, 2, 8, 9]`:

- Uma tripla Tipo 1 poderia ser `(1, 1, 2)` porque `nums1[1]² = 16 = nums2[1] * nums2[2] = 2 * 8`.

A função deve retornar `1` pois existe uma tripla válida.

**Restrições:**

- `1 <= nums1.length, nums2.length <= 1000`
- `1 <= nums1[i], nums2[i] <= 10⁵`

## Link

https://leetcode.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/

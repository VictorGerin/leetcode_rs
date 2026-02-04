# 3314. Construct the Minimum Bitwise Array I

## Descrição

Você recebe um array `nums` de tamanho `n`, onde cada elemento `nums[i]` é um número primo.

Você precisa construir um array `ans`, também de tamanho `n`, de modo que para cada índice `i`:

```
ans[i] OR (ans[i] + 1) == nums[i]
```

Além disso, cada `ans[i]` deve ser o menor valor possível que satisfaz essa condição. Se não for possível encontrar tal valor para `nums[i]`, então `ans[i]` deve ser `-1`.

**Exemplo 1:**

```
Input: nums = [2, 3, 5, 7]
Output: [-1, 1, 4, 3]
```

**Exemplo 2:**

```
Input: nums = [11, 13, 31]
Output: [9, 12, 15]
```

**Restrições:**

- `1 <= nums.length <= 100`
- `2 <= nums[i] <= 1000`
- Todos os `nums[i]` são primos.

## Link

https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/description/?envType=daily-question&envId=2026-01-20

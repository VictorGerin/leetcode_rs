# 1929. Concatenation of Array

## Descrição

Dado um array de inteiros `nums` de comprimento `n`, crie um array `ans` de comprimento `2n` tal que:

- `ans[i] == nums[i]` para `0 <= i < n`
- `ans[i + n] == nums[i]` para `0 <= i < n`

Em essência, `ans` é formado pela concatenação de duas cópias de `nums`.

**Exemplo 1:**

```
Input: nums = [1, 2, 1]
Output: [1, 2, 1, 1, 2, 1]
```

**Exemplo 2:**

```
Input: nums = [1, 3, 2, 1]
Output: [1, 3, 2, 1, 1, 3, 2, 1]
```

**Restrições:**

- `n == nums.length`
- `1 <= n <= 1000`
- `1 <= nums[i] <= 1000`

## Link

https://leetcode.com/problems/concatenation-of-array/

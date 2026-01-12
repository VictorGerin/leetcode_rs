# 1470. Shuffle the Array

## Descrição

Você recebe um array `nums` consistindo de `2n` elementos, estruturado como `[x₁, x₂, ..., xₙ, y₁, y₂, ..., yₙ]`. Sua tarefa é reorganizar o array na forma `[x₁, y₁, x₂, y₂, ..., xₙ, yₙ]`.

**Exemplo 1:**

```
Input: nums = [2,5,1,3,4,7], n = 3
Output: [2,3,5,4,1,7]
```

Explicação: A primeira metade `[2,5,1]` e a segunda metade `[3,4,7]` são entrelaçadas para formar `[2,3,5,4,1,7]`.

**Exemplo 2:**

```
Input: nums = [1,2,3,4,4,3,2,1], n = 4
Output: [1,4,2,3,3,2,4,1]
```

**Exemplo 3:**

```
Input: nums = [1,1,2,2], n = 2
Output: [1,2,1,2]
```

**Restrições:**

- `1 <= n <= 500`
- `nums.length == 2n`
- `1 <= nums[i] <= 10³`

## Link

https://leetcode.com/problems/shuffle-the-array/

# 1800. Maximum Ascending Subarray Sum

## Descrição

Dado um array de inteiros positivos `nums`, retorne a soma máxima possível de uma subarray crescente em `nums`.

Uma subarray é definida como uma sequência contígua de números em um array.

Uma subarray `[nums[l], nums[l+1], ..., nums[r-1], nums[r]]` é crescente se para todo `i` onde `l <= i < r`, `nums[i] < nums[i+1]`. Note que uma subarray de tamanho `1` é crescente.

**Exemplo 1:**

```
Input: nums = [10,20,30,5,10,50]
Output: 65
```

Explicação: A subarray crescente `[5,10,50]` tem a soma máxima de `65`.

**Exemplo 2:**

```
Input: nums = [10,20,30,40,50]
Output: 150
```

Explicação: Todo o array `[10,20,30,40,50]` é crescente com uma soma de `150`.

**Exemplo 3:**

```
Input: nums = [12,17,15,13,10,11,12]
Output: 33
```

Explicação: A subarray crescente `[10,11,12]` tem a soma máxima de `33`.

**Restrições:**

- `1 <= nums.length <= 100`
- `1 <= nums[i] <= 100`

## Link

https://leetcode.com/problems/maximum-ascending-subarray-sum/

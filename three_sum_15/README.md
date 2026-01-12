# 15. 3Sum

## Descrição

Dado um array de inteiros `nums`, retorne todas as triplas `[nums[i], nums[j], nums[k]]` tais que:

- `i != j`, `i != k` e `j != k`
- `nums[i] + nums[j] + nums[k] == 0`

O conjunto de soluções não deve conter triplas duplicadas.

**Exemplo:**

```
Input: nums = [-1, 0, 1, 2, -1, -4]
Output: [[-1, -1, 2], [-1, 0, 1]]
```

**Restrições:**

- `3 <= nums.length <= 3000`
- `-10^5 <= nums[i] <= 10^5`

Uma abordagem comum para resolver este problema é primeiro ordenar o array e então usar uma técnica de dois ponteiros para encontrar pares que somam o negativo do elemento atual. Este método ajuda a identificar eficientemente triplas únicas sem duplicatas.

## Link

https://leetcode.com/problems/3sum/

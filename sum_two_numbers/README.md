# 1. Two Sum

## Descrição

Dado um array de inteiros `nums` e um inteiro `target`, retorne os índices dos dois números de forma que eles somem `target`.

**Restrições:**

- Cada entrada terá exatamente uma solução.
- Você não pode usar o mesmo elemento duas vezes.
- Você pode retornar a resposta em qualquer ordem.

**Exemplo:**

```
Input: nums = [2, 7, 11, 15], target = 9
Output: [0, 1]
```

Explicação: Porque `nums[0] + nums[1] == 9`, retornamos `[0, 1]`.

**Abordagem:**

Uma abordagem comum e eficiente para resolver este problema é usar uma tabela hash (dicionário). Conforme você itera pelo array, pode verificar se o complemento do número atual (ou seja, `target - número_atual`) existe na tabela hash. Se existir, você encontrou os dois números que somam o target. Este método tem complexidade de tempo O(n) e complexidade de espaço O(n).

## Link

https://leetcode.com/problems/two-sum/

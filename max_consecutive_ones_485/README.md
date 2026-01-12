# 485. Max Consecutive Ones

## Descrição

Dado um array binário `nums`, retorne o número máximo de `1`'s consecutivos no array.

**Exemplo:**

```
Input: nums = [1,1,0,1,1,1]
Output: 3
```

Explicação: Os primeiros dois dígitos ou os últimos três dígitos são 1s consecutivos. O número máximo de 1s consecutivos é 3.

**Restrições:**

- `1 <= nums.length <= 10^5`
- `nums[i]` é `0` ou `1`.

O objetivo é percorrer o array e determinar a sequência mais longa de `1`s consecutivos. Uma abordagem comum envolve iterar pelo array mantendo uma contagem de `1`s consecutivos e atualizando a contagem máxima encontrada.

## Link

https://leetcode.com/problems/max-consecutive-ones/

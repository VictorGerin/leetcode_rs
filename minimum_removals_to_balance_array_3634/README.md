# 3634. Minimum Removals to Balance Array

## Descrição

Você recebe um array de inteiros `nums` e um inteiro `k`. Um array é considerado **balanceado** se seu elemento máximo for no máximo `k` vezes seu elemento mínimo. Você pode remover qualquer quantidade de elementos de `nums`, sem deixá-lo vazio. Retorne o **número mínimo de elementos a remover** para que o array restante seja balanceado.

Um array com um único elemento é sempre balanceado.

**Link:** https://leetcode.com/problems/minimum-removals-to-balance-array/description/?envType=daily-question&envId=2026-02-06

## Exemplos

- `nums = [2,1,5], k = 2` → Saída: `1` (remover 5; max=2, min=1, 2 ≤ 1×2)
- `nums = [1,6,2,9], k = 3` → Saída: `2` (remover 1 e 9; max=6, min=2, 6 ≤ 2×3)
- `nums = [4,6], k = 2` → Saída: `0` (já balanceado; 6 ≤ 4×2)

## Restrições

- 1 ≤ nums.length ≤ 10⁵
- 1 ≤ nums[i] ≤ 10⁹
- 1 ≤ k ≤ 10⁵

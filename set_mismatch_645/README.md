# 645. Set Mismatch

## Descrição

Você tem um conjunto de inteiros `s` que originalmente contém todos os números de `1` a `n`. Devido a um erro, um número no conjunto foi duplicado, resultando em outro número faltando. Dado um array de inteiros `nums` representando este conjunto errôneo, sua tarefa é identificar o número que ocorre duas vezes e o número que está faltando, e retorná-los na forma de um array.

**Exemplo 1:**

```
Input: nums = [1,2,2,4]
Output: [2,3]
```

Explicação: 2 aparece duas vezes e 3 está faltando.

**Exemplo 2:**

```
Input: nums = [1,1]
Output: [1,2]
```

Explicação: 1 aparece duas vezes e 2 está faltando.

**Restrições:**

- `2 ≤ nums.length ≤ 10^4`
- `1 ≤ nums[i] ≤ 10^4`

Para resolver este problema, você pode usar várias abordagens:

1. **Abordagem Matemática:**
   - Calcule a soma esperada de números de `1` a `n` usando a fórmula `s1 = (1 + n) * n / 2`.
   - Calcule a soma dos números únicos em `nums` (`s2`) e a soma de todos os números em `nums` (`s`).
   - O número duplicado é `s - s2`, e o número faltando é `s1 - s2`.

2. **Abordagem com Tabela Hash:**
   - Use uma tabela hash para contar as ocorrências de cada número em `nums`.
   - Identifique o número que aparece duas vezes e o número que está faltando com base nas contagens.

3. **Abordagem com Manipulação de Bits:**
   - Utilize operações XOR para encontrar os números duplicado e faltando explorando as propriedades do XOR.

## Link

https://leetcode.com/problems/set-mismatch/

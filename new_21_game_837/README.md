# 837. New 21 Game

## Descrição

Alice joga um jogo similar ao Blackjack com as seguintes regras:

- Alice começa com 0 pontos.
- Ela continua sorteando números enquanto sua pontuação for menor que `k`.
- Cada sorteio adiciona um inteiro aleatório entre 1 e `maxPts` (inclusive) à sua pontuação, com todos os resultados igualmente prováveis.
- Uma vez que sua pontuação atinge `k` ou mais, ela para de sortear.

O objetivo é determinar a probabilidade de que a pontuação final de Alice seja `n` ou menos.

**Exemplo 1:**

```
Input: n = 10, k = 1, maxPts = 10
Output: 1.00000
Explicação: Alice sorteia uma carta e para.
```

**Exemplo 2:**

```
Input: n = 6, k = 1, maxPts = 10
Output: 0.60000
Explicação: Alice sorteia uma carta e para. Em 6 de 10 possibilidades, sua pontuação é 6 ou menos.
```

**Exemplo 3:**

```
Input: n = 21, k = 17, maxPts = 10
Output: 0.73278
```

**Restrições:**

- `0 <= k <= n <= 10^4`
- `1 <= maxPts <= 10^4`
- A solução deve ser precisa dentro de `10^-5` da probabilidade real.

## Link

https://leetcode.com/problems/new-21-game/description/

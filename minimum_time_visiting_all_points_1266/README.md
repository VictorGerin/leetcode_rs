# 1266. Minimum Time Visiting All Points

## Descrição

Em um plano 2D, existem `n` pontos com coordenadas inteiras `points[i] = [xi, yi]`. Sua tarefa é determinar o tempo mínimo em segundos necessário para visitar todos os pontos na ordem especificada pelo array `points`.

**Regras de Movimento:**

- Em um segundo, você pode:
  - Mover verticalmente por uma unidade.
  - Mover horizontalmente por uma unidade.
  - Mover diagonalmente por uma unidade em ambas as direções vertical e horizontal simultaneamente.

- Você deve visitar os pontos na sequência exata em que aparecem no array.

- Passar por pontos que aparecem mais tarde na sequência é permitido, mas estes não contam como visitas.

**Exemplo 1:**

```
Input: points = [[1,1],[3,4],[-1,0]]
Output: 7
```

Explicação: Um caminho ótimo é:

```
[1,1] -> [2,2] -> [3,3] -> [3,4] -> [2,3] -> [1,2] -> [0,1] -> [-1,0]
```

Tempo de [1,1] para [3,4] = 3 segundos.

Tempo de [3,4] para [-1,0] = 4 segundos.

Tempo total = 7 segundos.

**Exemplo 2:**

```
Input: points = [[3,2],[-2,2]]
Output: 5
```

**Restrições:**

- `points.length == n`
- `1 <= n <= 100`
- `points[i].length == 2`
- `-1000 <= points[i][0], points[i][1] <= 1000`

## Link

https://leetcode.com/problems/minimum-time-visiting-all-points/

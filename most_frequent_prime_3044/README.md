# 3044. Most Frequent Prime

## Descrição

Você recebe uma matriz 2D `m x n` indexada em 0 `mat`. De cada célula, você pode criar números:

- Selecionando um dos oito caminhos possíveis: leste, sudeste, sul, sudoeste, oeste, noroeste, norte e nordeste.
- Anexando dígitos ao longo do caminho escolhido para formar números.
- Gerando números a cada passo; por exemplo, se os dígitos ao longo do caminho são `1, 9, 1`, os números formados são `1`, `19` e `191`.

A tarefa é retornar o número primo mais frequente maior que 10 entre todos os números criados ao percorrer a matriz. Se nenhum número primo existir, retorne `-1`. Se múltiplos números primos tiverem a maior frequência, retorne o maior entre eles.

**Exemplo 1:**

```
Input: mat = [[1,1],
              [9,9],
              [1,1]]
Output: 19
```

Explicação: Vários números são formados ao percorrer a matriz, e entre eles, `19` é o número primo mais frequente maior que 10.

**Exemplo 2:**

```
Input: mat = [[7]]
Output: -1
```

Explicação: O único número que pode ser formado é `7`, que é primo mas não é maior que 10.

**Exemplo 3:**

```
Input: mat = [[9,7,8],
              [4,6,5],
              [2,8,6]]
Output: 97
```

Explicação: Entre os números formados, `97` é o número primo mais frequente maior que 10.

**Restrições:**

- `m == mat.length`
- `n == mat[i].length`
- `1 <= m, n <= 6`
- `1 <= mat[i][j] <= 9`

## Link

https://leetcode.com/problems/most-frequent-prime/

# 66. Plus One

## Descrição

Você recebe um array não vazio de dígitos decimais representando um inteiro não negativo. Os dígitos são armazenados de forma que o dígito mais significativo está no início da lista, e cada elemento no array contém um único dígito.

Sua tarefa é incrementar o inteiro em um e retornar o array de dígitos resultante.

**Exemplo 1:**

```
Input: digits = [1, 2, 3]
Output: [1, 2, 4]
```

Explicação: O array representa o inteiro 123. Incrementar em um resulta em 124.

**Exemplo 2:**

```
Input: digits = [4, 3, 2, 1]
Output: [4, 3, 2, 2]
```

Explicação: O array representa o inteiro 4321. Incrementar em um resulta em 4322.

**Exemplo 3:**

```
Input: digits = [9, 9, 9]
Output: [1, 0, 0, 0]
```

Explicação: O array representa o inteiro 999. Incrementar em um resulta em 1000, que requer um dígito adicional.

**Restrições:**

- `1 <= digits.length <= 100`
- `0 <= digits[i] <= 9`
- O inteiro não contém zeros à esquerda, exceto para o próprio número 0.

## Link

https://leetcode.com/problems/plus-one/

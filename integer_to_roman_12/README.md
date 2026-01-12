# 12. Integer to Roman

## Descrição

Converta um inteiro em sua representação em numeral romano. Os numerais romanos são compostos por sete símbolos:

| Símbolo | Valor |
|---------|-------|
| I       | 1     |
| V       | 5     |
| X       | 10    |
| L       | 50    |
| C       | 100   |
| D       | 500   |
| M       | 1000  |

Os números são tipicamente escritos do maior para o menor da esquerda para a direita. No entanto, para evitar quatro símbolos idênticos consecutivos, a notação subtrativa é usada:

- `I` pode preceder `V` (5) e `X` (10) para formar 4 (`IV`) e 9 (`IX`).
- `X` pode preceder `L` (50) e `C` (100) para formar 40 (`XL`) e 90 (`XC`).
- `C` pode preceder `D` (500) e `M` (1000) para formar 400 (`CD`) e 900 (`CM`).

**Exemplos de Conversão:**

- 3 → `III`
- 4 → `IV`
- 9 → `IX`
- 58 → `LVIII` (50 + 5 + 3)
- 1994 → `MCMXCIV` (1000 + 900 + 90 + 4)

A tarefa é implementar uma função que converte um inteiro dado (dentro do intervalo de 1 a 3999) para seu numeral romano correspondente.

## Link

https://leetcode.com/problems/integer-to-roman/

# 2787. Ways to Express an Integer as Sum of Powers

## Descrição

Dados dois inteiros positivos `n` e `x`, determine o número de maneiras que `n` pode ser expresso como a soma da `x`-ésima potência de inteiros positivos únicos. Em outras palavras, encontre o número de conjuntos distintos de inteiros únicos `[n₁, n₂, ..., nₖ]` tais que:

```
n = n₁ˣ + n₂ˣ + ... + nₖˣ
```

Como o resultado pode ser muito grande, retorne-o módulo `10⁹ + 7`.

**Exemplo:**

Se `n = 160` e `x = 3`, uma possível maneira de expressar `n` é:

```
160 = 2³ + 3³ + 5³ = 8 + 27 + 125
```

**Restrições:**

- `1 <= n <= 300`
- `1 <= x <= 5`

## Link

https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/

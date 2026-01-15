# 8. String to Integer (atoi)

## Descrição

Implemente a função `myAtoi(string s)`, que converte uma string em um inteiro de 32 bits assinado (similar à função `atoi` em C/C++).

O algoritmo para `myAtoi(string s)` é o seguinte:

1. **Ignorar espaços em branco à esquerda**: Leia e ignore quaisquer espaços em branco à esquerda até encontrar o primeiro caractere não-whitespace.

2. **Verificar o sinal**: Verifique se o próximo caractere (se não estiver no final da string) é '-' ou '+'. Leia este caractere se presente. Isso determina se o resultado final é negativo ou positivo, respectivamente. Assuma que o resultado é positivo se nenhum sinal estiver presente.

3. **Ler dígitos**: Continue lendo os caracteres até encontrar um caractere não-dígito ou o final da string. O resto da string é ignorado.

4. **Converter esses dígitos em um inteiro**: Se nenhum dígito foi lido, então o inteiro é 0. Ajuste o sinal conforme necessário (do passo 2).

5. **Tratar overflow**: Se o inteiro estiver fora do intervalo de inteiros de 32 bits assinados `[-2³¹, 2³¹ - 1]`, então fixe o inteiro para permanecer no intervalo. Especificamente, inteiros menores que `-2³¹` devem ser fixados em `-2³¹`, e inteiros maiores que `2³¹ - 1` devem ser fixados em `2³¹ - 1`.

6. **Retornar o inteiro como resultado final**.

**Nota:**
- Apenas o caractere de espaço `' '` é considerado um caractere de espaço em branco.
- Não ignore nenhum caractere que não seja o caractere de espaço em branco à esquerda ou o resto da string após os dígitos.

**Exemplo 1:**

```
Input: s = "42"
Output: 42
```

Explicação: Os caracteres sublinhados são o que é lido, o cursor é a posição atual.
```
Passo 1: "42" (nenhum caractere à esquerda é lido porque não há espaços em branco à esquerda)
         ^
Passo 2: "42" (nenhum sinal é lido porque não há sinal '+' ou '-')
         ^
Passo 3: "42" ("42" é lido)
           ^
```

**Exemplo 2:**

```
Input: s = "   -42"
Output: -42
```

Explicação:
```
Passo 1: "   -42" (espaços em branco à esquerda são lidos e ignorados)
            ^
Passo 2: "   -42" ('-' é lido, então o resultado deve ser negativo)
             ^
Passo 3: "   -42" ("42" é lido)
               ^
```

**Exemplo 3:**

```
Input: s = "4193 with words"
Output: 4193
```

Explicação:
```
Passo 1: "4193 with words" (nenhum caractere à esquerda é lido porque não há espaços em branco à esquerda)
         ^
Passo 2: "4193 with words" (nenhum sinal é lido porque não há sinal '+' ou '-')
         ^
Passo 3: "4193 with words" ("4193" é lido; a leitura para porque o próximo caractere é um não-dígito)
             ^
```

**Exemplo 4:**

```
Input: s = "words and 987"
Output: 0
```

Explicação:
```
Passo 1: "words and 987" (nenhum caractere à esquerda é lido porque não há espaços em branco à esquerda)
         ^
Passo 2: "words and 987" (nenhum sinal é lido porque não há sinal '+' ou '-')
         ^
Passo 3: "words and 987" (nenhum dígito é lido porque o primeiro caractere não-whitespace é 'w')
         ^
```

**Exemplo 5:**

```
Input: s = "-91283472332"
Output: -2147483648
```

Explicação:
```
Passo 1: "-91283472332" (nenhum caractere à esquerda é lido porque não há espaços em branco à esquerda)
         ^
Passo 2: "-91283472332" ('-' é lido, então o resultado deve ser negativo)
          ^
Passo 3: "-91283472332" ("91283472332" é lido)
                     ^
O inteiro "-91283472332" está fora do intervalo de um inteiro de 32 bits assinado.
O resultado é fixado em -2³¹ = -2147483648.
```

**Restrições:**

- `0 <= s.length <= 200`
- `s` consiste em letras em inglês (maiúsculas e minúsculas), dígitos (0-9), `' '`, `'+'`, `'-'` e `'.'`.

## Link

https://leetcode.com/problems/string-to-integer-atoi/

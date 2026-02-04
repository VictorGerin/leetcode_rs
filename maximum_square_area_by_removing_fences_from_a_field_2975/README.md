# 2975. Maximum Square Area by Removing Fences from a Field

## Descrição

Você tem um campo retangular grande de \((m - 1) \times (n - 1)\) com cantos em \((1, 1)\) e \((m, n)\). Dentro do campo existem algumas cercas horizontais e verticais:

- **Cercas horizontais** são colocadas ao longo de linhas de \((hFences[i], 1)\) até \((hFences[i], n)\).
- **Cercas verticais** são colocadas ao longo de linhas de \((1, vFences[i])\) até \((m, vFences[i])\).

Você pode **remover** algumas dessas cercas internas (ou nenhuma), mas existem cercas de limite que você *não pode remover*:

- Limites horizontais em \(x = 1\) e \(x = m\)
- Limites verticais em \(y = 1\) e \(y = n\)

Sua tarefa é selecionar duas linhas horizontais (do conjunto de cercas horizontais mais as duas cercas de limite) e duas linhas verticais (do conjunto de cercas verticais mais as cercas de limite), de forma que:

1. A distância entre o par selecionado de linhas horizontais seja a mesma que a distância entre o par selecionado de linhas verticais.
2. Essas quatro linhas delimitam uma região quadrada.
3. Entre todos os quadrados possíveis que você pode formar dessa forma, você quer aquele com **área máxima**.

Retorne a área máxima do quadrado possível, módulo \(10^9 + 7\). Se você não puder formar nenhum quadrado (ou seja, nenhum par de distâncias horizontais e verticais corresponde), retorne **-1**.

**Exemplo 1:**

```
Input: m = 4, n = 3, hFences = [2, 3], vFences = [2]
Output: 4
```

**Exemplo 2:**

```
Input: m = 6, n = 7, hFences = [2], vFences = [4]
Output: -1
```

**Restrições:**

- \(3 \le m, n \le 10^9\)
- \(1 < hFences[i] < m\)
- \(1 < vFences[i] < n\)
- Os elementos em `hFences` e `vFences` são únicos.

## Link

https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/

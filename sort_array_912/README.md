# 912. Sort an Array

## Descrição

Dado um array de inteiros `nums`, ordene o array em ordem crescente e retorne-o.

**Exemplo 1:**

```
Input: nums = [5,2,3,1]
Output: [1,2,3,5]
```

Explicação: Após a ordenação, as posições de alguns números permanecem inalteradas (por exemplo, 2 e 3), enquanto outras mudam (por exemplo, 1 e 5).

**Exemplo 2:**

```
Input: nums = [5,1,1,2,0,0]
Output: [0,0,1,1,2,5]
```

Explicação: O array contém valores duplicados, que também são ordenados.

**Restrições:**

- `1 <= nums.length <= 50,000`
- `-50,000 <= nums[i] <= 50,000`

**Abordagens de Solução:**

Para atender ao requisito de complexidade de tempo \(O(n \log n)\), algoritmos como Merge Sort e Quick Sort são adequados. Merge Sort oferece desempenho consistente com complexidade de tempo \(O(n \log n)\) e complexidade de espaço \(O(n)\). Quick Sort, embora tenha uma complexidade de tempo média de \(O(n \log n)\), pode degradar para \(O(n^2)\) no pior caso, mas geralmente usa \(O(\log n)\) de espaço.

## Link

https://leetcode.com/problems/sort-an-array/

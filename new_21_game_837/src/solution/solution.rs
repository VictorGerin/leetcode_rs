pub struct Solution;

impl Solution {
    ///
    /// Explicação inicial metemática do problema:
    /// 
    /// Imagine que faremos um jogo com várias estapas, em um determinado momento qualquer
    /// teremos na mão um valor de "i" (valor acumulado) para termos chegado a esse valor
    /// de "i" temos necessáriamente na estapa anterior temos um valor na mão de "i-1" e
    /// termos tirado "1" nas cartas OU termos "i-2" e termos tirados "2" OU "i-3" e tirar
    /// 3 ... até maxPts pois seria a maior carta que poderiamos termos tirado nesse meio tempo.
    /// 
    /// Então usando a teoria da probabilidade TOTAL que diz que a probabilidade de um
    /// evento ocorrer é dado pela soma das probbiliddes parciais de cada estado que
    /// compõem o evento princial.
    /// 
    /// No nosso caso a probabilidade de chegamos no estado "i" "P(i)" é o somatório das
    /// probabilidades de está em "i-j" "P(i-j)" vezes a probabilidde de chegar em i dado
    /// que "i-j" ocorreu "P(i|i-j)" onde 1 <= j <= maxPts ou seja:
    /// 
    /// P(i) = SUM( P(i-j) * P(i|i-j) ), 1 <= j <= maxPts
    /// 
    /// Vamos Analisar a equação:
    ///     P(i) é o que queremos encontrar não temos muito o que falar sobre.
    ///     P(i | i-j) Chamado de probabilidade condicional, ler-se, probabilidade de i
    ///         ocorrer dado que i-j tenha ocorrido. Imagine uma situação hipotetica temos
    ///         acomulado "5" (i-j) para chemados no estado de "9" (i) temos que tirar do 
    ///         baralho somente o numero "4" (j) é nossa unica possibilidade de ocorrer i
    ///         caso estivermos em i-j, e como no problema da as probabilidade de tirar
    ///         qualquer carta do baralho é igual e com valor de "1/maxPts" então simples-
    ///         mente P(i | i-j) = 1/maxPts.
    ///     P(i-j) é similar P(i) é simplesmente a probabilidade de um estado anterior a i
    ///         e esse fato nos permite criar um algoritimo para calcular, pois se conhecemos
    ///         um P(x) qualquer podemos iniciar as contas a partir dele.
    /// 
    /// Condição Inicial:
    /// 
    /// É dito na questão que começa o jogo com 0, por tanto, qualquer jogo a probabilidade de
    /// 0 (zero) ocorrer é 100% ! por tanto, P(0) = 1, e consequentimente qualquer valor menor
    /// que 0 (zero) é impossível 0% pois o valor acomulado sempre aumenta nunca diminiu !
    /// 
    /// Sabendo P(0) e P(i | i-j) podemos executar a equação de P(i) para i = 1
    /// P(1) = P(0) * P(1|0) + P(-1)*P(1|-1) + ... + P(-maxPts) * P(1|-maxPts)
    /// Vamos notar que P(i | i-j) é uma constante e por tanto pode ficar fora do somatorio
    /// para a questão e por isso:
    /// P(1) = (1/maxPts) * ( P(0) + P(-1) + ... + P(-maxPts) )
    /// 
    /// Seguindo a mesma lógica podemos calcular P(2), P(3) até P(n) da questão
    /// 
    /// Porem existe uma observação importante que deve ser feita. Como o jogo só pode ocorrer
    /// até o alice ter k ou mais esse criterio de parada afeta o calculo de
    /// P(i) quando i > k 
    /// 
    /// P(k + 1) = P(k)*P(k+1 | k) + P(k - 1)*P(k+1 | k - 1) + ... + P(k - (maxPts-1))*P(k+1 | k - (maxPts-1))
    /// 
    /// Note que a primeira parte da equação diz que "P(k)*P(k+1 | k)" "para chegamos em k+1 podemos
    /// está em k e damos a sorte de tirar a carta "1"" essa afirmação é IMPOSSIVEL pois se chegamos
    /// em k o jogo ACABA por tanto não existe probabilidade de ir para k+1 dado que k ocorreu ela
    /// por definição é zero "P(k+1 | k) = 0" e não mais 1/maxPts como discutido anteriormente, por
    /// tanto para i > k a equação de P(i) deve levar em consideração essa diferença !
    /// 
    /// Veja que no final temos a probabilidade de P(n) ocorrer mas a questão pede P(<=n), probabilidade de n
    /// ou menos que n, assim, basta somar
    ///
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {

        let transsision_prob = 1f64 / max_pts as f64;

        let mut probability_windows = vec![0f64;max_pts as usize];

        for i in 1..=n {
            
            let p_i = (1..=max_pts).fold(0f64, |acc, j| {
                let prev = i - j;
                if prev >= k {
                    acc
                } else {
                    let prob_prev = match prev {
                        x if x < 0 => 0f64,
                        0 => 1f64,
                        x if x >= 1 => probability_windows[(x % max_pts) as usize],
                        _ => panic!("Valor impossível de x")
                    };
                    acc + prob_prev * transsision_prob
                }
            });

            probability_windows[(i % max_pts) as usize] = p_i;

        }
        
        // Calcular a probabilidade de ter n ou menos
        (k..=n).map(|i| {
            match i {
                x if x < 0 => 0f64,
                0 => 1f64,
                x if x >= 1 => probability_windows[(x % max_pts) as usize],
                _ => panic!("Valor impossível de x")
            }
        }).sum()
    }
}

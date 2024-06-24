// Cormengo e Flaminthians participaram de um campeonato de futebol juntamente com outros times.
//
// CADA VITÓRIA CONTA 3 PONTOS, CADA EMPATE 1 PONTO. FICA MELHOR CLASSIFICADO O TIME QUE POSSUI
// MAIS PONTOS. E EM CASO DE EMPATE DE PONTOS, FICA MELHOR CLASSIFICADO O TIME COM MAIS GOLS.
//
// Se o número de pontos e o saldo de gols forem os mesmos para os dois times então os dois times estão empatados no campeonato.
//
// Dados os números de vitórias e empates, e os saldos de gols dos dois times, sua tarefa é determinar qual dos dois está melhor classificado, 
// ou se eles estão empatados no campeonato.
//
// Entradas:são, respectivamente, o número de vitórias do Cormengo, o número de empates do Cormengo, o saldo de gols do Cormengo, 
// o número de vitórias do Flaminthians, o número de empates do Flaminthians e o saldo de gols do Flaminthians. 

use std::io;

fn quem_vai_ganhar() -> char {
    let mut values = String::new(); 
    io::stdin().read_line(&mut values).unwrap();
    
    let values_array: Vec<_> = values.split_whitespace().into_iter().collect();

    let mut cormengo: Vec<i32> = Vec::new();
    let mut flaminthians: Vec<i32> = Vec::new();
    for (index, item) in values_array.iter().enumerate() {
        if index >= 0 && index <= 2 {
            cormengo.push(item.parse::<i32>().unwrap_or_default());
        } else {
            flaminthians.push(item.parse::<i32>().unwrap_or_default());
        } 
    }

    let cormengo_points = cormengo[0] * 3 + cormengo[1];
    let flaminthians_points = flaminthians[0] * 3 + flaminthians[1];
    let cormengo_gols = cormengo[2];
    let flaminthians_gols = flaminthians[2];

    if cormengo_points > flaminthians_points {
        'C'
    } else if flaminthians_points > cormengo_points {
        'F'
    } else {
        if cormengo_gols > flaminthians_gols {
            'C'
        } else if flaminthians_gols > cormengo_gols {
            'F'
        } else {
            '='
        }
    } 
}

fn main() {
   println!("{}", quem_vai_ganhar()); 
}

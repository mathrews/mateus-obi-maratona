// e por isso quer deixar sua garrafa de água exatamente no ponto da pista 
// onde ele termina o seu treinamento.

// calcular o local do ponto de término do treinamento. 

// O ponto de término é o local da pista onde ele termina o percurso de C metros considerando que
// ele parte do ponto de partida e se movimenta sempre na 
// mesma direção.

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();

    let mut values: Vec<i32> = Vec::with_capacity(2);
    let line: Vec<&str> = input.split_whitespace().collect();
    for item in line {
        values.push(item.trim().parse::<i32>().unwrap_or_default()); 
    }

    println!("{}", values[0] % values[1]);
}

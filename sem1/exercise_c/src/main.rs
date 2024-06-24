// Diversos números são ditos em voz alta, quando o número 0 (zero) é dito então o desafio termina e seus colegas devem dizer imediatamente qual foi o maior número.
//
// Basicamente: imprima o maior número da colecao

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let lista: Vec<&str> = input.split_whitespace().collect();
    let mut lista_conv: Vec<i32> = Vec::new(); 
    for item in lista {
        let i = item.parse::<i32>().unwrap_or_default();
        lista_conv.push(i);
    }
    lista_conv.sort_unstable();

    println!("{:?}", lista_conv.pop().unwrap_or_default());
}

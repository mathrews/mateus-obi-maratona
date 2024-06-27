// Felizmente eles armazenaram a pontuação de todos competidores – que foram apenas três, devido ao tamanho diminuto do país.
// Sabe-se também que as pontuações de todos jogadores foram diferentes, de forma que não ocorreu empate entre nenhum deles.

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let colec_str: Vec<&str> = input.split_whitespace().collect();
    let mut colec_conv: Vec<i32> = Vec::new();
    for item in colec_str {
        colec_conv.push(item.trim().parse::<i32>().unwrap_or_default());
    }

    colec_conv.sort();
    println!("{}", colec_conv[1]);
}

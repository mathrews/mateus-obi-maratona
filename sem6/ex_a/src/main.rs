// Cada atividade tem um valor de pontos associado: esportes valem 2 pontos,
// artes valem 3 pontos e ciências valem 5 pontos.
//
// Se a soma dos pontos de todas as atividades for igual ou maior (>=) que 200,
// os alunos ganham uma medalha de ouro.
//
// Se a soma dos pontos for maior ou igual (>=) a 150, mas menor (<) que 200,
// os alunos ganham uma medalha de prata.
//
// Se a soma dos pontos for maior ou igual (>=) a 100, mas menor (<) que 150,
// os alunos ganham uma medalha de bronze.
//
// Se a soma dos pontos for menor (<) que 100, os alunos não ganham nenhum prêmio.

use std::io;

fn main() {
    let (mut e, mut a, mut c): (i32, i32, i32) = (0, 0, 0);
    for i in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_conv: i32 = input.trim().parse().unwrap_or_default();
        match i {
            0 => {
                e = input_conv * 2;
            },
            1 => {
                a = input_conv * 3;
            },
            2 => {
                c = input_conv * 5;
            },
            _ => {
                println!("praq mano?");
            }
        }
    }

    if (e + a + c) >= 200 {
        println!("O");
    } else if (e + a + c) >= 150 && (e + a + c) < 200 {
        println!("S");
    } else if (e + a + c) >= 100 && (e + a + c) < 150 {
        println!("B");
    } else {
        println!("N");
    }
}

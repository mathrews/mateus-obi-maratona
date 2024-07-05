//

use std::io;

fn main() {
    let (mut p, mut d1, mut d2): (i32, i32, i32) = (0, 0, 0);
    for i in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_conv: i32 = input.trim().parse().unwrap_or_default();
        match i {
            0 => {
                p = input_conv;
            },
            1 => {
                d1 = input_conv;
            },
            2 => {
                d2 = input_conv;
            },
            _ => {
                println!("praq mano?");
            }
        }
    }

    if ((d1 + d2) % 2 == 0) && p == 0 {
        println!("0");
    } else if ((d1 + d2) % 2 != 0) && p == 1 {
        println!("0");
    } else {
        println!("1");
    }
}

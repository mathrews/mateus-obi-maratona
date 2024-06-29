use std::io;

fn main() {
    let pf = Vec::from([(1, 3), (2, 3), (2, 5), (5, 4)]);
    let mut pi = (4, 3);

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let line_vec: Vec<&str> = line.split_whitespace().collect();

    let inpu1_conv = input1.trim().parse::<i32>().unwrap();
    let line_conv: Vec<i32> = line_vec.iter().map(|i| i.trim().parse::<i32>().unwrap_or_default()).collect();

    let mut movs = 0;

    'first:for i in line_conv {
        match i {
            1 => {
                pi.0 += 1;
                pi.1 += 2;
            },
            2 => {
                pi.0 += 2;
                pi.1 += 1;
            },
            3 => {
                pi.0 += 2;
                pi.1 -= 1;
            },
            4 => {
                pi.0 += 1;
                pi.1 -= 2;
            },
            5 => {
                pi.0 -= 1;
                pi.1 -= 2;
            }, 
            6 => {
                pi.0 -= 2;
                pi.1 -= 1;
            },
            7 => {
                pi.0 -= 2;
                pi.1 += 1;
            },
            8 => {
                pi.0 -= 1;
                pi.1 += 2;
            },
            _ => {
                println!("Seu burro");
            }
        }
        for j in &pf {
            if pi == *j {
                movs += 1;
                break 'first;
            }
        }
        movs += 1;
    }

    println!("{}", movs);
}

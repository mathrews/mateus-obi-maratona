use std::io;

fn main() {
    let mut line1 = String::new();
    io::stdin().read_line(&mut line1).unwrap();
    let line1 = line1.trim().parse::<i32>().unwrap();
    
    let mut values: Vec<i32> = Vec::new();
    for _ in 0..line1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        values.push(input.trim().parse::<i32>().unwrap_or_default());
    }

    values.sort();
    
    let mut students = 0;
    for (index, item) in values.iter().enumerate() {
        if index == 0 {
            students += 1;
            continue;
        } else {
            if values[index] != values[index - 1] {
                students += 1;
            } else {
                continue;
            }
        }
    }

    println!("{}", students);
}

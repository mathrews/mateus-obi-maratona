use std::{collections::HashSet, io};

fn main() {
    let mut line1 = String::new();
    io::stdin().read_line(&mut line1).unwrap();
    let line1 = line1.trim().parse::<i32>().unwrap();
    
    let mut values: HashSet<i32> = HashSet::new();
    for _ in 0..line1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        values.insert(input.trim().parse::<i32>().unwrap_or_default());
    }

    println!("{}", values.len());
}

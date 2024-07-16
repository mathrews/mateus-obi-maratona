use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut test_cases: Vec<Vec<&str>> = Vec::new();
    let mut i = 0;
    while i < input.parse::<i32>().unwrap_or_default() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        test_cases.push(input.split_whitespace().collect());
        i += 1;
    }
}

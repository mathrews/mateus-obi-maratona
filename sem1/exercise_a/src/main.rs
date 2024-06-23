// t1 do terceiro == t*1 < t2 do segundo == t1*2 < t3 do primeiro == t2*2

use std::io;

fn clicked_links() -> i32 {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    
    let conv_t: i32 = t.trim().parse().unwrap_or_default();
    
    if conv_t >= 1 && conv_t <= 1000 {
        return (conv_t * 2) * 2;
    } else {
        return -1;
    }
}

fn main() {
    println!("{}", clicked_links().to_string());
}

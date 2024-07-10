fn reverse_string(s: &mut Vec<char>) {
    let mut temp = 'a';
    let mut start = 0;
    let mut end = s.len() - 1;

    while start < end {
        temp = s[start];
        s[start] = s[end];
        s[end] = temp;
        start += 1;
        end -= 1;
    }

    println!("{:?}", s);
}

fn main() {
    let mut s = Vec::from(['H', 'e', 'l', 'l', 'o']);
    reverse_string(&mut s);
}

// O número de pontos do participante é o tamanho da maior sequência de
// valores consecutivos iguais.

use std::io;

fn count_points(vec: &mut Vec<&str>) -> i32 {
    let mut points: i32 = 1;
    let mut collection_points: Vec<i32> = Vec::new();
    for (index, item) in vec.iter().enumerate() {
        if index == 0 {
            continue;
        } else {
            if vec[index] == vec[index - 1] {
                points += 1;
            } else {
                collection_points.push(points);
                points = 1;
                continue;
            }
        }
    }
    collection_points.push(points);
    collection_points.sort();
    collection_points[collection_points.len() - 1]
}

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();

    let mut repeated_values: Vec<&str> = input2.split_whitespace().collect();

    println!("{}", count_points(&mut repeated_values));
}

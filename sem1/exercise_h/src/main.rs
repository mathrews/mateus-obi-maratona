// As portas de sua casa têm altura H e largura L
// e existe um colchão que está em promoção com dimensões A × B × C.
// paralelepípedo reto retângulo
//
// A primeira linha da entrada contém três números inteiros A, B e C (1 ≤ A, B, C ≤ 300),
// as três dimensões do colchão, em centímetros
//
//  A segunda linha contém dois inteiros H e L (1 ≤ H, L ≤ 250),
//  respectivamente a altura e a largura das portas em centímetros.
//
//  O colchão tem a forma de um paralelepípedo reto retângulo e João só consegue arrastá-lo através de uma porta com uma de suas faces
//  paralelas ao chão, mas consegue virar e rotacionar o colchão antes de passar pela porta.

use std::io;

fn calcular() -> String {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();

    let colchao_dims: Vec<&str> = input1.split_whitespace().collect();
    let porta_dims: Vec<&str> = input2.split_whitespace().collect();

    let mut c_dims_conv: Vec<i32> = colchao_dims
        .iter()
        .map(|item| item.parse::<i32>().unwrap_or_default())
        .collect();
    c_dims_conv.sort_unstable();
    let p_dims_conv: Vec<i32> = porta_dims
        .iter()
        .map(|item| item.parse::<i32>().unwrap_or_default())
        .collect();

    let (heigth, width) = (p_dims_conv[0], p_dims_conv[1]);  

    if (c_dims_conv[0] <= heigth && c_dims_conv[1] <= width) || (c_dims_conv[0] <= width && c_dims_conv[1] <= heigth) {
        return "S".to_string();
    }

    "N".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = calcular();
        assert_eq!(result, "S".to_string());
    }
}

fn main() {
    println!("{}", calcular());
}

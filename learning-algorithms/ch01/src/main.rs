fn pesquisa_binaria(vetor: &Vec<usize>, acerto: usize) -> Option<usize> {
    let mut baixo: usize = 0;
    let mut alto: usize = vetor.len() - 1;

    while baixo <= alto {
        let meio: usize = (baixo + alto) / 2 as usize;
        let chute: usize = vetor[meio];
        if chute == acerto {
            println!("Acertou! O número era {acerto}");
            return Some(meio);
        } else if chute > acerto {
            println!("Muito alto");
            alto = meio - 1
        } else {
            println!("Muito baixo");
            baixo = meio + 1;
        } 
    }
    None
}

fn main() {
    let minha_lista: Vec<usize> = Vec::from([1, 3, 5, 7, 9]);
    match pesquisa_binaria(&minha_lista, 3) {
        Some(x) => println!("Result {x}"),
        None => println!("Not in the Vec"),
    }
    println!();
    match pesquisa_binaria(&minha_lista, 10) {
        Some(x) => println!("Result {x}"),
        None => println!("Not in vec"),
    }
}

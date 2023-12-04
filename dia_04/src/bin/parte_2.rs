fn get_numeros(numeros: &str) -> Vec<&str> {
    return numeros.split(" ").map(|n| n).filter(|n| *n != "").collect();
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input = include_str!("./input.txt");
    let linhas: std::str::Split<'_, &str> = input.split("\n");

    let qtd_linhas: usize = linhas.clone().count();

    let mut scratchcards: Vec<i32> = vec![1; qtd_linhas];

    for (index, linha,) in linhas.enumerate() {
        let index_i: i32 = index as i32;
        println!("linha {}", linha);
        let mut numeros: &str = &linha.split(":").nth(1).unwrap().replace("  ", " ");
        numeros = numeros.trim();

        let numeros_ganhadores: Vec<&str> = get_numeros(numeros.split("|").nth(0).unwrap());
        let meus_numeros: Vec<&str> = get_numeros(numeros.split("|").nth(1).unwrap());

        let mut qtd_numeros: i32 = 0;
        for numero in meus_numeros {
            if numeros_ganhadores.contains(&numero) {
                qtd_numeros += 1;
            }
        }

        println!("qtd {}", qtd_numeros);

        let mut i: i32 = index_i + 1;
        while i <= (index_i + qtd_numeros) && i < (qtd_linhas as i32){
            println!("{}", i);
            scratchcards[i as usize] += scratchcards[index_i as usize];
            i += 1;
        }
    }

    let mut soma: i32 = 0;
    for qtd in scratchcards {
        println!("{}", qtd);
        soma += qtd;
    }
    
    println!("{}", soma);
}

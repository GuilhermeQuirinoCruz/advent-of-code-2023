fn get_numeros(numeros : &str) -> Vec<&str> {
    return numeros.split(" ").map(|n| n).filter(|n| *n != "").collect();
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input = include_str!("./input.txt");
    let linhas: std::str::Split<'_, &str> = input.split("\n");

    let mut soma: i32 = 0;

    for linha in linhas {
        let mut numeros: &str = &linha.split(":").nth(1).unwrap().replace("  ", " ");
        numeros = numeros.trim();

        let numeros_ganhadores : Vec<&str> = get_numeros(numeros.split("|").nth(0).unwrap());
        let meus_numeros : Vec<&str> = get_numeros(numeros.split("|").nth(1).unwrap());

        let mut qtd_numeros: i32 = 0;
        for numero in meus_numeros {
            if numeros_ganhadores.contains(&numero) {
                // println!("[{}]", numero);
                qtd_numeros += 1;
            }
        }

        if qtd_numeros > 0 {
            let dois: i32 = 2;
            soma += dois.pow((qtd_numeros - 1) as u32);
        }
    }

    println!("{}", soma);
}

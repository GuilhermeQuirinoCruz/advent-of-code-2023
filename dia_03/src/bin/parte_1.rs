struct Numero {
    inicio: i32,
    fim: i32,
    linha: i32,
    valor: i32,
}

fn numeros_input(input: Vec<Vec<char>>) -> Vec<Numero> {
    let mut numeros: Vec<Numero> = Vec::new();

    let mut inicio: i32 = -1;
    let mut n_linha: i32 = 0;
    let mut n_coluna: i32;
    let mut str_numero: String = "".to_string();

    for linha in input {
        n_coluna = 0;
        for letra in linha {
            if letra.is_numeric() {
                str_numero.push(letra);

                if inicio == -1 {
                    inicio = n_coluna;
                }
            } else {
                if inicio != -1 {
                    numeros.push(Numero {
                        inicio: inicio,
                        fim: n_coluna - 1,
                        linha: n_linha,
                        valor: str_numero.parse::<i32>().unwrap(),
                    });

                    inicio = -1;
                    str_numero.clear();
                }
            }

            n_coluna += 1;
        }

        if inicio != -1 {
            numeros.push(Numero {
                inicio: inicio,
                fim: n_coluna - 1,
                linha: n_linha,
                valor: str_numero.parse::<i32>().unwrap(),
            });

            inicio = -1;
            str_numero.clear();
        }

        n_linha += 1;
    }

    return numeros;
}

fn numero_valido(numero : &Numero, input: Vec<Vec<char>>) -> bool{
    for y in (numero.linha - 1)..(numero.linha + 2) {
        if y < 0 || y >= (input.len() as i32) {
            continue;
        }

        for x in (numero.inicio - 1)..(numero.fim + 2) {
            if x < 0 || x >= ((input.get(y as usize).unwrap().len()) as i32) {
                continue;
            }

            let letra : char = input.get(y as usize).unwrap().get(x as usize).unwrap().clone();
            if !letra.is_numeric() && letra != '.' {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    // let input = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let mut matriz_input: Vec<Vec<char>> = Vec::new();

    for linha in input.split("\n") {
        matriz_input.push(linha.chars().collect());
    }

    let numeros: Vec<Numero> = numeros_input(matriz_input.clone());
    let mut soma: i32 = 0;
    for numero in numeros {
        // println!("{}={}", numero.valor.clone(), numero_valido(numero, matriz_input.clone()));
        if numero_valido(&numero, matriz_input.clone()) {
            soma += numero.valor;
        }
    }

    println!("{}", soma);
}

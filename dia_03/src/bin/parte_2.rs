use std::collections::HashMap;

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

fn gears_input(input: Vec<Vec<char>>) -> HashMap<(i32, i32), Vec<i32>> {
    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    let mut x: i32;
    let mut y: i32 = 0;

    for linha in input {
        x = 0;
        for letra in linha {
            if letra == '*' {
                gears.insert(
                    (x, y),
                    Vec::new(),
                );
            }

            x += 1;
        }

        y += 1;
    }

    return gears;
}

fn adicionar_numero_gears(numero: &Numero, gears: &mut HashMap<(i32, i32), Vec<i32>>) {
    for y in (numero.linha - 1)..(numero.linha + 2) {
        if y < 0 {
            continue;
        }

        for x in (numero.inicio - 1)..(numero.fim + 2) {
            if x < 0 {
                continue;
            }

            let coords: (i32, i32) = (x, y);
            if gears.contains_key(&coords) {
                let mut numeros: Vec<i32> = Vec::new();
                for numero in gears.get(&coords).unwrap() {
                    numeros.push(*numero);
                }

                numeros.push(numero.valor);

                gears.insert(coords, numeros);
            }
        }
    }
}

fn main() {
    // let input = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let mut matriz_input: Vec<Vec<char>> = Vec::new();

    for linha in input.split("\n") {
        matriz_input.push(linha.chars().collect());
    }

    let numeros: Vec<Numero> = numeros_input(matriz_input.clone());
    let mut gears: HashMap<(i32, i32), Vec<i32>> = gears_input(matriz_input.clone());

    for numero in numeros {
        adicionar_numero_gears(&numero, &mut gears);
    }

    let mut soma: i32 = 0;

    for gear in gears.values() {
        if gear.len() == 2 {
            soma += gear.iter().product::<i32>();
        }
    }

    println!("{}", soma);
}

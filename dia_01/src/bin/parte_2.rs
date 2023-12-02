fn get_numeric_char(linha: String, reverter: bool) -> char {
    let char_it = linha.chars();

    if reverter {
        for char in char_it.rev() {
            if char.is_numeric() {
                return char;
            }
        }
    } else {
        for char in char_it {
            if char.is_numeric() {
                return char;
            }
        }
    }

    return '\0';
}

fn converter_linha_numero(linha: String) -> i32 {
    let mut numero = String::new();

    numero.push(get_numeric_char(linha.clone(), false));
    numero.push(get_numeric_char(linha.clone(), true));

    let numero_convertido: i32 = match numero.parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return numero_convertido;
}

fn substituir_numeros_extenso(linha : String) -> String {
    let numeros : Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string()];
    
    let mut nova_linha : String = String::new();

    let mut sequencia : String = "".to_string();
    for char in linha.chars() {
        if char.is_numeric() {
            nova_linha.push(char);
            sequencia.clear();
            continue;
        }

        sequencia.push(char);

        let mut possivel_numero : bool = false;
        let mut indice: i32 = 1;
        for numero in numeros.clone() {
            if numero.starts_with(&sequencia) {
                possivel_numero = true;

                if sequencia.len() == numero.len() {
                    let posicao: i32 = indice;
                    nova_linha.extend(posicao.to_string().chars());
                    sequencia.clear();
                    break;
                }
            }

            indice += 1;
        }

        if !possivel_numero {
            sequencia = sequencia.chars().skip(1).collect();
        }
    }

    return nova_linha;
}

fn main() {
    // let input = include_str!("./exemplo_2.txt");
    let input = include_str!("./input.txt");
    let linhas = input.split("\n");

    let mut soma : i32 = 0;
    for linha in linhas {
        // println!("{} - {}", soma, substituir_numeros_extenso(linha.to_string()));
        // println!("{}", converter_linha_numero(substituir_numeros_extenso(linha.to_string())));
        soma += converter_linha_numero(substituir_numeros_extenso(linha.to_string()));
    }

    println!("{}", soma);
    // 52984 - muito alto
}
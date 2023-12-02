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

fn main() {
    // let input = include_str!("./exemplo_1.txt");
    let input = include_str!("./input.txt");
    let linhas = input.split("\n");

    let mut soma: i32 = 0;

    for linha in linhas {
        // println!("{}", converter_linha_numero(linha.to_string()));
        soma += converter_linha_numero(linha.to_string());
    }

    println!("{}", soma);
}

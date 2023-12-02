use std::collections::HashMap;

fn soma_id_games_possiveis(games : Vec<Vec<HashMap<String, i32>>>) -> i32 {
    let mut soma_ids: i32 = 0;
    let config : HashMap<String, i32> = HashMap::from([
        ("red".to_string(), 12),
        ("blue".to_string(), 14),
        ("green".to_string(), 13),
    ]);

    let mut id_game: i32 = 1;
    for game in games {
        let mut possivel : bool = true;
        for round in game {
            for key in round.keys() {
                let qtd_config: &i32 = config.get(key).unwrap();
                let qtd_round: &i32 = round.get(key).unwrap();
                if qtd_round > qtd_config {
                    possivel = false;
                    break;
                }
            }

            if !possivel {
                break;
            }
        }

        if possivel {
            soma_ids += id_game;
        }

        id_game += 1;
    }

    return soma_ids;
}

fn main() {
    // let input = include_str!("./exemplo_1.txt");
    let input = include_str!("./input.txt");

    let mut games: Vec<Vec<HashMap<String, i32>>> = Vec::new();

    for linha in input.split("\n") {
        let dados = linha.split(": ").nth(1).unwrap();
        let mut game: Vec<HashMap<String, i32>> = Vec::new();

        for cubes_set in dados.split("; ") {
            let mut round: HashMap<String, i32> = HashMap::new();
            for par in cubes_set.split(", ") {
                let cor = par.split(" ").nth(1).unwrap();
                let qtd = par.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                // println!("Cor: {}, qtd: {}", cor, qtd);
                round.insert(cor.to_string(), qtd);
            }

            game.push(round);
        }

        games.push(game);
    }

    println!("{}", soma_id_games_possiveis(games));
}

use std::collections::HashMap;

fn soma_poder_sets_game(games: Vec<Vec<HashMap<String, i32>>>) -> i32 {
    let mut soma_poderes: i32 = 0;
    
    for game in games {
        let mut minimos: HashMap<String, i32> = HashMap::from([
            ("red".to_string(), 0),
            ("blue".to_string(), 0),
            ("green".to_string(), 0),
        ]);

        for round in game {
            for key in round.keys() {
                let qtd_minimos: &i32 = minimos.get(key).unwrap();
                let qtd_round: &i32 = round.get(key).unwrap();

                minimos.insert(key.to_string(), Ord::max(*qtd_minimos, *qtd_round));
            }
        }

        let mut poder_set: i32 = 1;
        for minimo in minimos.values() {
            poder_set *= *minimo;
        }

        soma_poderes += poder_set;
    }

    return soma_poderes;
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

    println!("{}", soma_poder_sets_game(games));
}

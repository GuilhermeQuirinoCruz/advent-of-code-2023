use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    // hand: String,
    bid: i32,
    score: u64,
}

fn get_hand_type(hand: &str) -> char {
    let card_count: HashMap<char, i32> = hand
        .chars()
        .into_group_map_by(|&x| x)
        .into_iter()
        .map(|(k, v)| (k, v.len() as i32))
        .collect();

    let jokers: usize = if card_count.contains_key(&'J') { 1 } else { 0 };

    return match card_count.keys().len() - jokers {
        0 | 1 => 7,
        2 => {
            if card_count.values().contains(&4) {
                6 + jokers
            } else if card_count.values().contains(&3) {
                5 + jokers
            } else {
                if jokers > 0 && *card_count.get(&'J').unwrap() == 2 {
                    6
                } else {
                    4 + jokers
                }
            }
        }
        3 => {
            if card_count.values().contains(&3) {
                4 + jokers
            } else {
                3 + jokers
            }
        }
        4 => 2,
        5 | _ => 1,
    }
    .to_string()
    .chars()
    .next()
    .unwrap();
}

fn get_hand_score(hand: &str) -> u64 {
    let labels: String = "J23456789TQKA".to_string();
    let mut score: String = "".to_string();

    score.push(get_hand_type(hand));

    for card in hand.chars() {
        score.extend(format!("{:0>2}", (labels.find(card).unwrap() + 1).to_string()).chars())
    }

    score = format!("{:0<11}", score);

    return score.parse::<u64>().unwrap();
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./teste.txt");
    let input: &str = include_str!("./input.txt");

    let mut games: Vec<Game> = input
        .split("\n")
        .into_iter()
        .map(|game| {
            let hand: &str;
            let bid: &str;
            (hand, bid) = game.split_once(" ").unwrap();
            Game {
                // hand: hand.to_string(),
                bid: bid.parse::<i32>().unwrap(),
                score: get_hand_score(hand),
            }
        })
        .collect();

    games.sort_by(|g1, g2| g1.score.cmp(&g2.score));

    let mut i: i32 = 0;
    let sum: i32 = games
        .iter()
        .map(|game| {
            // println!("hand {}, score {}", game.hand, game.score);
            i += 1;
            game.bid * i
        })
        .sum();

    println!("{}", sum);
}

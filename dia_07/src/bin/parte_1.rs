use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    // hand: String,
    bid: i32,
    score: u64,
}

fn get_hand_type(hand: &str) -> char {
    let card_count: HashMap<char, u32> = hand
        .chars()
        .into_group_map_by(|&x| x)
        .into_iter()
        .map(|(k, v)| (k, v.len() as u32))
        .collect();

    return match card_count.keys().len() {
        1 => 7,
        2 => {
            if card_count.values().contains(&4) {
                6
            } else {
                5
            }
        }
        3 => {
            if card_count.values().contains(&3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        5 => 1,
        _ => 0,
    }
    .to_string()
    .chars()
    .next()
    .unwrap();
}

fn get_hand_score(hand: &str) -> u64 {
    let labels: String = "23456789TJQKA".to_string();
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
            i += 1;
            game.bid * i
        })
        .sum();

    println!("{}", sum);
}

// TJQKA

// AKQJT
// KAQJT

// 131211109
// 121311109

// 1131211109
// 1121311109
// 21313121110
// 21313123000

// AAKQJ
// AA234

// AAAAA
// 71313131313

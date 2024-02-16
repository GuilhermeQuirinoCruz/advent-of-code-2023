use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    direction: (i32, i32),
    move_amount: u8,
}

fn get_direction_from_char(c: char) -> (i32, i32) {
    match c {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => (0, 0),
    }
}

fn get_lagoon_positions(instructions: &Vec<Instruction>) -> HashSet<(i32, i32)> {
    let mut lagoon: HashSet<(i32, i32)> = HashSet::new();

    let mut current_position: (i32, i32) = (1, 1);
    lagoon.insert(current_position);
    for instruction in instructions {
        for _ in 0..instruction.move_amount {
            current_position = (
                current_position.0 + instruction.direction.0,
                current_position.1 + instruction.direction.1,
            );

            lagoon.insert(current_position);
        }
    }

    return lagoon;
}

fn is_position_valid(position: (i32, i32), limits: ((i32, i32), (i32, i32))) -> bool {
    let valid_x: bool = position.0 >= limits.0.0 && position.0 <= limits.0.1;
    let valid_y: bool = position.1 >= limits.1.0 && position.1 <= limits.1.1;

    return valid_x && valid_y;
}

fn flood_fill_count(lagoon: &HashSet<(i32, i32)>) -> i32 {
    let iter_x = lagoon.iter().map(|a| a.0);
    let iter_y = lagoon.iter().map(|a| a.1);

    let limits_x: (i32, i32) = (iter_x.clone().min().unwrap() - 1, iter_x.max().unwrap() + 1);
    let limits_y: (i32, i32) = (iter_y.clone().min().unwrap() - 1, iter_y.max().unwrap() + 1);

    let mut flood_fill_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position: (i32, i32) = (limits_x.0, limits_y.0);
    flood_fill_positions.insert(current_position);

    let mut next_positions: Vec<(i32, i32)> = Vec::new();
    let directions: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    loop {
        for direction in &directions {
            let position: (i32, i32) = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );

            if is_position_valid(position, (limits_x, limits_y))
                && !lagoon.contains(&position)
                && !flood_fill_positions.contains(&position)
            {
                flood_fill_positions.insert(position);
                next_positions.push(position);
            }
        }

        if next_positions.len() == 0 {
            break;
        }

        current_position = next_positions.pop().unwrap();
    }

    let total_area: i32 = ((limits_x.1 - limits_x.0) + 1) * ((limits_y.1 - limits_y.0) + 1);

    return total_area - flood_fill_positions.len() as i32;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| l.split(" ("))
        .map(|mut t| t.next().unwrap().split_once(" "))
        .map(|a| Instruction {
            direction: get_direction_from_char(a.unwrap().0.chars().next().unwrap()),
            move_amount: a.unwrap().1.parse::<u8>().unwrap(),
        })
        .collect();

    println!("{}", flood_fill_count(&get_lagoon_positions(&instructions)));
}
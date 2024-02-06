use std::collections::HashSet;
use substring::Substring;

#[derive(Debug)]
struct Instruction {
    direction: (i32, i32),
    move_amount: u32,
}

fn get_direction_from_char(c: char) -> (i32, i32) {
    match c {
        '3' => (0, -1),
        '1' => (0, 1),
        '2' => (-1, 0),
        '0' => (1, 0),
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
    let valid_x: bool = position.0 >= limits.0 .0 && position.0 <= limits.0 .1;
    let valid_y: bool = position.1 >= limits.1 .0 && position.1 <= limits.1 .1;

    return valid_x && valid_y;
}

fn flood_fill_count(lagoon: &HashSet<(i32, i32)>) -> i32 {
    let iter_x = lagoon.iter().map(|a| a.0);
    let iter_y = lagoon.iter().map(|a| a.1);

    let limits_x: (i32, i32) = (iter_x.clone().min().unwrap() - 1, iter_x.max().unwrap() + 1);
    let limits_y: (i32, i32) = (iter_y.clone().min().unwrap() - 1, iter_y.max().unwrap() + 1);

    // println!("limits {:?} {:?}", limits_x, limits_y);

    let mut flood_fill_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position: (i32, i32) = (limits_x.0, limits_y.0);
    flood_fill_positions.insert(current_position);

    let mut next_positions: Vec<(i32, i32)> = Vec::new();
    let directions: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    loop {
        // println!("current {:?}", current_position);
        for direction in &directions {
            let position: (i32, i32) = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );

            // println!("checking {:?}", position);

            if is_position_valid(position, (limits_x, limits_y))
                && !lagoon.contains(&position)
                && !flood_fill_positions.contains(&position)
            {
                flood_fill_positions.insert(position);
                next_positions.push(position);

                // println!("valid");
            } else {
                // println!("invalid");
            }
        }

        if next_positions.len() == 0 {
            break;
        } else {
            // println!("{:?}", next_positions);
            // println!("");
        }

        current_position = next_positions.pop().unwrap();
    }

    let total_area: i32 = ((limits_x.1 - limits_x.0) + 1) * ((limits_y.1 - limits_y.0) + 1);

    // println!("total area {}", total_area);
    // println!("flood fill {:?}", flood_fill_positions);
    // println!("len {:?}", flood_fill_positions.len());

    return total_area - flood_fill_positions.len() as i32;
}

fn main() {
    let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./input.txt");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.split_once(" (#").unwrap().1)
        .map(|rs| rs.replace(")", ""))
        .map(|code| Instruction {
            direction: get_direction_from_char(code.substring(5, 6).chars().next().unwrap()),
            move_amount: u32::from_str_radix(code.substring(0, 5), 16).unwrap(),
        })
        .collect();

    println!("{}", flood_fill_count(&get_lagoon_positions(&instructions)));
}
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Pipe {
    _tile: char,
    connections: Vec<Position>,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn get_pipe_connections(tile: char, position: Position) -> Vec<Position> {
    let (add_x, add_y) = match tile {
        '|' => ((0, 0), (1, -1)),
        '-' => ((1, -1), (0, 0)),
        'L' => ((0, 1), (-1, 0)),
        'J' => ((0, -1), (-1, 0)),
        '7' => ((-1, 0), (0, 1)),
        'F' => ((0, 1), (1, 0)),
        '.' | 'S' | _ => ((0, 0), (0, 0)),
    };

    return Vec::from([
        Position {
            x: position.x + add_x.0,
            y: position.y + add_y.0,
        },
        Position {
            x: position.x + add_x.1,
            y: position.y + add_y.1,
        },
    ]);
}

fn print_pipes(pipes: &HashSet<Position>, limit_x: i32, limit_y: i32) {
    for y in 0..limit_y {
        for x in 0..limit_x {
            if pipes.contains(&Position { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn flood_fill_count(doubled_pipes: HashSet<Position>) -> usize {
    let limit_x: i32 = doubled_pipes.iter().map(|pipe| pipe.x).max().unwrap() + 1;
    let limit_y: i32 = doubled_pipes.iter().map(|pipe| pipe.y).max().unwrap() + 1;

    let mut fill_positions: HashSet<Position> = HashSet::new();
    let mut next_positions: Vec<Position> = vec![Position { x: 1, y: 1 }];
    while next_positions.len() > 0 {
        let current_position: Position = next_positions.pop().unwrap();
        fill_positions.insert(current_position);

        for (add_x, add_y) in [-1, 1, 0, 0].iter().zip([0, 0, -1, 1].iter()) {
            let x: i32 = current_position.x + add_x;
            let y: i32 = current_position.y + add_y;

            if x < 1 || x > limit_x || y < 1 || y > limit_y {
                continue;
            }

            let position: Position = Position { x, y };
            if !doubled_pipes.contains(&position) && !fill_positions.contains(&position) {
                next_positions.push(position);
            }
        }
    }

    let mut enclosed_positions: HashSet<Position> = HashSet::new();
    for x in 1..(limit_x + 1) {
        for y in 1..(limit_y + 1) {
            if x % 2 != 0 || y % 2 != 0 {
                continue;
            }

            let position: Position = Position { x, y };
            // println!("{:?}", position);
            if doubled_pipes.contains(&position) || fill_positions.contains(&position) {
                continue;
            }

            enclosed_positions.insert(position);
        }
    }

    // print_pipes(&doubled_pipes, limit_x, limit_y);

    return enclosed_positions.len();
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./exemplo_2.txt");
    // let input: &str = include_str!("./exemplo_3.txt");
    // let input: &str = include_str!("./exemplo_4.txt");
    let input: &str = include_str!("./input.txt");

    let mut pipes: HashMap<Position, Pipe> = HashMap::new();
    let mut starting_position: Position = Position { x: 0, y: 0 };

    for (line, y) in input.lines().zip(1..input.lines().count() + 1) {
        for (tile, x) in line.chars().zip(1..line.len() + 1) {
            if tile == '.' {
                continue;
            }

            let position: Position = Position {
                x: x as i32,
                y: y as i32,
            };

            pipes.insert(
                position,
                Pipe {
                    connections: get_pipe_connections(tile, position),
                    _tile: tile,
                },
            );

            if tile == 'S' {
                starting_position = position;
            }
        }
    }

    let mut s_connections: Vec<Position> = Vec::new();

    for (x, y) in [-1, 1, 0, 0].iter().zip([0, 0, -1, 1].iter()) {
        let position: Position = Position {
            x: starting_position.x + x,
            y: starting_position.y + y,
        };

        if pipes.contains_key(&position) {
            if pipes
                .get(&position)
                .unwrap()
                .connections
                .contains(&starting_position)
            {
                s_connections.push(position);
            }
        }
    }

    pipes.insert(
        starting_position,
        Pipe {
            _tile: 'S',
            connections: s_connections,
        },
    );

    let mut current_position: Position = starting_position;
    let mut previous_position: Position = starting_position;
    let mut main_loop: Vec<Position> = Vec::new();
    main_loop.push(starting_position);

    let mut doubled_pipes: HashSet<Position> = HashSet::new();

    while main_loop.len() < 2 || current_position != starting_position {
        for position in pipes.get(&current_position).unwrap().connections.clone() {
            if position != previous_position {
                previous_position = current_position;
                current_position = position;
                break;
            }
        }

        main_loop.push(current_position);

        let doubled_position: Position = Position {
            x: current_position.x * 2,
            y: current_position.y * 2,
        };

        for position in pipes.get(&current_position).unwrap().connections.clone() {
            doubled_pipes.insert(Position {
                x: doubled_position.x + (position.x - current_position.x),
                y: doubled_position.y + (position.y - current_position.y),
            });

            doubled_pipes.insert(doubled_position);
        }
    }

    pipes.retain(|pipe, _| main_loop.contains(pipe));

    println!("{:?}", flood_fill_count(doubled_pipes));
}

use std::collections::HashMap;

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

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./exemplo_2.txt");
    let input: &str = include_str!("./input.txt");

    let mut pipes: HashMap<Position, Pipe> = HashMap::new();
    let mut starting_position: Position = Position { x: 0, y: 0 };

    // pipes = HashMap::from_iter(input
    //     .lines()
    //     .zip(0..input.lines().count())
    //     .map(|(line, y)| line.chars().zip(0..line.len()))
    //     .map(|(c, x)| y));

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

    while main_loop.len() < 2 || current_position != starting_position {
        for position in pipes.get(&current_position).unwrap().connections.clone() {
            if position != previous_position {
                previous_position = current_position;
                current_position = position;
                break;
            }
        }

        main_loop.push(current_position);
    }
    
    let mut junk_pipes: Vec<Position> = Vec::new();
    for position in pipes.keys() {
        if !main_loop.contains(position) {
            junk_pipes.push(*position);
        }
    }

    for position in junk_pipes {
        pipes.remove(&position);
    }

    println!("{}", pipes.len());
}
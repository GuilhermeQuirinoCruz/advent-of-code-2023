use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Path {
    // visited: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
    last_position: (i32, i32),
    current_direction: (i32, i32),
    steps_in_direction: u8,
    cost: u32,
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let size_y: usize = input.lines().count();
    let size_x: usize = input.lines().next().unwrap().len();

    let mut heat_loss: HashMap<(i32, i32), u32> = HashMap::new();
    let mut min_heat_loss: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();
    for (line, y) in input.lines().zip(0..size_y) {
        for (c, x) in line.chars().zip(0..size_x) {
            heat_loss.insert((x as i32, y as i32), c.to_string().parse::<u32>().unwrap());

            for direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                min_heat_loss.insert(((x as i32, y as i32), (direction.0, direction.1)), u32::MAX);
            }
        }
    }

    // let mut min_heat_loss: u32 = u32::MAX;

    let mut next_paths: Vec<Path> = vec![Path {
        // visited: Vec::from([(0, 0)]),
        visited: HashSet::from([(0, 0)]),
        last_position: (0, 0),
        current_direction: (0, 0),
        steps_in_direction: 0,
        cost: 0,
    }];

    while next_paths.len() > 0 {
        let path: Path = next_paths.pop().unwrap();

        // if path.cost > min_heat_loss {
        //     continue;
        // }

        let current_position: (i32, i32) = path.last_position;

        // let current_min_heat_loss: u32 = *min_heat_loss.get(&current_position).unwrap();
        // if path.cost < current_min_heat_loss {
        //     min_heat_loss.insert(current_position, path.cost);
        // } else {
        //     continue;
        // }

        if current_position == (size_x as i32 - 1, size_y as i32 - 1) {
            // println!("found valid path {:?}", path);
            // min_heat_loss = min_heat_loss.min(path.cost);

            continue;
        }

        for direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if path.current_direction == direction && path.steps_in_direction == 3 {
                continue;
            }

            let position: (i32, i32) = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );

            if path.visited.contains(&position) {
                continue;
            }

            match heat_loss.get(&position) {
                Some(heat) => {
                    if (path.cost + *heat)
                        > *min_heat_loss
                            .get(&((position.0, position.1), (direction.0, direction.1)))
                            .unwrap()
                    {
                        continue;
                    } else {
                        min_heat_loss.insert(
                            ((position.0, position.1), (direction.0, direction.1)),
                            path.cost + *heat,
                        );
                    }

                    let steps: u8 = if path.current_direction == direction {
                        path.steps_in_direction + 1
                    } else {
                        1
                    };

                    let mut new_path: Path = Path {
                        visited: path.visited.clone(),
                        last_position: position,
                        current_direction: direction,
                        steps_in_direction: steps,
                        cost: path.cost + *heat,
                    };

                    // new_path.visited.push(position);
                    new_path.visited.insert(position);

                    next_paths.push(new_path);
                }
                None => (),
            }
        }
    }

    // println!("{:?}", min_heat_loss);
    // for (k, v) in min_heat_loss {
    //     println!("Position: {:?}, Direction: {:?}, Min Heat Loss: {:?}", (k.0.0, k.0.1), (k.1.0, k.1.1), v);
    // }

    for direction in [(1, 0), (0, 1)] {
        println!(
            "{:?}",
            min_heat_loss
                .get(&(
                    (size_x as i32 - 1, size_y as i32 - 1),
                    (direction.0, direction.1)
                ))
                .unwrap()
        );
    }
}

use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Beam {
    position: (i32, i32),
    direction: (i32, i32),
}

fn is_position_valid(x: i32, y: i32, limit_x: i32, limit_y: i32) -> bool {
    return x >= 0 && x < limit_x && y >= 0 && y < limit_y;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let limit_x: i32 = input.lines().count() as i32;
    let limit_y: i32 = input.lines().next().unwrap().len() as i32;

    let mut mirrors: HashMap<(i32, i32), char> = HashMap::new();
    let mut splitters: HashMap<(i32, i32), char> = HashMap::new();

    for (line, y) in input.lines().zip(0..limit_y) {
        for (c, x) in line.chars().zip(0..limit_x) {
            if r"\/".contains(c) {
                mirrors.insert((x as i32, y as i32), c);
            } else if "|-".contains(c) {
                splitters.insert((x as i32, y as i32), c);
            }
        }
    }

    let mut beams: Vec<Beam> = vec![Beam {
        position: (0, 0),
        direction: (1, 0),
    }];

    let mut energized: HashSet<(i32, i32)> = HashSet::new();
    let mut beam_history: HashSet<Beam> = HashSet::new();

    while beams.len() > 0 {
        let mut beam: Beam = beams.pop().unwrap();

        if beam_history.contains(&beam) {
            continue;
        }
        
        while is_position_valid(beam.position.0, beam.position.1, limit_x, limit_y) {
            beam_history.insert(beam.clone());
            energized.insert(beam.position);

            match mirrors.get(&beam.position) {
                Some(mirror_type) => {
                    beam.direction = (beam.direction.1, beam.direction.0);
                    if mirror_type == &'/' {
                        beam.direction = (beam.direction.0 * (-1), beam.direction.1 * (-1));
                    }
                }
                None => match splitters.get(&beam.position) {
                    Some(splitter_type) => {
                        if splitter_type == &'|' {
                            if beam.direction.0 != 0 {
                                beam.direction = (0, 1);

                                beams.push(Beam {
                                    position: beam.position,
                                    direction: (0, -1),
                                })
                            }
                        } else {
                            if beam.direction.1 != 0 {
                                beam.direction = (1, 0);

                                beams.push(Beam {
                                    position: beam.position,
                                    direction: (-1, 0),
                                })
                            }
                        }
                    }
                    None => (),
                },
            }

            beam.position = (
                beam.position.0 + beam.direction.0,
                beam.position.1 + beam.direction.1,
            );
        }
    }

    // println!("{:?}", energized);
    println!("{}", energized.len());
}

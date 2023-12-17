use std::collections::HashSet;

fn calculate_load(rocks: &HashSet<(i32, i32)>, total_rows: i32) -> i32 {
    return rocks.iter().map(|(_, y)| total_rows - y).sum();
}

fn position_in_limits(x: i32, y: i32, limits: (i32, i32), direction: (i32, i32)) -> bool {
    return match direction {
        (0, -1) => y > 0,
        (-1, 0) => x > 0,
        (0, 1) => y < limits.1 - 1,
        (1, 0) => x < limits.0 - 1,
        _ => false,
    };
}

fn tilt_platform(
    rounded_rocks: &HashSet<(i32, i32)>,
    cube_rocks: &HashSet<(i32, i32)>,
    direction: (i32, i32),
    limits: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut rocks_after_tilt: HashSet<(i32, i32)> = HashSet::new();

    for (mut x, mut y) in rounded_rocks {
        let mut rocks_found: i32 = 0;

        // println!("x {}, y {}", x, y);

        while position_in_limits(x, y, limits, direction)
            && !cube_rocks.contains(&(x + direction.0, y + direction.1))
        {
            x += direction.0;
            y += direction.1;

            // println!("x {}, y {}", x, y);

            if rounded_rocks.contains(&(x, y)) {
                rocks_found += 1;
            }
        }

        // println!("rocks found: {}", rocks_found);
        // println!(
        //     "x {}, y {}",
        //     x - direction.0 * rocks_found,
        //     y - direction.1 * rocks_found
        // );
        // println!("");

        rocks_after_tilt.insert((x - direction.0 * rocks_found, y - direction.1 * rocks_found));
    }

    return rocks_after_tilt;
}

fn spin_cycle(
    rounded_rocks: &HashSet<(i32, i32)>,
    cube_rocks: &HashSet<(i32, i32)>,
    limits: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut rocks_after_cycle: HashSet<(i32, i32)> =
        tilt_platform(rounded_rocks, cube_rocks, (0, -1), limits);
    // print_rocks(&rocks_after_cycle, cube_rocks, limits);

    rocks_after_cycle = tilt_platform(&rocks_after_cycle, cube_rocks, (-1, 0), limits);
    // print_rocks(&rocks_after_cycle, cube_rocks, limits);

    rocks_after_cycle = tilt_platform(&rocks_after_cycle, cube_rocks, (0, 1), limits);
    // print_rocks(&rocks_after_cycle, cube_rocks, limits);

    rocks_after_cycle = tilt_platform(&rocks_after_cycle, cube_rocks, (1, 0), limits);
    // print_rocks(&rocks_after_cycle, cube_rocks, limits);

    return rocks_after_cycle;
}

fn print_rocks(
    rounded_rocks: &HashSet<(i32, i32)>,
    cube_rocks: &HashSet<(i32, i32)>,
    limits: (i32, i32),
) {
    let rocks: String = rocks_to_string(rounded_rocks, cube_rocks, limits);
    for (i, c) in rocks.char_indices() {
        if i > 0 && i as i32 % limits.0 == 0 {
            println!("");
        }
        print!("{}", c);
    }

    println!("");
}

fn rocks_to_string(
    rounded_rocks: &HashSet<(i32, i32)>,
    cube_rocks: &HashSet<(i32, i32)>,
    limits: (i32, i32),
) -> String {
    let mut result: String = String::new();
    for y in 0..limits.1 {
        for x in 0..limits.0 {
            if rounded_rocks.contains(&(x, y)) {
                result.push('O');
            } else if cube_rocks.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
    }

    return result;
}

fn main() {
    let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./input.txt");

    let total_rows: i32 = input.lines().count() as i32;
    let total_columns: i32 = input.lines().next().unwrap().len() as i32;

    let limits: (i32, i32) = (total_rows, total_columns);

    let mut rounded_rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut cube_rocks: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'O' {
                rounded_rocks.insert((x as i32, y as i32));
            } else if c == '#' {
                cube_rocks.insert((x as i32, y as i32));
            }
        }
    }

    let mut rocks: HashSet<(i32, i32)> = HashSet::from(rounded_rocks);
    let mut maps: Vec<String> = Vec::new();
    let mut start: usize = 0;
    let mut end: usize = 0;

    loop {
        rocks = spin_cycle(&rocks, &cube_rocks, limits);

        let str_rocks: String = rocks_to_string(&rocks, &cube_rocks, limits);

        if maps.contains(&str_rocks) {
            start = maps.iter().position(|r| *r == str_rocks).unwrap();
            end = maps.len();
            break;
        } else {
            maps.push(str_rocks);
        }
    }

    let cycles_remaining: usize = (1000000000 - start - 1) % (end - start);
    for _ in 0..cycles_remaining {
        rocks = spin_cycle(&rocks, &cube_rocks, limits);
    }

    println!("{}", calculate_load(&rocks, total_rows));
}

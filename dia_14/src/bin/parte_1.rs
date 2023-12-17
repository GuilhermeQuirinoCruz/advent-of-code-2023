use std::collections::HashSet;

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let total_rows: u32 = input.lines().count() as u32;

    let mut rounded_rocks: HashSet<(u32, u32)> = HashSet::new();
    let mut cube_rocks: HashSet<(u32, u32)> = HashSet::new();
    let mut rocks_after_slide: HashSet<(u32, u32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'O' {
                rounded_rocks.insert((x as u32, y as u32));
            } else if c == '#' {
                cube_rocks.insert((x as u32, y as u32));
            }
        }
    }

    for (x, mut y) in &rounded_rocks {
        let mut rocks_found: u32 = 0;

        while y > 0 && !cube_rocks.contains(&(*x, y - 1)) {
            y -= 1;

            if rounded_rocks.contains(&(*x, y)) {
                rocks_found += 1;
            }
        }

        rocks_after_slide.insert((*x, y + rocks_found));
    }

    let sum: u32 = rocks_after_slide.iter().map(|(_, y)| total_rows - y).sum();
    println!("{}", sum);
}
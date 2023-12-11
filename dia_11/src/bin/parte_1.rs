#[derive(Debug)]
struct Galaxy {
    x: u32,
    y: u32,
}

fn count_empty_lines_between(mut x1: u32, mut x2: u32, lines: &Vec<u32>) -> u32 {
    if x1 > x2 {
        (x1, x2) = (x2, x1);
    }

    return lines.iter().filter(|i| **i > x1 && **i < x2).count() as u32;
}

fn distance_between_galaxies(
    galaxy_1: &Galaxy,
    galaxy_2: &Galaxy,
    empty_rows: &Vec<u32>,
    empty_columns: &Vec<u32>,
) -> u32 {
    let x_distance = galaxy_2.x.abs_diff(galaxy_1.x)
        + count_empty_lines_between(galaxy_1.x, galaxy_2.x, empty_columns);

    let y_distance = galaxy_2.y.abs_diff(galaxy_1.y)
        + count_empty_lines_between(galaxy_1.y, galaxy_2.y, empty_rows);

    return x_distance + y_distance;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let mut empty_rows: Vec<u32> = (1..input.lines().count() + 1).map(|n| n as u32).collect();
    let mut empty_columns: Vec<u32> = (1..input.lines().clone().next().unwrap().len() + 1)
        .map(|n| n as u32)
        .collect();

    let mut galaxies: Vec<Galaxy> = Vec::new();

    for (line, y) in input
        .lines()
        .zip((1..input.lines().count() + 1).map(|y| y as u32))
    {
        for (c, x) in line.chars().zip((1..line.len() + 1).map(|x| x as u32)) {
            if c == '#' {
                galaxies.push(Galaxy { x, y });

                if empty_columns.contains(&x) {
                    empty_columns.remove(empty_columns.iter().position(|i| *i == x).unwrap());
                }

                if empty_rows.contains(&y) {
                    empty_rows.remove(empty_rows.iter().position(|i| *i == y).unwrap());
                }
            }
        }
    }

    let mut sum: u32 = 0;
    for i in 0..galaxies.len() - 1 {
        for i2 in (i + 1)..galaxies.len() {
            sum += distance_between_galaxies(&galaxies[i], &galaxies[i2], &empty_rows, &empty_columns);
        }
    }

    println!("{}", sum);
}

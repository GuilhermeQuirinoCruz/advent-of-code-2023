use std::collections::HashSet;

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut garden_plots: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rocks.insert((x as i32, y as i32));
            } else if c == 'S' {
                garden_plots.insert((x as i32, y as i32));
            }
        }
    }

    let limits: (i32, i32) = (
        input.lines().count() as i32,
        input.lines().next().unwrap().len() as i32,
    );

    let mut remaining_steps: u8 = 64;
    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    while remaining_steps > 0 {
        remaining_steps -= 1;

        let next_plots: Vec<(i32, i32)> = garden_plots.iter().map(|plot| *plot).collect();
        garden_plots.clear();

        for plot in next_plots {
            for direction in &directions {
                let position: (i32, i32) = (plot.0 + direction.0, plot.1 + direction.1);
                // println!("checking {:?}", position);

                if !(0..limits.0).contains(&position.0) || !(0..limits.1).contains(&position.1) {
                    // println!("out of range");
                    continue;
                }

                if rocks.contains(&position) || garden_plots.contains(&position) {
                    // println!("rocks or already in plots");
                    continue;
                }

                garden_plots.insert(position);
                // println!("all good");
            }
        }

        // println!("current plots {:?}", garden_plots);
        // println!("");
    }

    println!("{}", garden_plots.len());
}

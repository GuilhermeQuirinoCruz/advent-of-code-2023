fn predict_previous_value(history: &Vec<i32>) -> i32 {
    let mut differences: Vec<i32> = Vec::new();
    differences.extend(history);

    let mut last_values: Vec<i32> = Vec::new();

    while !differences.iter().all(|&d| d.eq(&0)) {
        for i in 0..(differences.len() - 1) {
            differences[i] = differences[i + 1] - differences[i];
        }

        last_values.push(*differences.last().unwrap());

        differences.pop();
        // println!("{:?}", differences);
    }

    // println!("{:?}", last_values);
    // println!("");

    return last_values.iter().sum::<i32>();
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|v| v.parse::<i32>().unwrap())
                .rev()
                .collect()
        })
        .collect();

    let sum_extrapolated: i32 = histories
        .iter()
        .map(|history| predict_previous_value(history))
        .sum::<i32>();

    println!("{}", sum_extrapolated);
}
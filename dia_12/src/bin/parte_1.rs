#[derive(Debug)]
struct Spring {
    condition_record: String,
    groups: Vec<u32>,
}

fn count_damaged_springs_group(springs: &str) {
    let a = springs.split(".").count();
    println!("{}", a);
}

fn main() {
    let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./input.txt");

    let springs: Vec<Spring> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(condition, groups)| Spring {
            condition_record: condition.to_string(),
            groups: groups
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
        })
        .collect();

    println!("{:?}", springs);
}

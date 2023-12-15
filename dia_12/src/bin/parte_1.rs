fn main() {
    let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./input.txt");

    let spring_size = input
        .lines()
        .map(|l| l.split_whitespace().next().unwrap())
        .map(|springs| springs.matches("?").count())
        .collect::<Vec<usize>>();

    println!(
        "min {:?}, max {:?}",
        spring_size.iter().min().unwrap(),
        spring_size.iter().max().unwrap()
    );
}

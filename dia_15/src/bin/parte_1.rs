fn get_ascii_code(c: char) -> u32 {
    return c as u32;
}

fn get_hash(str: &str) -> u32 {
    let mut hash: u32 = 0;

    for c in str.chars() {
        hash += get_ascii_code(c);
        hash *= 17;
        hash %= 256;
    }

    return hash;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let sum: u32 = input
        .split(",")
        .map(|str| get_hash(str))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();

    println!("{}", sum);
}

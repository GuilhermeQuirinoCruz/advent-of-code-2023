fn count_different_chars(line1: &String, line2: &String) -> i32 {
    let mut diff: i32 = 0;
    for (c1, c2) in line1.chars().zip(line2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
    }

    return diff;
}

fn find_line_of_reflection(lines: &Vec<String>) -> i32 {
    for (mut i, mut i2) in (0..lines.len() - 1).zip(1..lines.len()) {
        let mut different_chars: i32 = 0;
        let start = i + 1;

        while i2 < lines.len() {
            different_chars += count_different_chars(&lines[i], &lines[i2]);

            if i == 0 || different_chars > 1 {
                break;
            }

            i -= 1;
            i2 += 1;
        }

        if different_chars == 1 {
            return start as i32;
        }
    }

    return -1;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let mut sum: i32 = 0;

    for pattern in input.split("\n\n") {
        let mut rows: Vec<String> = Vec::new();
        let mut columns: Vec<String> = vec!["".to_string(); pattern.lines().next().unwrap().len()];

        for line in pattern.split("\n") {
            rows.push(line.to_string());

            for (i, c) in line.chars().enumerate() {
                columns[i].push(c);
            }
        }

        let start: i32 = find_line_of_reflection(&columns);
        if start != -1 {
            sum += start;
        } else {
            sum += find_line_of_reflection(&rows) * 100;
        }
    }

    println!("{}", sum);
}

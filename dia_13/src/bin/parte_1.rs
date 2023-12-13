fn find_line_of_reflection(lines: &Vec<String>) -> i32 {
    for (mut i, mut i2) in (0..lines.len() - 1).zip(1..lines.len()) {
        let mut symmetrical: bool = true;
        let start = i + 1;

        while i2 < lines.len() {
            if lines[i] != lines[i2] {
                symmetrical = false;
                break;
            }

            if i == 0 {
                break;
            }

            i -= 1;
            i2 += 1;
        }

        if symmetrical {
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

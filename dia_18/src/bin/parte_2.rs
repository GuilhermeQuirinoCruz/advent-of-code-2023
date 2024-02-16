use substring::Substring;

#[derive(Debug)]
struct Instruction {
    direction: (i64, i64),
    move_amount: u64,
}

fn get_direction_from_char(c: char) -> (i64, i64) {
    match c {
        '3' => (0, 1),
        '1' => (0, -1),
        '2' => (-1, 0),
        '0' => (1, 0),
        _ => (0, 0),
    }
}

fn get_lagoon_vertices(instructions: &Vec<Instruction>) -> Vec<(i64, i64)> {
    let mut vertices: Vec<(i64, i64)> = Vec::new();

    let mut current_position: (i64, i64) = (1, 1);
    for instruction in instructions {
        let move_amount: i64 = instruction.move_amount as i64;

        current_position = (
            current_position.0 + (instruction.direction.0 * move_amount),
            current_position.1 + (instruction.direction.1 * move_amount),
        );

        vertices.push(current_position);
    }

    let i: usize = vertices.len() - 3;
    if (vertices[i].0 == vertices[i + 1].0 && vertices[i + 1].0 == vertices[i + 2].0)
        || (vertices[i].1 == vertices[i + 1].1 && vertices[i + 1].1 == vertices[i + 2].1)
    {
        vertices.pop();
    }

    vertices.reverse();
    return vertices;
}

fn get_lagoon_area(vertices: &Vec<(i64, i64)>) -> i64 {
    let mut area: i64 = 0;
    let vertex_count: usize = vertices.len();
    for i in 0..vertices.len() {
        let vertex_a: (i64, i64) = vertices[i];
        let vertex_b: (i64, i64) = vertices[(i + 1) % vertex_count];

        let det: i64 = (vertex_a.0 * vertex_b.1) - (vertex_b.0 * vertex_a.1);
        area += det;
    }

    return area.abs() / 2;
}

fn get_perimeter(instructions: &Vec<Instruction>) -> i64 {
    return instructions.iter().map(|i| i.move_amount).sum::<u64>() as i64;
}

fn get_corner_delta(instructions: &Vec<Instruction>) -> i64 {
    let mut delta: i64 = 0;
    for i in 0..instructions.len() {
        let dir_a: (i64, i64) = instructions[i].direction;
        let dir_b: (i64, i64) = instructions[(i + 1) % instructions.len()].direction;

        if dir_a.1 != 0 {
            delta += dir_a.1 * dir_b.0;
        } else {
            delta += -(dir_a.0 * dir_b.1);
        }
    }

    return delta;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./exemplo_2.txt");
    // let input: &str = include_str!("./exemplo_3.txt");
    // let input: &str = include_str!("./exemplo_4.txt");
    let input: &str = include_str!("./input.txt");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.split_once(" (#").unwrap().1)
        .map(|rs| rs.replace(")", ""))
        .map(|code| Instruction {
            direction: get_direction_from_char(code.substring(5, 6).chars().next().unwrap()),
            move_amount: u64::from_str_radix(code.substring(0, 5), 16).unwrap(),
        })
        .collect();

    let mut total_area: i64 = get_lagoon_area(&get_lagoon_vertices(&instructions));
    total_area += get_perimeter(&instructions) / 2;
    total_area += get_corner_delta(&instructions) / 4;

    println!("{}", total_area);
}

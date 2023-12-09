use std::collections::HashMap;

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./exemplo_2.txt");
    let input: &str = include_str!("./input.txt");

    let lr_instructions: &str;
    let labeled_nodes: &str;

    (lr_instructions, labeled_nodes) = input.split_once("\n\n").unwrap();

    let nodes: HashMap<String, (String, String)> =
        HashMap::from_iter(labeled_nodes.lines().map(|line| {
            let (label, dest_pair) = line.split_once(" = ").unwrap();
            let (left, right) = dest_pair.split_once(", ").unwrap();
            (
                label.to_string(),
                (left.replace("(", ""), right.replace(")", "")),
            )
        }));

    let mut steps: usize = 0;
    let instructions: usize = lr_instructions.len();
    let mut current_node: &str = "AAA";
    loop {
        let (left, right) = nodes.get(current_node).unwrap();
        let direction: char = lr_instructions.chars().nth(steps % instructions).unwrap();
        // println!(
        //     "step {}, node {}, l/r {}/{}, dir {}",
        //     steps, current_node, left, right, direction
        // );
        if direction == 'L' {
            current_node = left;
        } else {
            current_node = right;
        }

        steps += 1;

        if current_node == "ZZZ" {
            break;
        }
    }

    println!("{}", steps);
}

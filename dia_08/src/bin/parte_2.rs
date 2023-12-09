use std::collections::HashMap;
use num::integer::lcm;

fn steps_to_z(
    nodes: &HashMap<String, (String, String)>,
    starting_node: &str,
    instructions: &str,
) -> u64 {
    let mut steps: usize = 0;
    let instructions_length: usize = instructions.len();
    let mut current_node: &str = starting_node;
    loop {
        let (left, right) = nodes.get(current_node).unwrap();
        let direction: char = instructions
            .chars()
            .nth(steps % instructions_length)
            .unwrap();

        current_node = if direction == 'L' { &left } else { &right };

        steps += 1;

        if current_node.ends_with("Z") {
            return steps as u64;
        }
    }
}

fn lcm_vec(steps_per_node: Vec<u64>) -> u64 {
    let mut result: u64 = 1;
    for steps in steps_per_node {
        result = lcm(result, steps);
    }

    return result;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./exemplo_2.txt");
    // let input: &str = include_str!("./exemplo_3.txt");
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

    let current_nodes: Vec<&str> = nodes
        .keys()
        .filter(|label| label.ends_with("A"))
        .map(|label| label.as_str())
        .collect();

    let steps_per_node: Vec<u64> = current_nodes
        .iter()
        .map(|node| steps_to_z(&nodes, node, lr_instructions))
        .collect();

    println!("{}", lcm_vec(steps_per_node));
}

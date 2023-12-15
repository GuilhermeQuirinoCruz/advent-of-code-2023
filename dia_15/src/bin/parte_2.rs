#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Clone, Debug)]
struct Box {
    lenses: Vec<Lens>,
}

fn get_ascii_code(c: char) -> u32 {
    return c as u32;
}

fn get_hash(str: &str) -> usize {
    let mut hash: u32 = 0;

    for c in str.chars() {
        hash += get_ascii_code(c);
        hash *= 17;
        hash %= 256;
    }

    return hash as usize;
}

fn find_lens_by_label(lenses: &Vec<Lens>, label: String) -> i32 {
    for (i, lens) in lenses.iter().enumerate() {
        if lens.label == label {
            return i as i32;
        }
    }

    return -1;
}

fn proccess_instruction(instruction: &str, mut boxes: Vec<Box>) -> Vec<Box> {
    if instruction.contains("=") {
        let label: &str;
        let focal_length: &str;

        (label, focal_length) = instruction.split_once("=").unwrap();

        let hash: usize = get_hash(label);
        let index: i32 = find_lens_by_label(&boxes[hash].lenses, label.to_string());

        if index == -1 {
            boxes[hash].lenses.push(Lens {
                label: label.to_string(),
                focal_length: focal_length.parse::<u8>().unwrap(),
            });
        } else {
            boxes[hash].lenses[index as usize].focal_length = focal_length.parse::<u8>().unwrap();
        }
    } else {
        let label: &str = &instruction.replace("-", "");

        let hash: usize = get_hash(label);
        let index: i32 = find_lens_by_label(&boxes[hash].lenses, label.to_string());

        if index != -1 {
            boxes[hash].lenses.remove(index as usize);
        }
    }

    return boxes;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let mut boxes: Vec<Box> = vec![Box { lenses: Vec::new() }; 256];

    for instruction in input.split(",") {
        boxes = proccess_instruction(instruction, boxes.clone());
    }

    let mut focusing_power: i32 = 0;

    for (box_number, box_i) in boxes.iter().enumerate() {
        for (slot, lens) in box_i.lenses.iter().enumerate() {
            focusing_power += (box_number as i32 + 1) * (slot as i32 + 1) * (lens.focal_length as i32);
        }
    }

    println!("{}", focusing_power);
}

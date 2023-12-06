fn parse_number(line: &str) -> u64 {
    return line
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>().unwrap();
}

fn get_lower_bound(total_time: u64, record_distance: u64) -> u64 {
    for time in 0..(total_time + 1) {
        let distance_traveled: u64 = (total_time - time) * time;
        // println!("time {}, dist {}", time, distance_traveled);
        if distance_traveled > record_distance {
            return time;
        }
    }

    return total_time;
}

fn get_upper_bound(total_time: u64, record_distance: u64) -> u64 {
    for time in (0..(total_time + 1)).rev() {
        let distance_traveled: u64 = (total_time - time) * time;
        // println!("time {}, dist {}", time, distance_traveled);
        if distance_traveled > record_distance {
            return time;
        }
    }

    return total_time;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let input_times: &str;
    let input_distances: &str;

    (input_times, input_distances) = input.split_once("\n").unwrap();

    let time: u64 = parse_number(input_times);
    let distance: u64 = parse_number(input_distances);

    let winning_range: u64 = get_upper_bound(time, distance) - get_lower_bound(time, distance) + 1;
    
    println!("{}", winning_range);
}

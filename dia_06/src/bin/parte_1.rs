fn parse_numbers(line: &str) -> Vec<i32> {
    return line
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .into_iter()
        .map(|t| t.parse::<i32>().unwrap())
        .collect();
}

fn get_lower_bound(total_time: i32, record_distance: i32) -> i32 {
    for time in 0..(total_time + 1) {
        let distance_traveled: i32 = (total_time - time) * time;
        // println!("time {}, dist {}", time, distance_traveled);
        if distance_traveled > record_distance {
            return time;
        }
    }

    return total_time;
}

fn get_upper_bound(total_time: i32, record_distance: i32) -> i32 {
    for time in (0..(total_time + 1)).rev() {
        let distance_traveled: i32 = (total_time - time) * time;
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

    let times: Vec<i32>;
    let distances: Vec<i32>;

    times = parse_numbers(input_times);
    distances = parse_numbers(input_distances);

    let mut winning_ranges: Vec<i32> = Vec::new();

    for i in 0..times.len() {
        let range: i32 =
            get_upper_bound(times[i], distances[i]) - get_lower_bound(times[i], distances[i]) + 1;

        winning_ranges.push(range);
    }
    
    println!("{}", winning_ranges.iter().product::<i32>());
}

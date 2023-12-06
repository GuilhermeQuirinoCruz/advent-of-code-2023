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

fn get_lower_bound(total_time: &i32, record_distance: &i32) -> i32 {
    for time in 0..(total_time + 1) {
        let distance_traveled: i32 = (total_time - time) * time;
        // println!("time {}, dist {}", time, distance_traveled);
        if distance_traveled > *record_distance {
            return time;
        }
    }

    return *total_time;
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

    let numbers_product: i32 = times
    .iter()
        .zip(distances)
        .map(|(time, distance)| {
            time - (2 * get_lower_bound(time, &distance)) + 1
        })
        .product::<i32>();
    
    println!("{}", numbers_product);
}

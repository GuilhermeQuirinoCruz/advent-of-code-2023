struct Map {
    dest: u64,
    source: u64,
    length: u64,
}

fn map_location_to_seed(location: u64, map_vec: &Vec<Vec<Map>>) -> u64 {
    let mut seed: u64 = location;
    for maps in map_vec.iter().rev() {
        for map in maps {
            // println!("checking map d[{}] s[{}] l[{}]", map.dest, map.source, map.length);
            if seed >= map.dest && seed < (map.dest + map.length) {
                seed = map.source + (seed - map.dest);
                // println!("found!");
                break;
            }
        }

        // println!("mapped to {}", seed);
    }

    return seed;
}

fn is_seed_in_range(seed: u64, seeds_range: &Vec<u64>) -> bool {
    for i in (0..seeds_range.len()).step_by(2) {
        if seed >= seeds_range[i] && seed < (seeds_range[i] + seeds_range[i + 1]) {
            return true;
        }
    }

    return false;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let seeds_line: &str;
    let maps_data: &str;

    (seeds_line, maps_data) = input.split_once("\n").unwrap();

    let seeds_range: Vec<u64> = seeds_line
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut maps: Vec<Vec<Map>> = Vec::new();

    for mut map in maps_data.trim_start().split("\n\n") {
        map = map.split_once(":\n").unwrap().1;

        let mut new_map: Vec<Map> = Vec::new();
        for linha in map.split("\n") {
            let map_data: Vec<u64> = linha
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            new_map.push(Map {
                dest: map_data[0],
                source: map_data[1],
                length: map_data[2],
            })
        }

        maps.push(new_map);
    }

    let mut min_location : u64 = 0;

    loop {
        let seed = map_location_to_seed(min_location, &maps);
        if is_seed_in_range(seed, &seeds_range) {
            break;
        }
        
        min_location += 1;
    }

    println!("{}", min_location);
}

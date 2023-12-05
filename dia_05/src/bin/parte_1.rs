struct Map {
    dest: u64,
    source: u64,
    length: u64,
}

fn map_seed_to_location(seed: u64, map_vec: &Vec<Vec<Map>>) -> u64 {
    let mut location: u64 = seed;

    for maps in map_vec {
        for map in maps {
            // println!("checking map d[{}] s[{}] l[{}]", map.dest, map.source, map.length);
            if location >= map.source && location < (map.source + map.length) {
                location = map.dest + (location - map.source);
                // println!("found!");
                break;
            }
        }

        // println!("mapped to {}", location);
    }

    return location;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let seeds_line: &str;
    let maps_data: &str;

    (seeds_line, maps_data) = input.split_once("\n").unwrap();

    let seeds: Vec<u64> = seeds_line
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

    let min_location = seeds
        .iter()
        .map(|s| map_seed_to_location(*s, &maps))
        .min()
        .unwrap();

    println!("{}", min_location);

    // for seed in seeds {
    //     println!("seed {}", seed);
    //     println!("location {}", map_seed_to_location(seed, &maps));
    // }
}

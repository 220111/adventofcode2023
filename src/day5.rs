use std::fs;

fn trace_back_through_all_maps(maps: &Vec<Vec<Vec<u64>>>, seed: &u64) -> u64 {
    let mut target: u64 = *seed;
    maps.iter().for_each(|y| {
        for map in y {
            if target >= map[1] && target <= map[1] + map[2] {
                target = target - map[1] + map[0];
                break;
            }
        }
    });
    target
}

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Couldn't read the file provided.");

    let blocks: Vec<&str> = contents.split_terminator("\n\n").collect();
    let seeds: Vec<u64> = blocks[0].split_terminator(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Parsing seed numbers failed."))
        .collect();

    let maps: Vec<Vec<Vec<u64>>> = blocks
        .iter()
        .enumerate()
        .filter(|(i, _val)| *i > 0)
        .map(|(_i, val)| {
            val.lines()
                .enumerate()
                .filter(|(i, _val)| *i > 0)
                .map(|(_i, val)| {
                    val.split_whitespace()
                        .map(|x| x.parse::<u64>().expect("Parsing map numbers failed."))
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect();

    let locations: Vec<u64> = seeds
        .iter()
        .map(|x| trace_back_through_all_maps(&maps, x))
        .collect();

    println!("Part 1:\n{:?}", locations.iter().min().unwrap());
}

fn trace_back_through_all_maps_with_range(maps: &Vec<Vec<Vec<u64>>>, seeds: &(u64,u64)) -> u64 {
    let (seed, range) = *seeds;
    let possible:Vec<u64> = (seed..seed+range).map(|x| trace_back_through_all_maps(&maps, &x)).collect();
    *possible.iter().min().unwrap()
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Couldn't read the file provided.");

    let blocks: Vec<&str> = contents.split_terminator("\n\n").collect();
    let seeds: Vec<u64> = blocks[0].split_terminator(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Parsing seed numbers failed."))
        .collect();

    let mut seed_ranges:Vec<(u64,u64)> = Vec::new();
    for index in 0..seeds.len()-1{
        if index%2 == 0 {
            seed_ranges.push((seeds[index], seeds[index+1]));
        }
    }

    let maps: Vec<Vec<Vec<u64>>> = blocks
        .iter()
        .enumerate()
        .filter(|(i, _val)| *i > 0)
        .map(|(_i, val)| {
            val.lines()
                .enumerate()
                .filter(|(i, _val)| *i > 0)
                .map(|(_i, val)| {
                    val.split_whitespace()
                        .map(|x| x.parse::<u64>().expect("Parsing map numbers failed."))
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect();

    let locations: Vec<u64> = seed_ranges
        .iter()
        .map(|x| trace_back_through_all_maps_with_range(&maps, x))
        .collect();

    println!("Part 2:\n{:?}", locations.iter().min().unwrap());
}

pub fn main() {
    println!("DAY 5:");
    let file_path: String = "./src/day5test.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

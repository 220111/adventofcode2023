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

fn part2(_file_path: String) {}

pub fn main() {
    println!("DAY 5:");
    let file_path: String = "./src/day5real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

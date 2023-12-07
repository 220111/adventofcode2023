use std::collections::HashMap;
use std::fs;

fn build_vec_with_map(map: &Vec<Vec<u32>>) -> HashMap<u32, u32> {
    let mut nums: HashMap<u32, u32> = HashMap::new();

    println!("building map");

    for line in map {
        (line[0]..line[0] + line[2]).for_each(|x| {
            nums.insert(x, line[1] + (x - line[0]));
        });
    }

    nums
}

fn trace_back_all_maps(maps: Vec<HashMap<u32, u32>>, seed: &u32) -> u32 {
    let mut target: u32 = *seed;
    maps.iter().for_each(|y| {
        let (a, _b) = y
            .iter()
            .find(|(_k, v)| **v == target)
            .unwrap_or((&target, &target));
        target = *a;
    });
    target
}

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let blocks: Vec<&str> = contents.split_terminator("\n\n").collect();
    let seeds: Vec<u32> = blocks[0].split_terminator(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("should be a number"))
        .collect();

    println!("Found seeds");

    let map_nums: Vec<Vec<Vec<u32>>> = blocks
        .iter()
        .enumerate()
        .filter(|(i, _val)| *i > 0)
        .map(|(_i, val)| {
            val.lines()
                .enumerate()
                .filter(|(i, _val)| *i > 0)
                .map(|(_i, val)| {
                    val.split_whitespace()
                        .map(|x| x.parse::<u32>().expect("should be numbers"))
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect();

    let maps: Vec<HashMap<u32, u32>> = map_nums.iter().map(|x| build_vec_with_map(x)).collect();
    
    

    let locations: Vec<u32> = seeds
        .iter()
        .map(|x| trace_back_all_maps(maps.clone(), x))
        .collect();

    println!("Part 1:\n{:?}", locations.iter().min().unwrap());
}

fn part2(_file_path: String) {}

pub fn main() {
    println!("DAY 4:");
    let file_path: String = "./src/day5real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

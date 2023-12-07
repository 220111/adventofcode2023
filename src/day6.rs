use std::fs;

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let time_string: &str = contents.split_terminator('\n').collect::<Vec<&str>>()[0]
        .split_terminator(':')
        .collect::<Vec<&str>>()[1]
        .trim();
    let distance_string: &str = contents.split_terminator('\n').collect::<Vec<&str>>()[1]
        .split_terminator(':')
        .collect::<Vec<&str>>()[1]
        .trim();

    let times: Vec<u32> = time_string
        .split_whitespace()
        .filter(|x| x != &"")
        .map(|x| x.parse().expect("should be a number"))
        .collect();
    let distances: Vec<u32> = distance_string
        .split_whitespace()
        .filter(|x| x != &"")
        .map(|x| x.parse().expect("should be a number"))
        .collect();

    let totals: Vec<u32> = times
        .iter()
        .enumerate()
        .map(|(i, time)| (0..*time).map(|x| (*time-x)*(x)).filter(|x| *x > distances[i]).count() as u32)
        .collect();

    println!("Part 1:\n{}", totals.iter().product::<u32>().to_string());
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let time_string: String = contents.split_terminator('\n').collect::<Vec<&str>>()[0]
        .split_terminator(':')
        .collect::<Vec<&str>>()[1]
        .trim().replace(" ", "");
    let distance_string: String = contents.split_terminator('\n').collect::<Vec<&str>>()[1]
        .split_terminator(':')
        .collect::<Vec<&str>>()[1]
        .trim().replace(" ", "");

    let time:u64 = time_string.parse().expect("should be number");
    let distance:u64 = distance_string.parse().expect("should be number");

    let total:u64 = (0..time).map(|x| (time-x)*(x)).filter(|x| *x > distance).count() as u64;

    println!("Part 2:\n{}", total.to_string());
}

pub fn main() {
    println!("DAY 6:");
    let file_path: String = "./src/day6real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

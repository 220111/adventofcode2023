use std::fs;

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total: u32 = 0;

    for line in contents.lines() {
        let mut game_total: u32 = 0;
        let split_line: Vec<&str> = line.split_terminator("|").collect();
        let winning: Vec<u32> = split_line[0].split_terminator(":").collect::<Vec<&str>>()[1]
            .trim()
            .split_terminator(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| x != &&"")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let actual: Vec<u32> = split_line[1]
            .trim()
            .split_terminator(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| x != &&"")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        actual.iter().for_each(|x| {
            if winning.contains(x) {
                if game_total == 0 {
                    game_total = 1;
                } else {
                    game_total = game_total * 2;
                }
            }
        });

        total += game_total;
    }

    println!("Part 1:\n{}", total.to_string());
}

fn part2(_file_path: String) {}

pub fn main() {
    println!("DAY 4:");
    let file_path: String = "./src/day4real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

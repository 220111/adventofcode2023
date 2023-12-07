use std::fs;

fn part1(file_path: String) {
    let _contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    

    println!("Part 1:\n{}", 3.to_string());
}

fn part2(_file_path: String) {
}

pub fn main() {
    println!("DAY 4:");
    let file_path: String = "./src/day5test.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

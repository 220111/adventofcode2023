use std::fs;

fn part1(file_path: String) {
    let _contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    

    println!("Part 1:\n{}", 3.to_string());
}

fn part2(_file_path: String) {
    
}

pub fn main() {
    println!("DAY 9:");
    let file_path: String = "./src/day4real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

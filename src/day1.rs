use std::fs;

pub fn main() {
    println!("DAY 1:");
    let file_path: String = "./src/day1real.txt".to_string();
    let contents =
        fs::read_to_string(file_path.clone()).expect("Should have been able to read the file");
    println!("In file {}", file_path);
    let mut total: u32 = 0;

    for line in contents.lines() {
        for c in line.chars() {
            if c.is_numeric() {
                total += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                total += c.to_digit(10).unwrap();
                break;
            }
        }
    }

    println!("With text:\n{}", total.to_string());
}

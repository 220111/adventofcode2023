use std::fs;

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut possible: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let l: Vec<&str> = line.split_terminator(":").collect();

        //Get number of game
        let n: u32 = l[0]
            .trim_start_matches("Game ")
            .parse()
            .expect("Should be a number");

        //Get if possible
        let mut pos: bool = true;
        for play in l[1].split_terminator(";") {
            for cube in play.split_terminator(",") {
                let v: Vec<&str> = cube.trim().split_whitespace().collect();
                let num: u32 = v[0].parse().expect("Should be a number");
                match v[1] {
                    "blue" => {
                        if num > 14 {
                            pos = false
                        }
                    }
                    "red" => {
                        if num > 12 {
                            pos = false
                        }
                    }
                    "green" => {
                        if num > 13 {
                            pos = false
                        }
                    }
                    _ => println!("Something went way wrong"),
                }
            }
        }
        if pos {
            possible.push(n);
        }
    }

    println!("Part 1:\n{}", possible.iter().sum::<u32>().to_string());
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut values: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let l: Vec<&str> = line.split_terminator(":").collect();

        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        let mut red: u32 = 0;

        for play in l[1].split_terminator(";") {
            for cube in play.split_terminator(",") {
                let v: Vec<&str> = cube.trim().split_whitespace().collect();
                let num: u32 = v[0].parse().expect("Should be a number");
                match v[1] {
                    "blue" => {
                        if num > blue {
                            blue = num
                        }
                    }
                    "red" => {
                        if num > red {
                            red = num
                        }
                    }
                    "green" => {
                        if num > green {
                            green = num
                        }
                    }
                    _ => println!("Something went way wrong"),
                }
            }
        }
        values.push(blue * green * red);
    }

    println!("Part 2:\n{}", values.iter().sum::<u32>().to_string());
}

pub fn main() {
    println!("DAY 2:");
    let file_path: String = "./src/day2real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

use std::fs;
use std::cmp;
use std::collections::HashMap;

fn part1(file_path:String) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut total:u32 = 0;
    let mut grid:Vec<Vec<char>> = Vec::new();

    //collect chars into vectors
    for line in contents.lines() {
        let chars: Vec<char> = line.chars()
            .collect();
        grid.push(chars);
    }

    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if grid[i][j].is_numeric(){
                if j == 0 || !grid[i][j-1].is_numeric() {
                    let mut nums:Vec<char> = Vec::new();
                    let mut a:usize = 0;
                    while j+a < grid[i].len() && grid[i][j+a].is_numeric(){
                        nums.push(grid[i][j+a]);
                        a += 1;
                    }
                    
                    let num:u32 = nums.iter().map(|x| x.to_digit(10).expect("Should be num")).collect::<Vec<u32>>().iter().fold(0, |acc, elem| acc * 10 + elem);

                    //check if a good number
                    let starth:usize = j.saturating_sub(1);
                    let startv:usize = i.saturating_sub(1);
                    let endh:usize = cmp::min(j+a, grid[i].len()-1);
                    let endv:usize = cmp::min(i+1, grid.len()-1);


                    for v in startv..=endv{
                        for h in starth..=endh{
                            if !grid[v][h].is_numeric() && grid[v][h] != '.' {
                                 //.iter().fold(0, |acc, elem| acc * 10 + elem)
                                total += num;
                                break;
                            }
                        }
                    }

                }
            }
        }
    }

    println!("Part 1:\n");
    println!("{}", total.to_string())
}

fn part2(file_path:String){
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut total:u32 = 0;
    let mut grid:Vec<Vec<char>> = Vec::new();
    let mut gears = HashMap::new();

    //collect chars into vectors
    for line in contents.lines() {
        let chars: Vec<char> = line.chars()
            .collect();
        grid.push(chars);
    }

    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if grid[i][j].is_numeric(){
                if j == 0 || !grid[i][j-1].is_numeric() {
                    let mut nums:Vec<char> = Vec::new();
                    let mut a:usize = 0;
                    while j+a < grid[i].len() && grid[i][j+a].is_numeric(){
                        nums.push(grid[i][j+a]);
                        a += 1;
                    }
                    
                    let num:u32 = nums.iter().map(|x| x.to_digit(10).expect("Should be num")).collect::<Vec<u32>>().iter().fold(0, |acc, elem| acc * 10 + elem);

                    //check if a good number
                    let starth:usize = j.saturating_sub(1);
                    let startv:usize = i.saturating_sub(1);
                    let endh:usize = cmp::min(j+a, grid[i].len()-1);
                    let endv:usize = cmp::min(i+1, grid.len()-1);


                    for v in startv..=endv{
                        for h in starth..=endh{
                            if grid[v][h] == '*' {
                                let key:usize = v*1000+h;

                                if gears.contains_key(&key){
                                    let old = gears.get(&key).expect("already checked for key");
                                    total += old*num;
                                }else{
                                    gears.insert(key, num);
                                }
                                break;
                            }
                        }
                    }

                }
            }
        }
    }

    println!("Part 2:\n{}", total.to_string());

}


pub fn main() {
    println!("DAY 3:");
    let file_path:String = "./src/day3real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}
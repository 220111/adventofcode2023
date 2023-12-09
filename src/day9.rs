use std::fs;

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let mut total:i64 = 0; 

    for line in contents.lines(){
        let mut runs:Vec<Vec<i64>> = Vec::new();

        runs.push(line.split_whitespace().map(|x| x.parse::<i64>().expect("should all be numbers")).collect());

        let mut cur:usize = 0;
        
        while !runs[cur].iter().all(|x| *x == 0){
            let mut next:Vec<i64> = Vec::new();
            for i in 0..runs[cur].len()-1{
                next.push(runs[cur][i+1]-runs[cur][i]);
            }
            runs.push(next);
            cur += 1;
        }

        let mut next:i64 = 0;

        for r in runs.iter().rev(){
            next += r[r.len()-1];
        }

        total += next;
    }

    println!("Part 1:\n{}", total);
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let mut total:i64 = 0; 

    for line in contents.lines(){
        let mut runs:Vec<Vec<i64>> = Vec::new();

        runs.push(line.split_whitespace().map(|x| x.parse::<i64>().expect("should all be numbers")).collect());

        let mut cur:usize = 0;
        
        while !runs[cur].iter().all(|x| *x == 0){
            let mut next:Vec<i64> = Vec::new();
            for i in 0..runs[cur].len()-1{
                next.push(runs[cur][i+1]-runs[cur][i]);
            }
            runs.push(next);
            cur += 1;
        }

        let mut next:i64 = 0;
        let mut i:usize = runs.len()-1;

        while i > 0 {
            next = runs[i-1][0] - next;
            i -= 1;
        }
        total += next;
    }

    println!("Part 2:\n{}", total);
}

pub fn main() {
    println!("DAY 9:");
    let file_path: String = "./src/day9real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

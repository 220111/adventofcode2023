use std::{collections::HashMap, fs, process::exit};

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let inst: Vec<u32> = contents.split_terminator("\n\n").collect::<Vec<&str>>()[0]
        .trim()
        .chars()
        .map(|x| match x {
            'L' => 0,
            'R' => 1,
            _ => exit(-1),
        })
        .collect();

    let mut map: HashMap<String, [String; 2]> = HashMap::new();

    contents.split_terminator("\n\n").collect::<Vec<&str>>()[1]
        .lines()
        .for_each(|line| {
            let line: Vec<&str> = line.split(" = ").collect();
            let dests: Vec<&str> = line[1].split(", ").collect();
            map.insert(
                String::from(line[0]),
                [dests[0].replace('(', ""), dests[1].replace(')', "")],
            );
        });

    let mut cur: String = String::from("AAA");
    let mut moves: u32 = 0;
    let mut inst_iter = inst.iter().cycle();

    while cur != *"ZZZ" {
        moves += 1;
        let next = inst_iter
            .next()
            .expect("This should be looping forever unless the instructions didn't get read.");
        cur = map.get(&cur).expect("Map is missing a value")[*next as usize].clone();
    }

    println!("Part 1:\n{}", moves);
}

fn part2(_file_path: String) {
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let inst: Vec<u32> = contents.split_terminator("\n\n").collect::<Vec<&str>>()[0]
    //     .trim()
    //     .chars()
    //     .map(|x| match x {
    //         'L' => 0,
    //         'R' => 1,
    //         _ => exit(-1),
    //     })
    //     .collect();

    // let mut string_map: HashMap<String, [String; 2]> = HashMap::new();

    // contents.split_terminator("\n\n").collect::<Vec<&str>>()[1]
    //     .lines()
    //     .for_each(|line| {
    //         let line: Vec<&str> = line.split(" = ").collect();
    //         let dests: Vec<&str> = line[1].split(", ").collect();
    //         string_map.insert(
    //             String::from(line[0]),
    //             [
    //                 String::from(dests[0].replace("(", "")),
    //                 String::from(dests[1].replace(")", "")),
    //             ],
    //         );
    //     });

    // let mut map: HashMap<&str, [&str; 2]> = HashMap::new();

    // let cur: Vec<&str> = contents.split_terminator("\n\n").collect::<Vec<&str>>()[1]
    //     .lines()
    //     .map(|line| line.split(" = ").collect::<Vec<&str>>()[0])
    //     .filter(|x| x.ends_with("A"))
    //     .collect();
    // let mut moves: u32 = 0;

    // let results = cur.iter()
    //     .map(|node| {
    //         let mut visited_nodes = vec![*node];
    //         let mut current_node = *node;

    //         inst.iter().cycle().enumerate().find_map(|(index, inst)| {
    //             let options = map.get(current_node).expect("always exists at a valid node");
    //             let next_node = options[*inst as usize];
    //             if next_node.ends_with("Z") {
    //                 Some(index+1)
    //             } else {
    //                 visited_nodes.push(next_node);
    //                 current_node = next_node;
    //                 None
    //             }
    //         })
    //         .expect("should find a cycle")
    //     })
    //     .collect::<Vec<usize>>();

    // println!("Part 1:\n{}", moves.to_string());
}

pub fn main() {
    println!("DAY 8:");
    let file_path: String = "./src/day8real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

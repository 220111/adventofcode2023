use std::fs;

fn generate_data(line: &str) -> (u32, u32, String) {
    let hand_string: String = line.split_whitespace().collect::<Vec<&str>>()[0]
        .chars()
        .map(|c| match c {
            'T' => 'a',
            'J' => 'b',
            'Q' => 'c',
            'K' => 'd',
            'A' => 'e',
            _ => c,
        })
        .collect::<String>();
    let hand_hex: u32 =
        u32::from_str_radix(&hand_string, 16).expect("Error parsing hex representation");
    let hand_value: u32 = line.split_whitespace().collect::<Vec<&str>>()[1]
        .parse()
        .expect("Error parsing hand value");
    (hand_hex, hand_value, String::from(hand_string))
}

enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

fn classify_hand(hand: &str) -> HandType {
    let mut cards: Vec<u32> = vec![0; 16];
    for c in hand.chars() {
        let hex: usize = c.to_digit(16).expect("Error parsing hex digit in classify") as usize;
        cards[hex] += 1;
    }
    let mut found: Vec<u32> = Vec::new();
    for num in cards {
        if num == 5 {
            return HandType::Five;
        } else if num == 4 {
            return HandType::Four;
        } else if num == 3 {
            if found.contains(&2) {
                return HandType::Full;
            }
            found.push(3);
        } else if num == 2 {
            if found.contains(&3) {
                return HandType::Full;
            } else if found.contains(&2) {
                return HandType::Two;
            }
            found.push(2);
        }
    }
    if found.contains(&3) {
        return HandType::Three;
    } else if found.contains(&2) {
        return HandType::One;
    }
    HandType::High
}

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut five: Vec<(u32, u32)> = Vec::new();
    let mut four: Vec<(u32, u32)> = Vec::new();
    let mut full: Vec<(u32, u32)> = Vec::new();
    let mut three: Vec<(u32, u32)> = Vec::new();
    let mut two: Vec<(u32, u32)> = Vec::new();
    let mut one: Vec<(u32, u32)> = Vec::new();
    let mut high: Vec<(u32, u32)> = Vec::new();

    let hands: Vec<(u32, u32, String)> = contents.lines().map(|x| generate_data(x)).collect();

    for hand in hands.clone() {
        let (num, val, hand_string) = hand;
        match classify_hand(&hand_string) {
            HandType::Five => {
                five.push((num, val));
            }
            HandType::Four => {
                four.push((num, val));
            }
            HandType::Full => {
                full.push((num, val));
            }
            HandType::Three => {
                three.push((num, val));
            }
            HandType::Two => {
                two.push((num, val));
            }
            HandType::One => {
                one.push((num, val));
            }
            HandType::High => {
                high.push((num, val));
            }
        }
    }

    five.sort_by_key(|(num, _value)| *num);
    four.sort_by_key(|(num, _value)| *num);
    full.sort_by_key(|(num, _value)| *num);
    three.sort_by_key(|(num, _value)| *num);
    two.sort_by_key(|(num, _value)| *num);
    one.sort_by_key(|(num, _value)| *num);
    high.sort_by_key(|(num, _value)| *num);

    let mut rank: u64 = 1;
    let mut total: u64 = 0;
    high.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    one.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    two.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    three.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    full.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    four.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    five.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });

    println!("Part 1:\n{}", total);
}

fn generate_data_jokers(line: &str) -> (u32, u32, String) {
    let hand_string: String = line.split_whitespace().collect::<Vec<&str>>()[0]
        .chars()
        .map(|c| match c {
            'T' => 'a',
            'J' => '0',
            'Q' => 'c',
            'K' => 'd',
            'A' => 'e',
            _ => c,
        })
        .collect::<String>();
    let hand_hex: u32 =
        u32::from_str_radix(&hand_string, 16).expect("Error parsing hex representation");
    let hand_value: u32 = line.split_whitespace().collect::<Vec<&str>>()[1]
        .parse()
        .expect("Error parsing hand value");
    (hand_hex, hand_value, String::from(hand_string))
}

fn classify_hand_joker(hand: &str) -> HandType {
    let mut cards: Vec<u32> = vec![0; 16];
    let mut jokers: u32 = 0;
    for c in hand.chars() {
        let hex: usize = c.to_digit(16).expect("Error parsing hex digit in classify") as usize;
        if hex == 0 {
            jokers += 1;
        }
        cards[hex] += 1;
    }

    let mut found: Vec<u32> = Vec::new();
    for (i, num) in cards.iter().enumerate() {
        if i != 0 {
            if *num == 5 {
                return HandType::Five;
            } else if *num == 4 {
                if jokers == 1 {
                    return HandType::Five;
                }
                return HandType::Four;
            } else if *num == 3 {
                if jokers == 2 {
                    return HandType::Five;
                } else if jokers == 1 {
                    return HandType::Four;
                }
                if found.contains(&2) {
                    return HandType::Full;
                }
                found.push(3);
            } else if *num == 2 {
                if jokers == 3 {
                    return HandType::Five;
                } else if jokers == 2 {
                    return HandType::Four;
                } else if jokers == 1 {
                    if found.contains(&3) {
                        return HandType::Full;
                    }
                    found.push(3);
                } else {
                    if found.contains(&3) {
                        return HandType::Full;
                    } else if found.contains(&2) {
                        return HandType::Two;
                    }
                    found.push(2);
                }
            }
        }
    }
    if found.contains(&3) {
        return HandType::Three;
    } else if found.contains(&2) {
        return HandType::One;
    }
    if jokers == 5 || jokers == 4 {
        return HandType::Five;
    } else if jokers == 3 {
        return HandType::Four;
    } else if jokers == 2 {
        return HandType::Three;
    } else if jokers == 1 {
        return HandType::One;
    }
    HandType::High
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut five: Vec<(u32, u32)> = Vec::new();
    let mut four: Vec<(u32, u32)> = Vec::new();
    let mut full: Vec<(u32, u32)> = Vec::new();
    let mut three: Vec<(u32, u32)> = Vec::new();
    let mut two: Vec<(u32, u32)> = Vec::new();
    let mut one: Vec<(u32, u32)> = Vec::new();
    let mut high: Vec<(u32, u32)> = Vec::new();

    let hands: Vec<(u32, u32, String)> =
        contents.lines().map(|x| generate_data_jokers(x)).collect();

    for hand in hands.clone() {
        let (num, val, hand_string) = hand;
        match classify_hand_joker(&hand_string) {
            HandType::Five => {
                five.push((num, val));
            }
            HandType::Four => {
                four.push((num, val));
            }
            HandType::Full => {
                full.push((num, val));
            }
            HandType::Three => {
                three.push((num, val));
            }
            HandType::Two => {
                two.push((num, val));
            }
            HandType::One => {
                one.push((num, val));
            }
            HandType::High => {
                high.push((num, val));
            }
        }
    }

    five.sort_by_key(|(num, _value)| *num);
    four.sort_by_key(|(num, _value)| *num);
    full.sort_by_key(|(num, _value)| *num);
    three.sort_by_key(|(num, _value)| *num);
    two.sort_by_key(|(num, _value)| *num);
    one.sort_by_key(|(num, _value)| *num);
    high.sort_by_key(|(num, _value)| *num);

    let mut rank: u64 = 1;
    let mut total: u64 = 0;
    high.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    one.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    two.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    three.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    full.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    four.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });
    five.iter().for_each(|(_num, value)| {
        total += rank * (*value as u64);
        rank += 1;
    });

    println!("Part 2:\n{}", total);
}

pub fn main() {
    println!("DAY 7:");
    let file_path: String = "./src/day7real.txt".to_string();
    println!("In file {}", file_path);

    part1(file_path.clone());
    part2(file_path.clone());
}

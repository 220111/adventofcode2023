mod day1;
mod day2;
mod day3;

use text_io::read;

fn main() {
    println!("Which day would you like to run?");
    let i: u32 = read!();
    match i {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        _ => println!("incorrect input"),
    }
}

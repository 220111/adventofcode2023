mod day1;
mod day2;

use text_io::read;

fn main() {
    println!("Which day would you like to run?");
    let i: u32 = read!();
    match i {
        1 => day1::main(),
        2 => day2::main(),
        _ => println!("incorrect input"),
    }
}

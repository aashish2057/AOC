use std::env; 

mod common;
mod y2015;

use common::inputs;
fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 3 {
        panic!("Usage cargo run <year> <day>")
    }

    let year: u32 = argv[1].parse().expect("Year must be a number");
    let day: u8 = argv[2].parse().expect("Day must be a number");

    println!("Running year {}, day {}", year, day);

    match (year, day) {
        (2015, 1) => y2015::day01::run(&inputs::read(year, day)),
        (_, _) => panic!("you haven't coded this one yet"),
    }
}

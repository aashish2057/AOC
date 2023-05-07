use std::fs::read_to_string;

use std::fs::File;
use std::io::{prelude::*, BufReader};
pub fn read(year: u32, day: u8) -> String {
    let path = format!("./inputs/y{}/day{:02}.txt", year, day);
    let s = read_to_string(path)
        .expect("File doesn't exist")
        .trim()
        .to_string();
    s
}

pub fn read_lines_into_vec(year: u32, day: u8) -> Vec<String> {
    let path = format!("./inputs/y{}/day{:02}.txt", year, day);
    let file = File::open(path).expect("File not found");
    let buf = BufReader::new(file);

    

    buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}

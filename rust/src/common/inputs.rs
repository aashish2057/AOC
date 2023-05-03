use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read(year:u32, day:u8) -> Vec<String> {
    let mut r = Vec::with_capacity(1000);
    let path = format!("./inputs/{}/day{:02}.txt",year, day);
    let fp = File::open(&path).expect(&format!("Can't open {}", path));
    let fp = BufReader::new(fp);
    for line in fp.lines() {
        r.push(line.unwrap());
    }
    return r;
}

use std::fs::read_to_string;
pub fn read(year:u32, day:u8) -> String  {
    let path = format!("./inputs/y{}/day{:02}.txt",year, day);
    let s = read_to_string(path).expect("File doesn't exist").trim().to_string();
    return s;
}


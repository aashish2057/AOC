use crate::twenty_fifteen::file_string::file_string;

// Day `1` Not Quite Lisp
//
// You are given a file input containing parenthesis, you start on floor 0 and you are to determine
// what floor you end on. An opening parenthesis means you go up one floor and a closing
// parenthesis means you go down one floor. 
// Steps:
// 1. Read in file 
// 2. set variable floor 
// 3. go through file char by char and increment or decrement floor by one depending on parenthesis
// 4. return floor
pub fn one() {
    println!("this is one.rs");

    let path = String::from("./twenty_fifteen/one.txt");
    let contents = file_string(path);

    let mut floor = 0;
    let mut position = 0;
    for c in contents.chars() {
        position += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("File done"),
        }
        if floor == -1 {
            println!("{} basement", position);
        }
    }
    println!("{}", floor);
}

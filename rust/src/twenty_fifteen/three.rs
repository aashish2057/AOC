// Santa is at a starting position, each char in the file represents a move left, right, up or
// down, you have to determine how many houses have recieved one present. Every time santa reaches
// a position a present in delivered even if he already visited said position
//
// Steps:
// 1. Read in the file into string 
// 2. Create variable to keep track of unique houses
// 3. Use a dictionary where they key is a tuple reppresenting santa (x, y) position and the value
//    is a boolean if he has visited the house or not 
//4. iterate through the string, +1, -1 the respective x or y position based on the char// 5. for
//   every char check if he has visited the house or not, if it he hasn't add one to the variable
//   of unique houses and set its value to true, if he has just continue
//   6. return the variable of unique houses

use std::collections::HashMap;

use super::file_string::file_string;


pub fn three() {
    let file = String::from("./twenty_fifteen/three.txt");
    let s = file_string(file);

    let mut unique_houses = 0;
    let mut santa = (0,0);
    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();
    
    grid.insert(santa, true);
    unique_houses += 1;

    for c in s.chars() {
        match c {
            '<' => santa = (santa.0-1, santa.1),
            '>' => santa = (santa.0+1, santa.1), 
            '^' => santa = (santa.0, santa.1+1),
            'v' => santa = (santa.0, santa.1-1),
            _ => println!("i messed up")
        }

        grid.entry(santa).or_insert_with(|| {
            unique_houses += 1;
            return true;
        });
    }
    println!("{}", unique_houses);
}

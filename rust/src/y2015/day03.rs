// Problem - Determine how many unique houses santa has dellivered presents
//
// Input - file of with 4 characters representing the directions he moves
// in a 2d plane 
//
// How to solve
//
// use a hashset to keep track of grid positions and if he has visited
// or (0,0) = true
//
// start him off at (0,0) and move according to the input, at each iteration
// check to see if the new position is true or false
// if true continue and do nothing
// if false mark the position true and increment one to unique houses
// return unique houses
//
// part 2 same as before there are just 2 santas, use mod to select between which one 
// from a list of tuples

use std::collections::HashMap;

pub fn run(input: &String) {
    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("Part1: {}, Part2: {}", p1, p2);
}

fn part1(input: &String) -> i32 {
    let mut santa = (0,0);
    let mut unique_houses = 0;
    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();

    grid.insert(santa, true);
    unique_houses += 1; 
    for c in input.chars() {
        match c {
            '<' => santa = (santa.0-1, santa.1),
            '>' => santa = (santa.0+1, santa.1),
            '^' => santa = (santa.0, santa.1+1),
            'v' => santa = (santa.0, santa.1-1),
            _ => panic!("File has invalid character"),
        }

        grid.entry(santa).or_insert_with(|| {
            unique_houses += 1;
            return true;
        });
    };

    return unique_houses;
}

fn part2(input: &String) -> i32 {
    let mut santa = [(0,0), (0,0)];
    let mut unique_houses = 0;
    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();

    grid.insert(santa[0], true);
    unique_houses += 1; 
    for (i, c) in input.chars().enumerate() {
        match c {
            '<' => santa[i % 2] = (santa[i % 2].0-1, santa[i % 2].1),
            '>' => santa[i % 2] = (santa[i % 2].0+1, santa[i % 2].1),
            '^' => santa[i % 2] = (santa[i % 2].0, santa[i % 2].1+1),
            'v' => santa[i % 2] = (santa[i % 2].0, santa[i % 2].1-1),
            _ => panic!("File has invalid character"),
        }

        grid.entry(santa[0]).or_insert_with(|| {
            unique_houses += 1;
            return true;
        });
        grid.entry(santa[1]).or_insert_with(|| {
            unique_houses += 1;
            return true;
        });
    };

    return unique_houses;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let s1 = String::from(">");
        let s2 = String::from("^>v<");
        let s3 = String::from("^v^v^v^v^v");

        assert_eq!(part1(&s1), 2);
        assert_eq!(part1(&s2), 4);
        assert_eq!(part1(&s3), 2);
    }

    #[test]
    fn test_part2() {
        let s1 = String::from("^v");
        let s2 = String::from("^>v<");
        let s3 = String::from("^v^v^v^v^v");
        assert_eq!(part2(&s1), 3);
        assert_eq!(part2(&s2), 3);
        assert_eq!(part2(&s3), 11);
    }
}

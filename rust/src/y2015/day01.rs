// Problem - What floor does santa end up on after all the steps
//
// Steps
// Santa starts on floor 0
// ( goes up one floor ) goes down one floor
// There is a basement so he could be negative floors
//
// set variable of santa = 0
// iterate through the string
// match ( as a +1
// match ) as a -1
// return the final floor
//
// part 2
// Do same thing as above just break when floor is -1 and return the index

pub fn run(input: &String) {
    let p1 = part1(input);
    let p2 = part2(input);
    println!("Part 1: {}, Part 2: {}", p1, p2);
}

fn part1(input: &String) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("File contains invalid character"),
        }
    }
    floor
}

fn part2(input: &String) -> usize {
    let mut floor = 0;
    let mut floor_num = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("File contains invalid character"),
        }
        if floor == -1 {
            floor_num = i;
            break;
        }
    }
    floor_num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "()()(((";
        assert_eq!(part1(&s.to_string()), 3);
    }

    #[test]
    fn test_part2() {
        let s = "(((((((())))))))))))";
        assert_eq!(part2(&s.to_string()), 17);
    }
}

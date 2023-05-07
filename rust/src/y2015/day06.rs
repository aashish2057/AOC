//
// Determine how many lights are lit based on a set of instructions for a grid
//
// 3 instruction types turn on, turn off and toggle
// rectangles given with each instruction in coord pairs 0,0 and 2,2
// after everything determine how many lights are lit
//
// Steps:
//
// Create variable for 2d matrix 0-999
//
// loop through input
// each iteration pull out instruction and grid locations
// make switches to 2d matric (0 representing off 1 reprsenting on)
//
// at the end loop through the 2d matrix and count the number of ones return that
pub fn run(input: &Vec<String>) {
    let p1 = part1(input);
    let p2 = part2(input);
    println!("Part 1: {}, Part2: {}", p1, p2);
}

struct Instruction {
    ins: String,
}

impl Instruction {
    fn get_instruction(&self) -> &str {
        let instructions = ["turn on", "turn off", "toggle"];
        let mut answer = " ";
        for instr in instructions {
            if self.ins.matches(instr).count() > 0 {
                answer = instr;
            }
        }

        answer
    }

    fn get_grid_points(&self) -> Vec<usize> {
        let x = self.ins.replace(',', " ");
        let y: Vec<&str> = x.split_whitespace().collect();
        let mut points: Vec<usize> = vec![];
        for s in y {
            if s.parse::<usize>().is_ok() {
                points.push(s.parse::<usize>().unwrap());
            }
        }

        points
    }
}

fn part1(input: &Vec<String>) -> i32 {
    let mut grid = vec![[0; 1000]; 1000];

    for i in input {
        let _s = Instruction { ins: i.to_string() };
        let light: u8;

        match _s.get_instruction() {
            "turn on" => light = 1,
            "turn off" => light = 0,
            "toggle" => light = 2,
            _ => panic!("well fuck"),
        }

        let points = _s.get_grid_points();
        let mut x1 = points[0];
        let mut y1 = points[1];
        let x2 = points[2];
        let y2 = points[3];
        while x1 <= x2 {
            while y1 <= y2 {
                if light == 2 {
                    if grid[x1][y1] == 1 {
                        grid[x1][y1] = 0;
                    } else {
                        grid[x1][y1] = 1;
                    }
                } else {
                    grid[x1][y1] = light;
                }
                y1 += 1;
            }
            y1 = points[1];
            x1 += 1;
        }
    }
    let mut count = 0;
    for x in grid.iter() {
        for y in x.iter() {
            match y {
                1 => count += 1,
                _ => continue,
            }
        }
    }
    count
}

fn part2(input: &Vec<String>) -> i32 {
    let mut grid = vec![[0; 1000]; 1000];

    for i in input {
        let _s = Instruction { ins: i.to_string() };
        let light: i32;

        match _s.get_instruction() {
            "turn on" => light = 1,
            "turn off" => light = -1,
            "toggle" => light = 2,
            _ => panic!("well fuck"),
        }

        let points = _s.get_grid_points();
        let mut x1 = points[0];
        let mut y1 = points[1];
        let x2 = points[2];
        let y2 = points[3];
        while x1 <= x2 {
            while y1 <= y2 {
                grid[x1][y1] += light;
                if grid[x1][y1] < 0 {
                    grid[x1][y1] = 0;
                }
                y1 += 1;
            }
            y1 = points[1];
            x1 += 1;
        }
    }
    let mut count = 0;
    for x in grid.iter() {
        for y in x.iter() {
            count += y;
        }
    }
    count
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test1: Vec<String> = vec![String::from("turn on 0,0 through 999,999")];
        let test2: Vec<String> = vec![String::from("toggle 0,0 through 999,0")];
        let test3: Vec<String> = vec![String::from("turn off 499,499 through 500,500")];
        assert_eq!(part1(&test1), 1000000);
        assert_eq!(part1(&test2), 1000);
        assert_eq!(part1(&test3), 0);
    }

    #[test]
    fn test_part2() {
        let test1: Vec<String> = vec![String::from("turn on 0,0 through 0,0")];
        let test2: Vec<String> = vec![String::from("toggle 0,0 through 999,999")];
        assert_eq!(part2(&test1), 1);
        assert_eq!(part2(&test2), 2000000);
    }
}

// Problem - Determine the amount of wrapping paper needed
//
// Input - file where each line represents the dimensions of a present
// Wrapping Paper = 2 (l*w + w*h + h*l) + min(l*w, w*h, h*l)
//
//
pub fn run(input: &Vec<String>) {
    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("Part1: {}, Part2: {}", p1, p2);
}

struct Presents {
    length: u32,
    width: u32,
    height: u32,
}

impl Presents {
    fn side1(&self) -> u32 {
        return self.length * self.width;
    }
    fn side2(&self) -> u32 {
        return self.width * self.height;
    }
    fn side3(&self) -> u32 {
        return self.length * self.height;
    }

    fn surface_area(&self) -> u32 {
        return 2 * (self.side1() + self.side2() + self.side3());
    }

    fn min_side(&self) -> u32 {
        return self.side1().min(self.side2()).min(self.side3());
    }

    fn area(&self) -> u32 {
        return self.length * self.width * self.height;
    }

    fn perimeter1(&self) -> u32 {
        return self.length + self.width;
    }

    fn perimeter2(&self) -> u32 {
        return self.width + self.height;
    }
    fn perimeter3(&self) -> u32 {
        return self.length + self.height;
    }
    fn shortest_perimeter(&self) -> u32 {
        return (2 * self.perimeter1())
            .min(2 * self.perimeter2())
            .min(2 * self.perimeter3());
    }
}

fn part1(input: &Vec<String>) -> u32 {
    let mut wrapping_paper = 0;
    for s in input {
        let vals: Vec<&str> = s.split('x').collect();

        let _p = Presents {
            length: vals
                .get(0)
                .expect("Missing number")
                .parse::<u32>()
                .expect("Not valid number"),
            width: vals
                .get(1)
                .expect("Missing number")
                .parse::<u32>()
                .expect("Not valid number"),
            height: vals
                .get(2)
                .expect("Missing number")
                .parse::<u32>()
                .expect("Not valid number"),
        };

        wrapping_paper += _p.surface_area() + _p.min_side();
    }
    return wrapping_paper;
}

fn part2(input: &Vec<String>) -> u32 {
    let mut bow = 0;
    for s in input {
        let vals: Vec<&str> = s.split('x').collect();

        let _p = Presents {
            length: vals
                .get(0)
                .expect("Missing number")
                .parse::<u32>()
                .expect("Not valid number"),
            width: vals
                .get(1)
                .expect("Missing number")
                .parse::<u32>()
                .expect("Not valid number"),
            height: vals
                .get(2)
                .expect("Missing number")
                .parse::<u32>()
                .expect("Not valid number"),
        };
        bow += _p.area() + _p.shortest_perimeter();
    }
    return bow;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let v = vec![
            String::from("13x14x12"),
            String::from("15x14x18"),
            String::from("8x6x4"),
        ];
        assert_eq!(part1(&v), 3074);
    }

    #[test]
    fn test_part2() {
        let v = vec![String::from("2x3x4"), String::from("1x1x10")];
        assert_eq!(part2(&v), 48);
    }
}

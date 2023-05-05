// problem - determine how many strings in santas text file are naughty or nice
//
// A string is nice if contains atleast 3 vowels (aeiou)
// A string is nice if at least one double occurance of a letter
// A string is nice if it doesn't contain ab, cd, pq, xy
//
// Steps:
// create variable to keep track of count of nice strings
// iterate through the vec string
// iterate through each string 
// check if they have atleast 3 vowels
// check if they have one occurance of a double letter
// check if it doesn't contain ab, cd, pq, xy
// if all pass increment one if not continue
// return count of nice strings

use std::collections::HashSet;

use fancy_regex::Regex;

pub fn run(input: &Vec<String>) {
    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("Part1: {}, Part2: {}", p1, p2);
}

struct NiceStrings {
    s: String,
}

impl NiceStrings {
    fn contains_three_vowels(&self) -> bool {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let valid: bool;
        let mut v_count = 0;
        for c in self.s.chars() {
            if v_count == 3 {
                break
            }

            if vowels.contains(&c) {
                v_count += 1
            }

        }
        valid = v_count == 3;
        return valid
    }

    fn contains_naughty_substring(&self) -> bool {
        let mut valid = false;

        if self.s.contains("ab") || self.s.contains("cd") || self.s.contains("pq") || self.s.contains("xy") {
            valid = true;
        }  
        return valid;
    }

    fn contains_duplicates(&self) -> bool {
        let mut valid = false;
        let mut prev = ' ';
        for c in self.s.chars() {
            if c == prev {
                valid = true;
                break
            }
            prev = c;
        }
        return valid;
    }

    fn contains_two_pairs(&self) -> bool {
        let rule1 = Regex::new(r"(..).*\1").unwrap();
        let rule2 = Regex::new(r"(.).\1").unwrap();
        return rule1.is_match(&self.s).unwrap() && rule2.is_match(&self.s).unwrap();
    }
} 


fn part1(input: &Vec<String>) -> u32 {

    let mut nice_string_count = 0;
    for st in input.iter() {
        let nstring = NiceStrings {
            s: st.to_string(),
        };
        if nstring.contains_three_vowels() && !nstring.contains_naughty_substring() && nstring.contains_duplicates() {
            nice_string_count += 1;
        }
        
    }
    return nice_string_count; 
}

fn part2(input: &Vec<String>)  -> u32 {
    let mut nice_string_count = 0;
    for st in input.iter() {
        let nstring = NiceStrings {
            s: st.to_string(),
        };
        if nstring.contains_two_pairs() {
            nice_string_count += 1;
        }
    }
    return nice_string_count; 
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_part1() {
        let s1 = vec![String::from("ugknbfddgicrmopn")];
        let s2 = vec![String::from("aaa")];
        let s3 = vec![String::from("jchzalrnumimnmhp")];
        let s4 = vec![String::from("haegwjzuvuyypxyu")];
        let s5 = vec![String::from("dvszwmarrgswjxmb")];
        

        assert_eq!(part1(&s1), 1);
        assert_eq!(part1(&s2), 1);
        assert_eq!(part1(&s3), 0);
        assert_eq!(part1(&s4), 0);
        assert_eq!(part1(&s5), 0);
    }

    #[test]
    fn test_part2() {
        let s1 = vec![String::from("qjhvhtzxzqqjkmpb")];
        let s2 = vec![String::from("xxyxx")];
        let s3 = vec![String::from("uurcxstgmygtbstg")];
        let s4 = vec![String::from("ieodomkazucvgmuy")];
        

        assert_eq!(part2(&s1), 1);
        assert_eq!(part2(&s2), 1);
        assert_eq!(part2(&s3), 0);
        assert_eq!(part2(&s4), 0);
    }
}

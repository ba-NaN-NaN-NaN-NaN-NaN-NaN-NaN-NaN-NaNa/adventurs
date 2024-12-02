use core::num;
use std::collections::{VecDeque, HashMap};
use regex::Regex;

#[allow(dead_code)]
pub fn line_to_intvec(line: &str) -> Vec<i64> {
    let frags:Vec<&str> = line.trim().split(" ").collect();
    let mut toreturn = Vec::new();
    for frag in frags {
        let trimmed = frag.trim();
        if trimmed.len() == 0 {
            continue;
        }
        let as_int: i64 = trimmed.parse().unwrap();
        toreturn.push(as_int);
    }
    toreturn
}


#[allow(dead_code)]
pub fn calc_deltas(input: &Vec<i64>) -> Vec<i64> {
    let mut toreturn = Vec::new();
    for n in 0..(input.len()-1) {
        toreturn.push(input.get(n+1).unwrap()-input.get(n).unwrap());
    }

    toreturn
}


#[allow(dead_code)]
pub struct Lists {
    left: Vec<i64>,
    right: Vec<i64>,
}



#[allow(dead_code)]
impl Lists {
    fn from_lines(cleaned: Vec<String>) -> Lists  {
        let mut left: Vec<i64> = Vec::new();
        let mut right: Vec<i64> = Vec::new();

        for line in cleaned.iter() {
            let parts = line_to_intvec(line);
            left.push(parts[0]);
            right.push(parts[1]);
        }

        left.sort();
        right.sort();

        return Lists {
            left:left,
            right:right,
        }
    }

    fn part_1(&self) -> i64 {
        let mut toreturn = 0;
        for n in 0..self.left.len() {
            let delta = (self.left[n] - self.right[n]).abs();
            toreturn += delta;
        }

        toreturn
    }

    fn part_2(&self) -> i64 {
        let mut toreturn: i64 = 0;
        for num in self.left.iter() {
            let mut count = 0;
            for occurence in self.right.iter() {
                if num == occurence {
                    count += 1;
                }
            }

            toreturn += count * num;
        }
        return toreturn
    }

}


#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut cleaned: Vec<String> = Vec::new();
    for line in input.split("\n") {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }

        cleaned.push(trimmed.to_string());
    }
    let lists: Lists = Lists::from_lines(cleaned);
    return lists.part_1()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut cleaned: Vec<String> = Vec::new();
    for line in input.split("\n") {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }

        cleaned.push(trimmed.to_string());
    }
    let lists: Lists = Lists::from_lines(cleaned);
    return lists.part_2()
}



#[allow(dead_code)]
pub fn tokenize(input: &str) -> VecDeque<String> {
    // Break input into relevant tokens, ignoring formatting chars.
    let mut toreturn = Vec::new();
    let mut worklist = input;
    
    let re_token = Regex::new(r"^([a-zA-Z0-9]+)").unwrap();
    let re_noise = Regex::new(r"^([^a-zA-Z0-9]+)").unwrap();

    while worklist.len() > 0 {
        match re_token.find(worklist) {
            None => {
                let to_discard = re_noise.find(worklist).unwrap().as_str();
                worklist = &worklist[to_discard.len()..];
            }
            Some(tok) => {
                let eat = tok.as_str();
                worklist = &worklist[eat.len()..];
                toreturn.push(eat.to_string());
                // println!("Eating {}", eat);
            }
        }
    }
    return VecDeque::from(toreturn)
    
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d9::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_from_line() {
        let mut ints= line_to_intvec("-5 3");
        assert_eq!(2, ints.len());
        ints = line_to_intvec("-5 344444 -5445");
        assert_eq!(-5445, *ints.get(2).unwrap());
        ints = line_to_intvec("10  13  16  21  30  45  68  ");
    
        let expected_deltas = line_to_intvec("3   3   5   9  15  23");
        assert_eq!(format!("{:?}", expected_deltas), format!("{:?}", calc_deltas(&ints)));
    }
    
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d9::tests --nocapture


        
        {
            let pbuf = input::get_input("2024_d1_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 11); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 1646452); // p1 skarp
        }

        {
            let pbuf = input::get_input("2024_d1_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 31); // p2 sample
        }
        
        {
            let pbuf = input::get_input("2024_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 23609874); // p2 skarp
        }
        
         
    }
}

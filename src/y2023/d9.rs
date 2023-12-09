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
pub struct History {
    table: Vec<Vec<i64>>,
}


#[allow(dead_code)]
pub fn is_all_zeroes(nums: &Vec<i64>) -> bool {
    for num in nums {
        if *num != 0 {
            return false
        }
    }
    return true
}

#[allow(dead_code)]
impl History {
    fn from_line(input: &str) -> History  {
        let mut table: Vec<Vec<i64>> = Vec::new();

        let mut nums = line_to_intvec(input);
        table.push(nums.clone());

        let mut more = true;
        while more {
            let next = calc_deltas(&nums);
            if is_all_zeroes(&next) {
                more = false;
            } else {
                table.push(next.clone());
                nums = next;
            }
        }

        return History { table: table }      
    }

    fn prettyprint(&self) {
        for line in &self.table {
            for elem in line {
                print!("{:^9}", elem);
            }
            println!("");
        }
    }

    fn iterate(&mut self) {
        for line_nr in (0..self.table.len()).rev() {
            let elem_nr = self.table.get(line_nr).unwrap().len();

            let left_nr = self.table.get(line_nr).unwrap().get(elem_nr-1).unwrap();
            let left_down_nr = if line_nr == self.table.len()-1 {
                0
            } else {
                *self.table.get(line_nr+1).unwrap().get(elem_nr-1).unwrap()
            };

            let mut new_line = self.table.get(line_nr).unwrap().clone();
            new_line.push(left_down_nr+left_nr);
            self.table.remove(line_nr);
            self.table.insert(line_nr, new_line);
        }
    }

    fn reverse_it(&mut self) {
        // Part 2: Left insert
        for line_nr in (0..self.table.len()).rev() {
            let right_nr = self.table.get(line_nr).unwrap().get(0).unwrap();
            let right_down_nr = if line_nr == self.table.len()-1 {
                0
            } else {
                *self.table.get(line_nr+1).unwrap().get(0).unwrap()
            };

            let mut new_line = self.table.get(line_nr).unwrap().clone();
            new_line.insert(0, right_nr-right_down_nr);
            self.table.remove(line_nr);
            self.table.insert(line_nr, new_line);
        }
    }

    fn get_p1_prediction(&self) -> i64 {
        let top_line = self.table.get(0).unwrap();
        top_line.last().unwrap().clone()
    }

    fn get_p2_prediction(&self) -> i64 {
        let top_line = self.table.get(0).unwrap();
        top_line.first().unwrap().clone()
    }

    fn iterate_line_nr(&self, line_nr: usize) -> i64 {
        // Iterate history for a line.
        //if line_nr = self.table
        0
    }

}


#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut toreturn = 0;
    for line in input.split("\n") {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }
        let mut history  = History::from_line(trimmed);
        history.iterate();
        // history.prettyprint();
        // println!(" -- ");
        let p1pred = history.get_p1_prediction();
        toreturn += p1pred;
    }
    
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut toreturn = 0;
    for line in input.split("\n") {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }
        let mut history  = History::from_line(trimmed);
        history.reverse_it();
        history.prettyprint();
        println!(" -- ");
        let p1pred = history.get_p2_prediction();
        toreturn += p1pred;
    }
    
    toreturn

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
            let pbuf = input::get_input("2023_d9_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 114); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d9.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 1789635132); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d9_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 2); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d9.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 913); // p2 skarp
        }
         
    }
}

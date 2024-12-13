use regex::Regex;
use std::str::FromStr;
use std::{collections::HashSet, hash::Hash};
use lazy_static::lazy_static;
use std::collections::VecDeque;
use crate::{grids::basic2d::{self, Grid2d, GridDirs8}, input};


#[derive(Clone)]
#[allow(dead_code)]
pub struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

impl Machine {

    #[allow(dead_code)]
    pub fn from_lines(line_a: &String, line_b: &String, line_prize: &String) -> Machine {
        /*
        Button A: X+56, Y+23
Button B: X+16, Y+61
Prize: X=19656, Y=5328
 */
        lazy_static! {
            static ref RE_A: Regex = Regex::new(r"Button A: X.(\d+), Y.(\d+)$").unwrap();
            static ref RE_B: Regex = Regex::new(r"Button B: X.(\d+), Y.(\d+)$").unwrap();
            static ref RE_PRIZE: Regex = Regex::new(r"Prize: X.(\d+), Y.(\d+)$").unwrap();
        }

        let mut toreturn = Machine {
            a_x: 0,
            a_y: 0,
            b_x: 0,
            b_y: 0,
            prize_x: 0,
            prize_y: 0,
        };
    
        match RE_A.captures(&line_a) {
            Some(t) => {
                let left = &t[1];
                let right = &t[2];
                toreturn.a_x = i64::from_str(left).unwrap();
                toreturn.a_y = i64::from_str(right).unwrap();
                
            },
            None => {
                panic!("Failed to match RE_A for '{}'", line_a.to_string());
            }
        };
        match RE_B.captures(&line_b) {
            Some(t) => {
                let left = &t[1];
                let right = &t[2];
                toreturn.b_x = i64::from_str(left).unwrap();
                toreturn.b_y = i64::from_str(right).unwrap();
                
            },
            None => {
                panic!("Failed to match RE_B for '{}'", line_b.to_string());
            }
        };
        match RE_PRIZE.captures(&line_prize) {
            Some(t) => {
                let left = &t[1];
                let right = &t[2];
                toreturn.prize_x = i64::from_str(left).unwrap();
                toreturn.prize_y = i64::from_str(right).unwrap();
                
            },
            None => {
                panic!("Failed to match RE_PRIZE for '{}'", line_prize.to_string());
            }
        };


        toreturn
    
    }

    #[allow(dead_code)]
    pub fn push2win(&self) -> Option<(i64, i64)> {
        /*
            From first example:
        
            8400 == 94*a + 22*b
            5400 == 34*a + 67*b

            k == 34/94 = ay/ax

            Construct equivalency containing only constants and one variable:

            5400 - k*8400 == 34*a - k*94*b + 67*b - k*22*b =[since 34*a - k*94*b == 0]= b(67-k*22)

                5400-k*8400   py - k*px
            b = ----------- = ---------
                67 - k*22     by - k*bx

                px - bx*b
            a = ---------
                    ax

            THEN VERIFY! ESPECIALLY SINCE WE WILL BE ROUNDING! AND NEGATIVE ARE NOT OK!
         */
        let k: f64 = (self.a_y as f64)/(self.a_x as f64);
        let b = (self.prize_y as f64 - (k * self.prize_x as f64)) / (self.b_y as f64 - (k * self.b_x as f64));
        let a: f64 = (self.prize_x as f64 - b * self.b_x as f64) / (self.a_x as f64);
        println!("k = {}, a = {}, b = {}" , k, a, b);

        let a_i64: i64 = a.round() as i64;
        let b_i64: i64 = b.round() as i64;

        if a_i64 < 0 {
            return None
        }

        if b_i64 < 0 {
            return None
        }

        if self.prize_x == self.a_x * a_i64 + self.b_x * b_i64 &&
            self.prize_y == self.a_y * a_i64 + self.b_y * b_i64 {
            Some((a_i64, b_i64))
        } else {
            None
        }        
    }
}


#[allow(dead_code)]
pub struct Day13 {
    machines: Vec<Machine>,
}


impl Day13 {
    pub fn from_lines(content: &str) -> Day13 {
        let mut cleaned_lines: VecDeque<String> = VecDeque::new();

        for line in content.split("\n") {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            cleaned_lines.push_back(trimmed.to_string());           
        }

        let mut machines: Vec<Machine> = Vec::new();

        while cleaned_lines.len() > 0 {
            let line_a = cleaned_lines.pop_front().unwrap();
            let line_b = cleaned_lines.pop_front().unwrap();
            let line_prize = cleaned_lines.pop_front().unwrap();
            machines.push(Machine::from_lines(&line_a, &line_b, &line_prize));
        }

        Day13 { machines }
    }

    pub fn part1(&mut self) -> i64 {
        let mut presses_a: i64 = 0;
        let mut presses_b: i64 = 0;

        for machine in self.machines.iter() {
            if let Some((a, b)) = machine.push2win() {
                presses_a += a;
                presses_b += b;
            }
        }

        presses_a * 3 + presses_b
    }



    pub fn part2(&mut self) -> i64 {
        let mut presses_a: i64 = 0;
        let mut presses_b: i64 = 0;

        for machine in self.machines.iter() {
            let mut distant_machine: Machine = (*machine).clone();
            distant_machine.prize_x += 10000000000000;
            distant_machine.prize_y += 10000000000000;
            if let Some((a, b)) = distant_machine.push2win() {
                presses_a += a;
                presses_b += b;
            }
        }

        presses_a * 3 + presses_b
    }

}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d13 = Day13::from_lines(input);    
    let toreturn = d13.part1();
    // d13.show();
    // println!("{:?}", d13.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d13 = Day13::from_lines(input);    
    let toreturn = d13.part2();
    // d13.show();
    // println!("{:?}", d13.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d13::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_parse() {
        let machine = Machine::from_lines(
        &"Button A: X+56, Y+23".to_owned(),
        &"Button B: X+16, Y+61".to_owned(),
    &"Prize: X=19656, Y=5328".to_owned());
        assert_eq!(machine.a_x, 56); 
        assert_eq!(machine.a_y, 23); 
        assert_eq!(machine.b_x, 16); 
        assert_eq!(machine.b_y, 61); 
        assert_eq!(machine.prize_x, 19656); 
        assert_eq!(machine.prize_y, 5328); 
    }
    

    #[test]
    fn test_p2w() {
        let machine = Machine::from_lines(
        &"Button A: X+94, Y+34".to_owned(),
        &"Button B: X+22, Y+67".to_owned(),
    &"Prize: X=8400, Y=5400".to_owned());
            let expected: Option<(i64, i64)> = Some((80,40));
        assert_eq!(expected, machine.push2win()); 
    }
    

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d13::tests --nocapture
        {
            let pbuf = input::get_input("2024_d13_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 480); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d13.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 31065); // p1 skarp
        }
        if false {
            let pbuf = input::get_input("2024_d13_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, -11); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d13.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 93866170395343); // p2 skarp
        }
    }
}

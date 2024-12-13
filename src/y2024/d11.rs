use core::num;
use std::{collections::HashSet, hash::Hash};
use std::collections::{HashMap, VecDeque};
use crate::{grids::basic2d::{self, Grid2d, GridDirs8}, input};

#[allow(dead_code)]
pub struct Day11 {
    initial: Vec<usize>,
    memoization: HashMap<(usize, usize), usize>,
}

#[allow(dead_code)]
pub fn blink(stones_before: Vec<usize>) -> Vec<usize> {
    let mut toreturn: Vec<usize> = Vec::new();
    for stone in stones_before.iter() {
        if *stone == 0 {
            toreturn.push(1);
            continue;
        }

        let as_str = format!("{}", *stone);
        if as_str.len() % 2 == 0 {
            let len_half: usize = (as_str.len()/2) as usize;
            let (left, right) = as_str.split_at(len_half);
            toreturn.push(left.parse().unwrap());
            toreturn.push(right.parse().unwrap());
            continue;
        }

        toreturn.push(2024 * *stone);
    }
    toreturn
}

impl Day11 {
    pub fn from_content(content: &str) -> Day11 {
        let mut initial: Vec<usize> = input::line_to_intvec(content).iter().map(|i| *i as usize).collect();

        Day11 {
            initial,
            memoization: HashMap::new(),
        }
    }

    pub fn part1_brute(&mut self) -> usize {
        let mut stones: Vec<usize> = self.initial.clone();
        for n in 0..25 {
            stones = blink(stones);
            println!("After blinked {} times, we have {} stones.", n, stones.len());
        }
        
        stones.len()
    }

    pub fn part1(&mut self) -> usize {
        let stones: Vec<usize> = self.initial.clone();
        let mut count: usize = 0;
        for stone in stones.iter() {
            // stones = blink(stones);
            count += self.stones_after_n_blinks(*stone, 25)
            // println!("After blinked {} times, we have {} stones.", n, stones.len());
        }
        
        count
    }

    pub fn part2(&mut self) -> usize {
        let stones: Vec<usize> = self.initial.clone();
        let mut count: usize = 0;
        for stone in stones.iter() {
            // stones = blink(stones);
            count += self.stones_after_n_blinks(*stone, 75)
            // println!("After blinked {} times, we have {} stones.", n, stones.len());
        }
        
        count
    }

    pub fn stones_after_n_blinks(&mut self, starting_stone_label: usize, num_blinks_to_apply: usize) -> usize {
        if let Some(n) = self.memoization.get(&(starting_stone_label, num_blinks_to_apply)) {
            return *n
        }

        if num_blinks_to_apply == 0 {
            self.memoization.insert((starting_stone_label, 0), 1);
            return 1
        }

        let if_blinked_once = blink(Vec::from([starting_stone_label]));
        let mut toreturn = 0;

        for stone in if_blinked_once.iter() {
            toreturn += self.stones_after_n_blinks(*stone, num_blinks_to_apply-1);
        }

        self.memoization.insert((starting_stone_label, num_blinks_to_apply), toreturn);
        toreturn
    }


}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut d11 = Day11::from_content(input);    
    let toreturn = d11.part1();
    // d11.show();
    // println!("{:?}", d11.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut d11 = Day11::from_content(input);    
    let toreturn = d11.part2();
    // d11.show();
    // println!("{:?}", d11.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d11::tests --nocapture

    use super::*;
    use crate::input;
    #[test]
    fn test_blink() {
        let expected_0:Vec<usize> = Vec::from([125, 17]);
        let expected_1:Vec<usize> = Vec::from([253000, 1, 7]);
        let expected_2:Vec<usize> = Vec::from([253, 0, 2024, 14168]);
        let expected_3:Vec<usize> = Vec::from([512072, 1, 20, 24, 28676032]);

        assert_eq!(expected_1, blink(expected_0));
        assert_eq!(expected_2, blink(expected_1));
        assert_eq!(expected_3, blink(expected_2));
    }
    
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d11::tests --nocapture
        {
            let pbuf = input::get_input("2024_d11_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 55312); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d11.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 184927); // p1 skarp
        }
        if false {
            let pbuf = input::get_input("2024_d11_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 1); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d11.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 220357186726677); // p2 skarp
        }
    }
}

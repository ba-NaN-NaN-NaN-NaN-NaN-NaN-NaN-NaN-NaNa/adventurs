use lazy_static::lazy_static;
use std::str::FromStr;
use regex::Regex;

use std::{collections::HashSet, hash::Hash};
use std::collections::VecDeque;
use crate::{grids::basic2d::{self, Grid2d, GridDirs8}, input};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Robot {
    row_pos: i64,
    col_pos: i64,
    row_vel: i64,
    col_vel: i64,
}


#[allow(dead_code)]
pub enum Quadrant {
    NE,
    SE,
    SW,
    NW,
}



impl Robot {
    pub fn from_line(content: &str) -> Robot {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"p=(.+),(.+) v=(.+),(.+)$").unwrap();
        }

        match RE.captures(&content) {
            Some(t) => {               
                Robot {
                    col_pos : i64::from_str(&t[1]).unwrap(),
                    row_pos : i64::from_str(&t[2]).unwrap(),
                    col_vel : i64::from_str(&t[3]).unwrap(),
                    row_vel : i64::from_str(&t[4]).unwrap(),
                }
            },
            None => {
                panic!("Failed to match RE_PRIZE for '{}'", content.to_string());
            }
        }
    }

    #[allow(dead_code)]
    pub fn pos_after_steps(self, step_count:i64, room_width: i64, room_height: i64) -> (i64, i64) {
        let mut row_nr = self.row_pos + step_count * self.row_vel;
        let mut col_nr = self.col_pos + step_count * self.col_vel;

        while row_nr < 0 {
            row_nr += 10000 * room_height
        }

        while col_nr < 0 {
            col_nr += 10000 * room_width
        }

        (row_nr % room_height, col_nr % room_width)
    }
}

#[allow(dead_code)]
pub struct Day14 {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

impl Day14 {
    pub fn from_lines(content: &str) -> Day14 {
        let mut robots: Vec<Robot> = Vec::new();

        for line in content.split("\n") {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            robots.push(Robot::from_line(trimmed));
        }

        let width = if robots.len() > 20  { 101 } else { 11 };
        let height = if robots.len() > 20  { 103 } else { 7 };

        Day14 {
            robots,
            width,
            height,
        }
    }

    pub fn quadrant_for(&self, row_nr: i64, col_nr: i64) -> Option<Quadrant> {
        if row_nr < 0 || row_nr >= self.height {
            panic!("kfjdh")
        }
        if col_nr < 0 || col_nr >= self.width {
            panic!("kfjxxdh")
        }

        let center_col = (self.width-1)/2.0 as i64;
        let center_row = (self.height-1)/2.0 as i64;

        if row_nr == center_row || col_nr == center_col {
            return None
        }

        if row_nr < center_row && col_nr < center_col {
            return Some(Quadrant::NW)
        }

        if row_nr > center_row && col_nr < center_col {
            return Some(Quadrant::SW)
        }

        if row_nr < center_row && col_nr > center_col {
            return Some(Quadrant::NE)
        }

        if row_nr > center_row && col_nr > center_col {
            return Some(Quadrant::SE)
        }


        None
    }

    pub fn part1(&mut self) -> i64 {   
        let mut count_se: i64 = 0;
        let mut count_sw: i64 = 0;
        let mut count_ne: i64 = 0;
        let mut count_nw: i64 = 0;
        
        for robot in self.robots.iter() {
            let(row_nr, col_nr) = robot.pos_after_steps(100, self.width, self.height);

            match self.quadrant_for(row_nr, col_nr) {
                Some(Quadrant::NE) => { count_ne += 1},
                Some(Quadrant::SE) => { count_se += 1},
                Some(Quadrant::NW) => { count_nw += 1},
                Some(Quadrant::SW) => { count_sw += 1},
                None => {},
            }
        }
        count_ne * count_nw * count_se * count_sw
    }



    pub fn part2(&mut self) -> i64 {
        0
    }

    #[allow(dead_code)]
    pub fn show(&self, timestamp: i64) {
        let mut lines: Vec<Vec<char>> = Vec::new();
        for _ in 0..self.height {
            let mut line: Vec<char> = Vec::new();
            for _ in 0..self.width {
                line.push('.');
            }
            lines.push(line);
        }

        for robot in self.robots.iter() {
            let (row_nr, col_nr) = robot.pos_after_steps(timestamp, self.width, self.height);
            lines[row_nr as usize][col_nr as usize] = 'X';
        }

        for line in lines.iter() {
            let formatted: String = line.iter().map( |i| format!("{}", i)).collect();
            println!("{}", formatted);
        }
    }



}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d14 = Day14::from_lines(input);
    let toreturn = d14.part1();
    // d14.show();
    // println!("{:?}", d14.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d14 = Day14::from_lines(input);    
    let toreturn = d14.part2();
    // d14.show();
    // println!("{:?}", d14.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d14::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_robot_steps() {
        let robot = Robot::from_line("p=2,4 v=2,-3");
        // (row_nr, col_nr)
        assert_eq!((4, 2), robot.pos_after_steps(0, 11, 7));
        assert_eq!((1, 4), robot.pos_after_steps(1, 11, 7));
        assert_eq!((5, 6), robot.pos_after_steps(2, 11, 7));
        assert_eq!((2, 8), robot.pos_after_steps(3, 11, 7));
        assert_eq!((6, 10), robot.pos_after_steps(4, 11, 7));
        assert_eq!((3, 1), robot.pos_after_steps(5, 11, 7));
    }

    #[test]
    fn test_p2_sample() {
        let pbuf = input::get_input("2024_d14_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let d14 = Day14::from_lines(&content);

        for n in 0..6 {
            println!("Day 14 sample after {} steps", n);
            d14.show(n);
        }

        assert_eq!("fd", "gfvd")

    }

    #[test]
    fn test_p2_skarp() {
        let pbuf = input::get_input("2024_d14.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let d14 = Day14::from_lines(&content);

        // Visual inspection shows that
        // horiz_anomaly when steps = 2 + n*103
        // vert_anomaly when steps = 23 + n*101


        // a.k.a 
        // steps % 101 == 23
        // steps % 103 == 2
        // So C.R.T. But! Brute force instead.

        /*
        for n in 6000..66000 {
            println!("Day 14 skarp after {} steps", n);
            d14.show(n);
        }
         */
        for steps in 1..(101*103*3) {
            if steps % 101 == 23 && steps % 103 == 2 {
                println!("Day 14 skarp after {} steps", steps);
                d14.show(steps);
            }
        }

        assert_eq!("fd", "gfvd") // Make output not get captured.

    }


    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d14::tests --nocapture
        {
            let pbuf = input::get_input("2024_d14_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 12); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d14.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 215476074); // p1 skarp
        }
        // Part 2 done via seperate manual inspection

        /* 
        {
            let pbuf = input::get_input("2024_d14_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 1); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d14.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 1); // p2 skarp
        }
        */
    }
}

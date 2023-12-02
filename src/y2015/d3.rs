use std::collections::{VecDeque, HashSet};
use std::str::from_utf8;

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let visitcount = do_deliveries(input);
    return visitcount.len().try_into().unwrap()
}

pub fn do_deliveries(input: &str) -> HashSet::<String> {
    let instr = input.trim().to_string();
    let mut visitcount = HashSet::<String>::new();
    visitcount.insert("0, 0".to_string());
    let mut curr_x = 0;
    let mut curr_y = 0;

    for ch in instr.chars() {
        match ch.to_string().as_str() {
            ">" => {
                curr_x += 1;
            }   
            "<" => {
                curr_x -= 1;
            }
            "v" => {
                curr_y += 1;
            }
            "^" => {
                curr_y -= 1;
            }            
            _ => { 
                println!("bad input char '{}'", ch);
                panic!("bad input char");
            }
        }
        let coord = format!("{}, {}", curr_x, curr_y);
        match visitcount.get(&coord) {
            Some(_) => { }
            None => { visitcount.insert(coord); }
        }
    }
    return visitcount
}
#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {

    let mut santa_steps  = Vec::<u8>::new();
    let mut robot_steps = Vec::<u8>::new();
    let mut worklist = VecDeque::<&u8>::from_iter(input.as_bytes());

    while worklist.len() > 0 {
        santa_steps.push(worklist.pop_front().unwrap().to_owned());
        if worklist.len() > 0 {
            robot_steps.push(worklist.pop_front().unwrap().to_owned());
        }
    }

    let santa_str = from_utf8(santa_steps.as_slice()).unwrap();
    let mut visited = do_deliveries(santa_str);

    let robot_steps = from_utf8(robot_steps.as_slice()).unwrap();
    visited.extend(do_deliveries(robot_steps));

    let toreturn = visited.len().try_into().unwrap();
    toreturn
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_d3() {
        // cargo test --package adventurs --bin adventurs -- y2015::d3::tests --nocapture
        let pbuf = input::get_input("2015_d3.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();

        {
            // Part 1
            assert_eq!(2, part1(">"));
            assert_eq!(4, part1("^>v<"));
            assert_eq!(2, part1("^v^v^v^v^v"));

            let actual = part1(&content);
            assert_eq!(actual, 2592); // p1
        }
         {
            // Part 2
            assert_eq!(3, part2("^v"));
            assert_eq!(3, part2("^>v<"));
            assert_eq!(11, part2("^v^v^v^v^v"));

            let actual = part2(&content);
            assert_eq!(actual, 2360); // p1
        }
    }
}

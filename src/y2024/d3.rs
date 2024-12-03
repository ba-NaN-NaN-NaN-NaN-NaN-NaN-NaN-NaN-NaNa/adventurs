use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;


#[allow(dead_code)]
pub fn split_after_rparen(line: &str) -> Vec<String> {
    let mut to_eat = line;
    let mut eaten : Vec<String> = Vec::new();
    while let Some((left, right)) = to_eat.split_once(")") {
        eaten.push(left.to_string() + ")");
        to_eat = right
    }

    eaten.push(to_eat.to_string());
    eaten
}

#[allow(dead_code)]
pub fn maybe_eval_mul_expr(expr: &String) -> Option<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)$").unwrap();
    }

    match RE.captures(expr) {
        Some(t) => {
            let left = &t[1];
            let right = &t[2];
            let x = i64::from_str(left).unwrap();
            let y = i64::from_str(right).unwrap();

            Some(x*y)
        },
        None => None
    } 
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let frags = split_after_rparen(input);
    let mut tally = 0;
    for frag in frags.iter() {
        if let Some(x) = maybe_eval_mul_expr(frag) {
            tally += x;
        }
    }
    tally
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut do_muls = true;
    let frags = split_after_rparen(input);
    let mut tally = 0;
    for frag in frags.iter() {
        // println!("Got frag {}", frag);
        if frag.ends_with("do()") {
            do_muls = true;
            // println!("Enabling");
        } else if frag.ends_with("don't()") {
            do_muls = false;
            // println!("Disabling");
        } else if let Some(x) = maybe_eval_mul_expr(frag) {
            // println!("Tally mul {}? {}!", x, do_muls);
            if do_muls {
                tally += x;
            }
        }
    }
    tally
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d3::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_split() {
        let res1 = split_after_rparen("a)b");
        assert_eq!(2, res1.len());
        assert_eq!("a)", res1[0]);
        assert_eq!("b", res1[1]);


        let res1 = split_after_rparen("a)))b))x");
        assert_eq!(6, res1.len());
        assert_eq!("a)", res1[0]);
        assert_eq!(")", res1[1]);
        assert_eq!("x", res1[5]);
    }

    #[test]
    fn test_maybe_eval_mul_expr() {
        assert_eq!(1, maybe_eval_mul_expr(&"mul(1,1)".to_owned()).unwrap());
        assert_eq!(25, maybe_eval_mul_expr(&"xxxmul(5,5)".to_owned()).unwrap());
        assert_eq!(None, maybe_eval_mul_expr(&"xxxmu(5,5)".to_owned()))
    }
    
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d3::tests --nocapture
        {
            let pbuf = input::get_input("2024_d3_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 161); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d3.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 166630675); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d3_sample_p2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 48); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d3.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 93465710); // p2 skarp
        }
    }
}

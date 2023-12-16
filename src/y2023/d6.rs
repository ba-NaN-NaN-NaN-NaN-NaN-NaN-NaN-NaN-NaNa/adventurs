use std::collections::VecDeque;
use regex::Regex;


#[derive(Debug, Clone, Copy)]
pub struct RaceResult {
    distance: i64,
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

#[allow(dead_code)]
pub fn results_for_duration(race_duration: i64) -> Vec<RaceResult> {
    let mut toreturn = Vec::new();

    for wait_time in 0..race_duration+1 {
        let speed = wait_time;
        let moving_time = race_duration - wait_time;
        let distance = speed * moving_time;
        toreturn.push(RaceResult{
            distance: distance,
        });
    }

    toreturn
}

#[allow(dead_code)]
pub fn part1(content: &str) -> i64 {
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let mut durations = tokenize(lines[0]);
    let mut records = tokenize(lines[1]);

    durations.pop_front();
    records.pop_front();

    let mut toreturn = 1;

    while durations.len() > 0 {
        let race_duration = durations.pop_front().unwrap().parse::<i64>().unwrap();
        let race_record = records.pop_front().unwrap().parse::<i64>().unwrap();
        let results = results_for_duration(race_duration);
        let mut waycount = 0;
        for res in &results {
            if res.distance > race_record {
                // println!("Waiting {} in race {} gives distance {}, which breaks the record {}!", res.wait_time, race_duration, res.distance, race_record);
                waycount += 1;
            }
        }

        // println!("Results = {:?}", &results.clone());
        // println!("Found {} ways to beat the record {} for the duration {}", waycount, race_record, race_duration);

        toreturn *= waycount;
    }

    return toreturn
}

#[allow(dead_code)]
fn part2(content: &str) -> i64 {
    return part1(&content.replace(&" ", &""));
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d6::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d6::tests --nocapture


        {
            let pbuf = input::get_input("2023_d6_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 288); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d6.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 741000); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d6_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 71503); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d6.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 38220708); // p2 skarp
        }
    }
}

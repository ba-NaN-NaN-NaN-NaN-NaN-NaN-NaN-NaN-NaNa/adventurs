use std::collections::VecDeque;
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
    // println!("line_to_intvec('{}')-> {:?}", line, toreturn);
    toreturn
}


#[allow(dead_code)]
pub struct Report {
    levels: Vec<i64>,
    first_err_increasing: Option<usize>,
    first_err_decreasing: Option<usize>,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Safety {
    Safe,
    Unsafe
}

fn find_first_err_increasing(levels: &Vec<i64> ) -> Option<usize> {
    for n in 0..levels.len()-1 {
        let delta = levels[n+1] - levels[n];
        if delta > 3 {
            return Some(n)
        } else if delta < 1 {
            return Some(n)
        }
    }

    return None
}

fn find_first_err_decreasing(levels: &Vec<i64> ) -> Option<usize> {
    for n in 0..levels.len()-1 {
        let delta = levels[n+1] - levels[n];
        if delta < -3 {
            return Some(n)
        } else if delta > -1 {
            return Some(n)
        }
    }

    return None
}

#[allow(dead_code)]
impl Report {
    fn from_line(line: &String) -> Report  {
        let levels = line_to_intvec(line.as_str());

        let found_first_increasing = find_first_err_increasing(&levels);
        let found_first_decreasing = find_first_err_decreasing(&levels);
        let toreturn = Report {
            levels,
            first_err_increasing: found_first_increasing,
            first_err_decreasing: found_first_decreasing,
        };

        toreturn
    }

    fn get_safetyness_1(&self) -> Safety {
        if self.first_err_decreasing == None || self.first_err_increasing == None {
            Safety::Safe
        } else {
            Safety::Unsafe
        }
    }

    fn get_safetyness_2(&self) -> Safety {
        if self.first_err_decreasing == None || self.first_err_increasing == None {
            return Safety::Safe
        }

        for n in 0..self.levels.len() {
            let mut levels_without_one = self.levels.clone();
            levels_without_one.remove(n);
            if find_first_err_increasing(&levels_without_one) == None || find_first_err_decreasing(&levels_without_one) == None {
                return Safety::Safe
            }
        }
        return Safety::Unsafe
    }


}

#[allow(dead_code)]
pub struct Reports {
    reports: Vec<Report>,
}

#[allow(dead_code)]
impl Reports {
    fn from_lines(cleaned: Vec<String>) -> Reports  {
        let mut reports: Vec<Report> = Vec::new();

        for line in cleaned.iter() {
            let report = Report::from_line(line);
            reports.push(report);
        }

        return Reports {
            reports:reports,
        }
    }

    fn part_1(&self) -> i64 {
        let mut toreturn = 0;
        for report in self.reports.iter() {
            if report.get_safetyness_1() == Safety::Safe {
                toreturn += 1
            }
        }

        toreturn
    }

    fn part_2(&self) -> i64 {
        let mut toreturn: i64 = 0;
        for report in self.reports.iter() {
            if report.get_safetyness_2() == Safety::Safe {
                toreturn += 1
            }
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
    let lists: Reports = Reports::from_lines(cleaned);
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
    let lists: Reports = Reports::from_lines(cleaned);
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
    fn test_record() {
        let record = Report::from_line(&"7 6 4 2 1".to_owned());
        assert_eq!(record.get_safetyness_1(), Safety::Safe);

    }
    
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d9::tests --nocapture
        
        {
            let pbuf = input::get_input("2024_d2_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 2); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 686); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d2_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 4); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 717); // p2 skarp
        }
    }
}

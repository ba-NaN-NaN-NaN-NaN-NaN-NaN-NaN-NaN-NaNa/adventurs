use regex::Regex;
use std::collections::HashSet;

#[allow(dead_code)]
pub struct Walker {
    direction: String,
    pos_x: i64,
    pos_y: i64,
    visited: HashSet<String>,

    p2ans: Option<i64>,
}

#[allow(dead_code)]
impl Walker {
    pub fn new() -> Walker {
        let mut toreturn = Walker { 
            direction: "N".to_string(), 
            pos_x: 0, 
            pos_y: 0,
            visited:HashSet::<String>::new(),
            p2ans: None,
        };
        toreturn.visited.insert("0,0".to_string());
        toreturn
    }

    pub fn ingest(&mut self, input: &str) {
        let re_token = Regex::new(r"^(?P<rot>[RL])(?P<count>[0-9]+)").unwrap();
        let re_noise = Regex::new(r"^([^RL0-9])").unwrap();

        let mut to_ingest = input;
    
        while to_ingest.len() > 0 {
            match re_token.captures(to_ingest) {
                None => {
                    let to_discard = re_noise.find(to_ingest).unwrap().as_str();
                    // println!("to_discard {}", to_discard);
                    to_ingest = &to_ingest[to_discard.len()..];
                }
                Some(tok) => {
                    let rot = &tok["rot"];
                    let count = &tok["count"];
                    self.turn(rot);
                    self.forward(count.parse::<i64>().unwrap() );
                    let n = rot.len()+count.len();
                    to_ingest = &to_ingest[n..];
                    // let eat = tok.as_str();
                    // to_ingest = &to_ingest[eat.len()..];
                    // toreturn.push(eat.to_string());
                    // println!("Eating {}{}", rot, count);
                }
            }
        }
    }

    pub fn forward(&mut self, count: i64) {
        match self.direction.as_str() {
            // One step at a time since we want to register each
            // position visited for part 2.
            "N" => { self.pos_y -= 1 },
            "E" => { self.pos_x += 1 },
            "S" => { self.pos_y += 1 },
            "W" => { self.pos_x -= 1 },
            _ => panic!("fldkjh")
        }

        let current_coord = format!("{},{}", self.pos_x, self.pos_y);
        match self.p2ans {
            None => { 
                if self.visited.contains(&current_coord) {
                    let current_distance = self.distance();
                    // println!("Found part 2 answer! At {} with distance {}. Historical coords is {:?}", current_coord, current_distance, self.visited);
                    self.p2ans = Some(current_distance);
                };
            },
            _ => {},
        }
        self.visited.insert(current_coord);

        if count > 1 {
            self.forward(count-1)
        }
    }

    pub fn turn(&mut self, rot: &str) {
        // Turn.
        let new_direction = match rot {
            "R" => {
                match self.direction.as_str() {
                    "N" => {  "E" },
                    "E" => {  "S" },
                    "S" => {  "W" },
                    "W" => {  "N" },
                    _ => panic!("fldkjh")
                }
            }
            "L" => {
                match self.direction.as_str() {
                    "N" => {  "W" },
                    "E" => {  "N" },
                    "S" => {  "E" },
                    "W" => {  "S" },
                    _ => panic!("fldkjh")
                }
            }
            _ => panic!("fldkjh")
        };

        self.direction = new_direction.to_string()
    }

    pub fn distance(&self) -> i64 {
        return self.pos_x.abs() + self.pos_y.abs()
    }    
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2016::d1::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn simple_walker() {
        {
            let content ="R13";
            let mut walker = Walker::new();
            walker.ingest(content);
            assert_eq!(walker.distance(), 13); // p1 sample

            let content ="R2, L3";
            let mut walker = Walker::new();
            walker.ingest(content);
            assert_eq!(walker.distance(), 5); // p1 sample

            let content ="R1,R1,R1,R1";
            let mut walker = Walker::new();
            walker.ingest(content);
            assert_eq!(walker.distance(), 0); // p1 sample
        }
    }

    #[test]
    fn test_d1_full() {
        {
            let pbuf = input::get_input("2016_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut walker = Walker::new();
            walker.ingest(content.as_str());
            assert!(walker.distance() < 503); // 503 too high
            assert_eq!(walker.distance(), 273); // p1 skarp
        }

        {
            let pbuf = input::get_input("2016_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut walker = Walker::new();
            walker.ingest(content.as_str());
            println!("Part 2 answer {}", walker.p2ans.unwrap());
            assert!(walker.p2ans.unwrap() < 503); // 503 too high
            assert!(walker.p2ans.unwrap() < 257); // 257 too high
            assert_eq!(walker.p2ans.unwrap(), 115); // p2 skarp
        }
    }
}

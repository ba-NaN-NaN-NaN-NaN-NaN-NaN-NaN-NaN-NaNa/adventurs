use std::collections::{VecDeque, HashSet, HashMap};
use std::cmp::Ordering;
use regex::Regex;



#[allow(dead_code)]
pub struct Navigation {
    directions: Vec<Direction>,
    locations: HashSet<String>,
    lefts: HashMap<String, String>,
    rights: HashMap<String, String>,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Direction {
    Left,
    Right,
}

impl Navigation {
    #[allow(dead_code)]
    pub fn left_from(&self, node:&str) -> String {
        self.lefts.get(&node.to_string()).unwrap().to_string()
    }

    #[allow(dead_code)]
    pub fn right_from(&self, node:&str) -> String {
        self.rights.get(&node.to_string()).unwrap().to_string()
    }

    #[allow(dead_code)]
    pub fn dir_at_step(&self, step_nr: usize) -> Direction {
        *self.directions.get(step_nr % self.directions.len()).unwrap()
    }

    #[allow(dead_code)]
    pub fn next_node(&self, node:String, step_nr: usize) -> String {
        let next_dir =  self.dir_at_step(step_nr);
        match next_dir {
            Direction::Left => self.lefts.get(&node.to_string()).unwrap().to_string(),
            Direction::Right => self.rights.get(&node.to_string()).unwrap().to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn next_nodes(&self, nodes:Vec<String>, step_nr: usize) -> Vec<String> {
        let mut toreturn: Vec<String> = Vec::new();
        for node_nr in 0..nodes.len() {
            let node = nodes.get(node_nr).unwrap();
            let next_dir =  self.dir_at_step(step_nr);
            let next_node = match next_dir {
                Direction::Left => self.lefts.get(&node.to_string()).unwrap().to_string(),
                Direction::Right => self.rights.get(&node.to_string()).unwrap().to_string(),
            };
            if next_node.ends_with("Z") {
                println!("Node nr {} at end location {} after step {}", node_nr, next_node, step_nr+1);
            }
            toreturn.push(next_node);
        }
        toreturn
    }



            /*
        let mut next: Vec<String> = Vec::new();
        for loc in locations {
            let next_node = &nav.next_node(loc, step_nr).clone();
            next.push(next_node.clone());
        }
        
        locations = next;
        */

}

#[allow(dead_code)]
pub fn tokenize(input: &str) -> Navigation {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let instructions = lines[0].trim().chars();

    let mut directions: Vec<Direction> = Vec::new();
    let mut lefts: HashMap<String, String> = HashMap::new();
    let mut rights: HashMap<String, String> = HashMap::new();
    let mut locations: HashSet<String> = HashSet::new();

    for instr in instructions {
        match instr {
            
                'L' => {
                    directions.push(Direction::Left);
                },
                'R' => {
                    directions.push(Direction::Right);
                },
                _ => { 
                    println!("Got step instruction {:?}, panicing", instr);
                    panic!("fjdgh"); 
                }
            
        }
    }

    let re = Regex::new(r"^(?P<origin>[A-Z0-9]{3}) = .(?P<left>[A-Z0-9]{3}), (?P<right>[A-Z0-9]{3}).$").unwrap();
    for line_nr in 2..lines.len() {
        let line = lines.get(line_nr).unwrap().trim();
        let res = match re.captures(line) {
            Some(stuff) => stuff,
            None => { 
                println!("Could not regexp match line nr '{}' of content '{:?}' : {}", line_nr, lines, line);
                panic!("gfduhjgkufdsh")
            },
        };
        let origin = &res["origin"];
        let left = &res["left"];
        let right = &res["right"];
        lefts.insert(origin.to_string(), left.to_string());
        rights.insert(origin.to_string(), right.to_string());

        locations.insert(origin.to_string());
        locations.insert(left.to_string());
        locations.insert(right.to_string());
    }
    Navigation { 
        directions: directions, 
        locations:locations,
        lefts: lefts, 
        rights: rights,
    }
}


#[allow(dead_code)]
pub fn part1(content: &str) -> i64 {
    let mut nav = tokenize(content);
    
    let mut location = "AAA".to_string();
    let mut destination = "ZZZ".to_string();
    let mut steps_taken = 0;
    while location != destination {
        // let next_dir = nav.dir_at_step(steps_taken);
        location = nav.next_node(location, steps_taken);
        steps_taken += 1;
    };
    steps_taken.try_into().unwrap()
}

#[allow(dead_code)]
pub fn all_locations_are_ends(locations: &Vec<String>) -> bool {
    for loc in locations {
        if !loc.ends_with("Z") {
            return false
        }
    }
    return true
}

#[allow(dead_code)]
fn part2(content: &str) -> i64 {
    let nav = tokenize(content);
    let mut starting_locations: Vec<String> = Vec::new();

    for loc in &nav.locations {
        if loc.ends_with("A") {
            starting_locations.push(loc.to_string());
        }
    }

    println!("Got starting_locations: {:?}", starting_locations);

    let mut locations = starting_locations.clone();
    let mut step_nr = 0;
    while !all_locations_are_ends(&locations) {
        /*
        let mut next: Vec<String> = Vec::new();
        for loc in locations {
            let next_node = &nav.next_node(loc, step_nr).clone();
            next.push(next_node.clone());
        }
        
        locations = next;
        */
        locations = nav.next_nodes(locations, step_nr);
        step_nr += 1;

        if step_nr > 4000 {
            println!("See comment near test");
        }
        // println!("After step nr {}, locations are {:?}", step_nr, locations);
    }
    
    step_nr.try_into().unwrap()
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d8::tests --nocapture

    use super::*;
    use crate::input;


    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d8::tests --nocapture


        {
            let pbuf = input::get_input("2023_d8_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 2); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d8_sample_2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 6); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d8.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 15517); // p1 skarp
        }
        {
            let pbuf = input::get_input("2023_d8_sample_3.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 6); // p1 sample
        }

        {
            let pbuf = input::get_input("2023_d8.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 14935034899483); // p1 sample
        }

/*


Node nr 4 at end location NTZ after step 12361
Node nr 3 at end location KDZ after step 13939
Node nr 1 at end location ZZZ after step 15517
Node nr 5 at end location XQZ after step 17621
Node nr 0 at end location LKZ after step 19199
Node nr 2 at end location XBZ after step 20777


Incomplete code: Inspect all steps for repeating factors, then multiply them together to get least common denominator.

$ factor 12361
12361: 47 263
$ factor 13939
13939: 53 263
$ factor 15517
15517: 59 263
$ factor 17621
17621: 67 263
$ factor 19199
19199: 73 263
$ factor 20777
20777: 79 263

>>> 47*53*59*67*73*79*263
14935034899483




*/
        

    }
}

use core::panic;
use std::collections::VecDeque;
use regex::Regex;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Almanac {
    seed_locations: Vec<usize>,
    mappings: Vec<Mapping>,
}

impl Almanac {

    #[allow(dead_code)]
    pub fn from_content(content: &str) -> Almanac {
        let mut seed_locations: Vec<usize> = Vec::new();
        let mut mappings:Vec<Mapping> = Vec::new();

        let segments:Vec<&str> = content.split("\n\n").collect();

        println!("Got {} segments from content", segments.len());

        let seed_input_row = segments.get(0).unwrap();
        let seed_frags:Vec<&str> = seed_input_row.trim().split(" ").collect();
        for frag_nr in 1..seed_frags.len() {
            let frag = seed_frags.get(frag_nr).unwrap();
            seed_locations.push(frag.parse::<usize>().unwrap());
        }

        for n in 1..segments.len() {
            let seg = segments.get(n).unwrap().trim();
            if seg.len() == 0 {
                continue;
            }
            let mapping = Mapping::from_content(seg);
            mappings.push(mapping);
        }

        let toreturn = Almanac { 
            seed_locations: seed_locations, 
            mappings: mappings 
        };
        println!("Got almanac: {:?}", toreturn);
        toreturn
    }


    #[allow(dead_code)]
    pub fn explode_seed_ranges(&mut self) {
        let mut new_seed_locations:Vec<usize> = Vec::new();

        let mut to_explode = VecDeque::from(self.seed_locations.clone());

        while to_explode.len() > 0 {
            let start = to_explode.pop_front().unwrap();
            let span = to_explode.pop_front().unwrap();

            for n in start..(start+span) {
                new_seed_locations.push(n);
            }
        }      
        
        if new_seed_locations.len() == 0 {
            println!("Something wrong, zero exploded seed locations from {:?}", self.seed_locations);
            panic!("jkflhgds")
        }
        self.seed_locations = new_seed_locations;
    }

    #[allow(dead_code)]
    pub fn convert_num(&self, loc: &Location) -> Location {
        let mapping = self.mappings.iter().find( |m| m.src_cat == loc.category ).unwrap();
        // println!("Will convert_num for location {:?} using mapping {:?}", loc, mapping);
        let converted = mapping.convert(loc);
        converted
    }

    pub fn iterate_convert_to_location(&self, loc: &Location) -> Location {
        let mut toreturn = loc.clone();
        while toreturn.category != "location" {
            toreturn = self.convert_num(&toreturn);
        }
        toreturn
    }


    #[allow(dead_code)]
    pub fn find_closest_loc(&self) -> usize {
        let mut candidate = self.iterate_convert_to_location(&Location { 
            category: "seed".to_string(), 
            position: self.seed_locations.get(0).unwrap().clone(), 
        });
        let mut toreturn = candidate;

        for seed_loc in &self.seed_locations {
            candidate = self.iterate_convert_to_location(&Location { 
                category: "seed".to_string(), 
                position: seed_loc.clone(), 
            });
            if candidate.position < toreturn.position {
                toreturn = candidate
            }
        }
        toreturn.position.try_into().unwrap()
    }

}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Mapping {
    src_cat: String,
    dst_cat: String,
    ranges: Vec<Range>,
}


impl Mapping {
    #[allow(dead_code)]
    pub fn convert(&self, loc: &Location) -> Location {
        for range in &self.ranges {
            match range.convert(loc.position) {
                Some(n) => {
                    return Location{
                        category: self.dst_cat.clone(),
                        position: n,
                    }
                },
                None => {},
            }
        }
        // println!("Cannot convert location {:?}", loc);
        // panic!("kfdh")

        return Location{
            category: self.dst_cat.clone(),
            position: loc.position,
        }
    }

    pub fn from_content(content: &str) -> Mapping {
        

        // soil-to-fertilizer map:
        // 0 15 37
        // 37 52 2
        // 39 0 15      

        let lines:Vec<&str> = content.trim().split("\n").collect();
        if lines.len() < 2 {
            println!("Can not build content from lines {:?}", lines);
            panic!("Can not build mapping content");
        }

        let cat_line = lines.get(0).unwrap();

        let mut ranges: Vec<Range> = Vec::new();
        for line_nr in 1..lines.len() {
            let line = lines.get(line_nr).unwrap().trim();
            if line.len() == 0 {
                continue;
            }
            ranges.push(Range::from_line(line));
        }
        
        // light-to-temperature map:
        let re_cat = Regex::new(r"^(?P<src>[a-zA-Z0-9]+)-to-(?P<dst>[a-zA-Z0-9]+) map:$").unwrap();

        let res_candidate = re_cat.captures(&cat_line.trim());

        let cat_res = match res_candidate {
            Some(res) => {res},
            None=> { 
                println!("Could not regexp category line from {}, which should be first line of {:?}", cat_line, lines);
                panic!("Bad cat regexp match")
            },
        };

        Mapping { 
            src_cat: cat_res["src"].to_string(), 
            dst_cat: cat_res["dst"].to_string(), 
            ranges: ranges 
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Location {
    category: String,
    position: usize, // Absolute
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Range {
    dst_start: usize,
    src_start: usize,
    length: usize,
}

impl Range {
    #[allow(dead_code)]
    pub fn convert(&self, position: usize) -> Option<usize> {
        if self.src_start <= position && position <= self.src_start + self.length {
            let offset = position - self.src_start;
            return Some(self.dst_start + offset)
        }
        None
    }

    #[allow(dead_code)]
    pub fn from_line(line: &str) -> Range {
        let frags: Vec<&str> = line.trim().split(" ").collect();
        Range { 
            dst_start: frags.get(0).unwrap().parse::<usize>().unwrap(), 
            src_start: frags.get(1).unwrap().parse::<usize>().unwrap(), 
            length: frags.get(2).unwrap().parse::<usize>().unwrap(), 
        }
    }
    
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
pub fn part1(content: &str) -> usize {
    let almanac = Almanac::from_content(content);
    // let mut toreturn = 0;
    let closest = almanac.find_closest_loc();
    closest
}

#[allow(dead_code)]
fn part2(content: &str) -> usize {
    let mut almanac = Almanac::from_content(content);
    almanac.explode_seed_ranges();
    let closest = almanac.find_closest_loc();
    closest

}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d5::tests --nocapture

    use super::*;
    use crate::input;
    #[test]
    fn test_convert_num() {
        let pbuf = input::get_input("2023_d5_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let almanac = Almanac::from_content(&content);

        let loc = almanac.convert_num(&Location { category: "seed".to_string(), position: 50 });
        assert_eq!(loc.category, "soil"); // p1 sample
        assert_eq!(loc.position, 52); // p1 sample

        let loc = almanac.convert_num(&Location { category: "water".to_string(), position: 20 });
        assert_eq!(loc.category, "light"); // p1 sample
        assert_eq!(loc.position, 90); // p1 sample

        let loc = almanac.convert_num(&Location { category: "seed".to_string(), position: 55 });
        assert_eq!(loc.category, "soil"); // p1 sample
        assert_eq!(loc.position, 57); // p1 sample

        let loc = almanac.convert_num(&Location { category: "soil".to_string(), position: 57 });
        assert_eq!(loc.category, "fertilizer"); // p1 sample
        assert_eq!(loc.position, 57); // p1 sample

        let loc = almanac.convert_num(&Location { category: "fertilizer".to_string(), position: 57 });
        assert_eq!(loc.category, "water"); // p1 sample
        assert_eq!(loc.position, 53); // p1 sample

        let loc = almanac.convert_num(&Location { category: "water".to_string(), position: 53 });
        assert_eq!(loc.category, "light"); // p1 sample
        assert_eq!(loc.position, 46); // p1 sample

        let loc = almanac.convert_num(&Location { category: "light".to_string(), position: 46 });
        assert_eq!(loc.category, "temperature"); // p1 sample
        assert_eq!(loc.position, 82); // p1 sample

        let loc = almanac.convert_num(&Location { category: "temperature".to_string(), position: 82 });
        assert_eq!(loc.category, "humidity"); // p1 sample
        assert_eq!(loc.position, 82); // p1 sample

        let loc = almanac.convert_num(&Location { category: "humidity".to_string(), position: 82 });
        assert_eq!(loc.category, "location"); // p1 sample
        assert_eq!(loc.position, 86); // p1 sample
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d5::tests --nocapture


        {
            let pbuf = input::get_input("2023_d5_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 35); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d5.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 157211394); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d5_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 46); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d5.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert!(actual < 881943038); // p2 skarp
            assert_eq!(actual, 50855035); // p2 skarp
        }
    }
}

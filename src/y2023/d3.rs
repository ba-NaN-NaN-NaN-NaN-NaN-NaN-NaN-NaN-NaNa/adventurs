use std::collections::VecDeque;
use regex::Regex;

#[allow(dead_code)]
pub struct Game {
    game_nr: i64,
    min_red: i64,
    min_green: i64,
    min_blue: i64,
}


pub struct PartField {
     content: Vec<Vec<char>>,
     reach: Vec<Vec<bool>>,
}

impl PartField {
    pub fn from_content(input: &str) -> PartField {
        let mut lines: Vec<&str> = input.split("\n").collect();
        lines = lines.iter().filter(|line| line.trim().len() > 0).cloned().collect();
        lines = lines.iter().map(|line| line.trim()).collect();
        // println!("Read {} lines in part 1.", lines.len());

        let mut content: Vec<Vec<char>> = Vec::new();
        let mut reachability: Vec<Vec<bool>> = Vec::new();
        for rn in 0..lines.len() {
            let chars:Vec<char> = lines[rn].chars().collect();
            let line_len = lines[rn].len();

            let mut chars_row: Vec<char> = Vec::new();
            let mut reach_row: Vec<bool> = Vec::new();

            for cn in 0..line_len {
                chars_row.push(chars[cn]);
                reach_row.push(false);
                
            }
            content.push(chars_row);
            reachability.push(reach_row);
        }

        let toreturn = PartField{
            content : content,
            reach : reachability,
        };
        toreturn
    }

    pub fn char_at(&self, col:i64, row:i64) -> Option<char> {
        if col < 0 || row < 0 {
            return None
        }

        let x: usize = col.try_into().unwrap();
        let y: usize = row.try_into().unwrap();

        if 0 <= col && x < self.content[0].len() {
            if 0 <= row && y < self.content.len() {
                return Some(self.content[y][x])
            }
        }
        None
    }

    pub fn reach_at(&self, col:i64, row:i64) -> bool {
        if col < 0 || row < 0 {
            return false
        }

        let cn: usize = col.try_into().unwrap();
        let rn: usize = row.try_into().unwrap();

        if rn < self.reach.len() {
            let reach_row = &self.reach[rn];
            if (row == 0 || row == 9 ) && col==2 {
                // println!("Reachability of row {} is {:?}", rn, reach_row)
            }
            if cn < reach_row.len() {
                let toreturn = reach_row[cn];
                return toreturn
            }
        }
        false
    }


    pub fn get_neigh_chars(&mut self, col:i64, row:i64) -> Vec<char> {
        // Return neigboring chars, but only if valid
        let mut toreturn: Vec<char> = Vec::new();

        if let Some(ch) = self.char_at(col-1, row-1) { toreturn.push(ch) };
        if let Some(ch) = self.char_at(col-1, row+0) { toreturn.push(ch) };
        if let Some(ch) = self.char_at(col-1, row+1) { toreturn.push(ch) };

        if let Some(ch) = self.char_at(col+0, row-1) { toreturn.push(ch) };
        // if let Some(ch) = self.char_at(col+0, row+0) { toreturn.push(ch) };
        if let Some(ch) = self.char_at(col+0, row+1) { toreturn.push(ch) };

        if let Some(ch) = self.char_at(col+1, row-1) { toreturn.push(ch) };
        if let Some(ch) = self.char_at(col+1, row+0) { toreturn.push(ch) };
        if let Some(ch) = self.char_at(col+1, row+1) { toreturn.push(ch) };
        toreturn
    }

    #[allow(dead_code)]
    pub fn propagate_reach(&mut self) {
        // Mark any subsequently reachable digits
        for cn in 0..self.content[0].len() {
            for rn in 0..self.content.len() {
                let col: i64 = cn.try_into().unwrap();
                let row: i64 = rn.try_into().unwrap();

                let subject = self.char_at(col, row).unwrap();
                if !subject.is_digit(10) {
                    continue;
                }

                if self.reach[rn][cn] {
                    continue;
                }

                // Column to the left
                if let Some(ch) = self.char_at(col-1, row-1) { 
                    if ch.is_digit(10) && self.reach_at(col-1, row-1) { self.reach[rn][cn] = true }
                };
                if let Some(ch) = self.char_at(col-1, row+0) { 
                    if ch.is_digit(10) && self.reach_at(col-1, row+0) { self.reach[rn][cn] = true }
                };
                if let Some(ch) = self.char_at(col-1, row+1) { 
                    if ch.is_digit(10) && self.reach_at(col-1, row+1) { self.reach[rn][cn] = true }
                };
        
                // Same column
                if let Some(ch) = self.char_at(col+0, row-1) { 
                    if ch.is_digit(10) && self.reach_at(col+0, row-1) { self.reach[rn][cn] = true }
                };
                if let Some(ch) = self.char_at(col+0, row+0) { // Technically skippable
                    if ch.is_digit(10) && self.reach_at(col+0, row+0) { self.reach[rn][cn] = true }
                };
                if let Some(ch) = self.char_at(col+0, row+1) { 
                    if ch.is_digit(10) && self.reach_at(col+0, row+1) { self.reach[rn][cn] = true }
                };
        

                // Column to the right
                if let Some(ch) = self.char_at(col+1, row-1) { 
                    if ch.is_digit(10) && self.reach_at(col+1, row-1) { self.reach[rn][cn] = true }
                };
                if let Some(ch) = self.char_at(col+1, row+0) { 
                    if ch.is_digit(10) && self.reach_at(col+1, row+0) { self.reach[rn][cn] = true }
                };
                if let Some(ch) = self.char_at(col+1, row+1) { 
                    if ch.is_digit(10) && self.reach_at(col+1, row+1) { self.reach[rn][cn] = true }
                };
                // println!("Propagated to row={}, col={}. Is now {}.", rn, cn, self.reach[rn][cn]);
            }
        }
    }

    pub fn mark_initial_reach(&mut self) {
        // Mark any digits that are initially reachable
        for cn in 0..self.content[0].len() {
            for rn in 0..self.content.len() {
                let col: i64 = cn.try_into().unwrap();
                let row: i64 = rn.try_into().unwrap();

                let subject = self.char_at(col, row).unwrap();
                if !subject.is_digit(10) {
                    continue;
                }

                self.reach[rn][cn] = false;
                
                let neighs = self.get_neigh_chars(col, row);
                for neigh in neighs {
                    if neigh.is_digit(10) {
                        continue;
                    }
                    if neigh == '.' {
                        continue;
                    }
                    // println!("Marking col={}, row={} as initially reachable.", col, row);
                    self.reach[rn][cn] = true;
                }               
            }
        }
    }

    #[allow(dead_code)]
    pub fn render(&self) -> String {
        // Returns display map.
        let mut toreturn = "".to_string();
        for line in self.content.iter() {
            let as_str = line.iter().cloned().collect::<String>();
            toreturn = format!("{}\n{}", toreturn, as_str)
        }
        toreturn
    }

    #[allow(dead_code)]
    pub fn reachability(&self) -> String {
        // Returns reachability map.
        let mut toreturn:Vec<char> = Vec::new();
        for row in 0..self.reach.len() {
            for col in 0..self.reach[0].len() {
                if self.reach_at(col.try_into().unwrap(), row.try_into().unwrap()) {
                    // println!("Found reach at row={}, col={}", row, col);
                    toreturn.push('X');
                } else {
                    toreturn.push('.');
                }
            }
            toreturn.push('\n');
        }
        toreturn.iter().cloned().collect::<String>()
    }


    #[allow(dead_code)]
    pub fn reachability_digits(&self) -> String {
        // Returns reachability map.
        let mut toreturn:Vec<char> = Vec::new();
        for rn in 0..self.reach.len() {
            for cn in 0..self.reach[0].len() {
                let col:i64 = cn.try_into().unwrap();
                let row:i64 = rn.try_into().unwrap();

                if self.reach_at(col, row) {
                    // println!("Found reach at row={}, col={}", row, col);
                    toreturn.push(self.char_at(col, row).unwrap());
                } else {
                    toreturn.push(' ');
                }
            }
            toreturn.push('\n');
        }
        toreturn.iter().cloned().collect::<String>()
    }

    #[allow(dead_code)]
    pub fn find_asterisks(&self) -> Vec<Coord> {
        // Find coords for all asterisks
        let mut toreturn = Vec::new();
        for rn in 0..self.reach.len() {
            for cn in 0..self.reach[0].len() {
                let col:i64 = cn.try_into().unwrap();
                let row:i64 = rn.try_into().unwrap();
                if let Some(ch) = self.char_at(col, row) {
                    if ch == '*' {
                        toreturn.push(Coord { rn, cn })
                    }
                }
            }
        }

        return toreturn
    }

    #[allow(dead_code)]
    pub fn find_ratios(&self) -> Vec<i64> {
        // Find ratios for all valid gears
        let mut toreturn = Vec::new();
        let asters = self.find_asterisks();

        for aster_coords in asters {
            let mut pf = PartField{content:self.content.clone(), reach:self.reach.clone() };
            pf.zap_nondigits_and_reach();
            pf.content[aster_coords.rn][aster_coords.cn] = '*';
            pf.mark_initial_reach();
            for _ in 0..20 {
                pf.propagate_reach()
            }

            let digits_str = pf.reachability_digits();
            // println!("Checking aster coord {:?}, reach is now\n{}", aster_coords, digits_str);

            let tokens = tokenize(&digits_str);
            if tokens.len() == 2 {
                let ratio = tokens[0].parse::<i64>().unwrap() * tokens[1].parse::<i64>().unwrap();
                toreturn.push(ratio);
            }
        }

        return toreturn
    }

    #[allow(dead_code)]
    pub fn zap_nondigits_and_reach(&mut self) {
        for rn in 0..self.reach.len() {
            for cn in 0..self.reach[0].len() {
                let col:i64 = cn.try_into().unwrap();
                let row:i64 = rn.try_into().unwrap();
                if let Some(ch) = self.char_at(col, row) {
                    if !ch.is_digit(10) {
                        self.content[rn][cn] = '.';
                    }
                }
                self.reach[rn][cn] = false;
            }
        }
    }
    
}

#[derive(Debug)]
pub struct Coord {
    rn: usize,
    cn: usize,
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut pf = PartField::from_content(input);
    pf.mark_initial_reach();
    for _ in 0..40 {
        pf.propagate_reach()
    }

    let to_tokenize = pf.reachability_digits();
    let tokens = tokenize(&to_tokenize);
    // println!("Part 1 got tokens {:?}", tokens);

    let mut toreturn = 0;
    for token in tokens {
        toreturn += token.parse::<i64>().unwrap();
    }
    return toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let pf = PartField::from_content(input);
    let ratios = pf.find_ratios();

    let mut toreturn = 0;
    for ratio in ratios {
        toreturn += ratio
    }

    return toreturn
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
    // $ cargo test --package adventurs --bin adventurs -- y2023::d3::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_tokenizer() {
        let input = "Game 2:            1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        
        Game 3: 8 green, 6 blue, 20 red; 5 blue,   4 red, 13           green; 5 green, 1 red
        ";
        let tokens = tokenize(input);
        assert!(tokens.len() > 0);
        assert!(tokens.get(0).unwrap() == "Game");
        assert!(tokens.get(5).unwrap() == "green");
        assert!(tokens.get(6).unwrap() == "3");

    }


    #[test]
    fn test_char_at() {
        let pbuf = input::get_input("2023_d3_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut pf = PartField::from_content(&content);
        assert_eq!(None, pf.char_at(-1, 0));
        assert_eq!(Some('4'), pf.char_at(0, 0));
        assert_eq!(Some('6'), pf.char_at(1, 0));            
        assert_eq!(Some('7'), pf.char_at(2, 0));   
        assert_eq!(Some('.'), pf.char_at(9, 9));
        assert_eq!(None, pf.char_at(9,10));
        assert_eq!(None, pf.char_at(10,9));

        pf.mark_initial_reach();
        println!("Got reachability 1\n{}", pf.reachability());
        assert!(pf.reach_at(2, 0));

        for _ in 0..20 {
           pf.propagate_reach();
        }
        println!("Got reachability 2\n{}", pf.reachability());

        let field_content = pf.reachability_digits();
        println!("Got reachability digits\n{}", field_content);

        assert!(field_content.contains(" 592 "));

    }
    
    #[test]
    fn test_gears() {
        let pbuf = input::get_input("2023_d3_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let pf = PartField::from_content(&content);


        let asters = pf.find_asterisks();
        println!("Got candidate asterisks {:?}", asters);
        assert_eq!(3, asters.len());

        let ratios = pf.find_ratios();
        println!("Got gear ratios {:?}", ratios);
        assert_eq!(2, ratios.len());

        // assert_eq!(2, ratios.len());

    }

    #[test]
    fn test_full() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d3::tests --nocapture

        {
            let pbuf = input::get_input("2023_d3_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();        
            let actual = part1(&content);
            assert_eq!(actual, 4361); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d3.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 512794); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d3_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 467835); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d3.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 67779080); // p2 skarp
        }
    }
}

use std::collections::{HashMap, HashSet};
use crate::input;
// use crate::grids::basic2d::{self, Grid2d, GridDirs8};

#[allow(dead_code)]
pub struct Day5 {
    ordering_rules: HashSet<(usize, usize)>,
    update_pages: Vec<Vec<usize>>,
    must_precede: HashMap<usize, HashSet<usize>>,
    must_succeed: HashMap<usize, HashSet<usize>>,
}

impl Day5 {
    pub fn from_lines(content: &str) -> Day5 {
        let mut ordering_rules: HashSet<(usize, usize)> = HashSet::new();
        let mut update_pages: Vec<Vec<usize>> = Vec::new();
        let rows: Vec<String> = content.split("\n").map(|r| r.to_owned()).collect();
        let mut must_precede: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut must_succeed: HashMap<usize, HashSet<usize>> = HashMap::new();

        for row in rows {
            if row.contains(",") {
                let ints = input::line_to_intvec(row.replace(",", " ").as_str()).iter().map(|i| *i as usize).collect();
                update_pages.push(ints);
            } else if row.contains("|") {
                let trimmed = row.trim();
                let pair: Vec<usize> = input::line_to_intvec(trimmed.replace("|", " ").as_str()).iter().map(|i| *i as usize).collect();
                ordering_rules.insert((pair[0], pair[1]));

                if !must_precede.contains_key(&pair[1]) {
                    must_precede.insert(pair[1], HashSet::new());
                }
                must_precede.get_mut(&pair[1]).unwrap().insert(pair[0]);

                if !must_succeed.contains_key(&pair[0]) {
                    must_succeed.insert(pair[0], HashSet::new());
                }
                must_succeed.get_mut(&pair[0]).unwrap().insert(pair[1]);
            }
        }
        
        Day5 {
            ordering_rules,
            update_pages,

            must_precede,
            must_succeed,
        }
    }

    pub fn part1(&mut self) -> i64 {
        let mut toreturn = 0;
        // println!("Update pages = {:?}", self.update_pages);
        // println!("Ordering rules = {:?}", self.ordering_rules);
        // println!("must_succeed = {:?}", self.must_succeed);
        // println!("must_precede = {:?}", self.must_precede);

        for update_nr in 0..self.update_pages.len() {
            if self.is_correct_order(&self.update_pages[update_nr]) {
                let mut index_middle = self.update_pages[update_nr].len();
                index_middle = ((index_middle as f64-1.0)/2.0) as usize;
                toreturn += self.update_pages[update_nr][index_middle];
            }
        }

        toreturn as i64
    }

    pub fn is_correct_order(&self, update: &Vec<usize>) -> bool {
        for page_idx_left in 0..update.len() {
            for page_idx_right in page_idx_left+1..update.len() {
                let page_nr_left = update[page_idx_left];
                let page_nr_right = update[page_idx_right];
                if let Some(must_be_right_of_right) = self.must_succeed.get(&page_nr_right) {
                    if must_be_right_of_right.contains(&page_nr_left) {
                        // println!("Update nr {} is NOT in correct order.", update_nr);
                        return false
                    }
                }
            }
        }

        // println!("Update nr {} is correct order.", update_nr);
        true
    }    



    pub fn part2(&mut self) -> i64 {
        let mut toreturn = 0;
        // println!("Update pages = {:?}", self.update_pages);
        // println!("Ordering rules = {:?}", self.ordering_rules);
        // println!("must_succeed = {:?}", self.must_succeed);
        // println!("must_precede = {:?}", self.must_precede);

        for update_nr in 0..self.update_pages.len() {
            let without_sorting = self.update_pages[update_nr].clone();
            if self.is_correct_order(&without_sorting) {
                // Do nothing with correct ones.
            } else {
                
                let ordered = self.order_update(without_sorting);

                let mut index_middle = ordered.len();
                index_middle = ((index_middle as f64-1.0)/2.0) as usize;
                toreturn += ordered[index_middle];
            }
        }

        toreturn as i64
    }

    pub fn order_update(&self, input: Vec<usize>) -> Vec<usize> {
        let mut toreturn = input.clone();
        while !self.is_correct_order(&toreturn)  {
            for page_idx_left in 0..toreturn.len() {
                for page_idx_right in page_idx_left+1..toreturn.len() {
                    let page_nr_left = toreturn[page_idx_left];
                    let page_nr_right = toreturn[page_idx_right];
                    if let Some(must_be_right_of_right) = self.must_succeed.get(&page_nr_right) {
                        if must_be_right_of_right.contains(&page_nr_left) {
                            // println!("Update nr {} is NOT in correct order.", update_nr);
                            let tmp = toreturn[page_idx_left];
                            toreturn[page_idx_left] = toreturn[page_idx_right];
                            toreturn[page_idx_right] = tmp;
                        }
                    }
                }
            }    
        }
        toreturn
    }

}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d5 = Day5::from_lines(input);    
    let toreturn = d5.part1();
    // d5.show();
    // println!("{:?}", d5.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d5 = Day5::from_lines(input);    
    let toreturn = d5.part2();
    // d5.show();
    // println!("{:?}", d5.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d5::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d5::tests --nocapture
        {
            let pbuf = input::get_input("2024_d5_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 143); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d5.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 5129); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d5_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 123); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d5.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 4077); // p2 skarp
        }
    }
}

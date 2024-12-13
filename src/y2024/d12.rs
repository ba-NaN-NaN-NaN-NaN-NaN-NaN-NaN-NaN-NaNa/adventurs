use std::cell;
use std::{collections::HashSet, hash::Hash};
use std::collections::VecDeque;
use crate::{grids::basic2d::{self, Grid2d, GridDirs8}, input};

#[allow(dead_code)]
pub struct Day12 {
    tiles: Vec<Vec<char>>,
    regions: Vec<Region>,
}

#[allow(dead_code)]
pub struct Region {
    representation: char,
    plot_locations: HashSet<(usize, usize)>,
}

impl Day12 {
    pub fn from_lines(content: &str) -> Day12 {
        let mut tiles: Vec<Vec<char>> = Vec::new();

        for line in content.split("\n") {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }

            let mut cells: Vec<char> = trimmed.chars().collect();
            cells.insert(0, '.');
            cells.push('.');
            tiles.push(cells);           
        }

        let header: Vec<char> = tiles[0].iter().map(|_| '.').collect();
        tiles.insert(0, header.clone());
        tiles.push(header);

        Day12 {
            tiles,
            regions: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn region_at(&self, row_nr: usize, col_nr: usize) -> Region {
        let repr = self.tiles[row_nr][col_nr];
        let mut plot_locations: HashSet<(usize, usize)> = HashSet::new();
        let mut worklist: VecDeque<(usize, usize)> = VecDeque::new();

        worklist.push_back((row_nr, col_nr));

        while worklist.len() > 0 {
            let current = worklist.pop_front().unwrap();
            if plot_locations.contains(&current) {
                continue
            }

            if self.tiles[current.0+1][current.1+0] == repr {
                worklist.push_back((current.0+1,current.1+0));
            }

            if self.tiles[current.0-1][current.1+0] == repr {
                worklist.push_back((current.0-1,current.1+0));
            }

            if self.tiles[current.0+0][current.1+1] == repr {
                worklist.push_back((current.0+0,current.1+1));
            }

            if self.tiles[current.0+0][current.1-1] == repr {
                worklist.push_back((current.0+0,current.1-1));
            }

            // println!("For location {:?}, worklist is now {:?}", current, worklist);
            plot_locations.insert(current);
        }

        println!("Found region at ({}, {}) with repr={} and area={}.",  row_nr, col_nr, repr, plot_locations.len());

        Region { representation: repr, plot_locations: plot_locations }
    }

    #[allow(dead_code)]
    pub fn calc_regions(&mut self) {
        let mut assigned_locations: HashSet<(usize, usize)> = HashSet::new();
        let mut found_regions: Vec<Region> = Vec::new();

        for row_nr in 0..self.tiles.len() {
            for col_nr in 0..self.tiles[0].len() {
                if self.tiles[row_nr][col_nr] == '.' {
                    continue;
                }

                if assigned_locations.contains(&(row_nr, col_nr))  {
                    continue
                }

                let region = self.region_at(row_nr, col_nr);
                for loc in region.plot_locations.iter() {
                    assigned_locations.insert(*loc);
                };
                found_regions.push(region);
            }
        }
        self.regions = found_regions
    }

    pub fn part1(&mut self) -> i64 {
        self.calc_regions();

        let mut toreturn = 0;
        for region in self.regions.iter() {
            toreturn += region.plot_locations.len() * region.num_boundaries();
        }

        toreturn as i64
    }

    pub fn part2(&mut self) -> i64 {
        self.calc_regions();
        let mut toreturn = 0;
        for region in self.regions.iter() {
            toreturn += region.plot_locations.len() * region.num_corners();
        }

        toreturn as i64
    }

    #[allow(dead_code)]
    pub fn show(&self) {
        for row_nr in 1..self.tiles.len()-1 {
            let mut row = self.tiles[row_nr].clone();
            row.pop();
            row.remove(0);
            
            let formatted: String = row.iter().map( |i| format!("{}", i)).collect();
            println!("show -> {:#?}", formatted);
        }
    }
}


impl Region {

    #[allow(dead_code)]
    pub fn num_boundaries(&self) -> usize {
        let mut toreturn:usize = 0;
        for loc in self.plot_locations.iter() {
            if !self.plot_locations.contains(&(loc.0+1, loc.1)) {
                toreturn += 1;
            }
            if !self.plot_locations.contains(&(loc.0-1, loc.1)) {
                toreturn += 1;
            }
            if !self.plot_locations.contains(&(loc.0, loc.1-1)) {
                toreturn += 1;
            }
            if !self.plot_locations.contains(&(loc.0, loc.1+1)) {
                toreturn += 1;
            }
        }

        println!("Region with representation '{}' has boundary length {}", self.representation, toreturn);
        toreturn
    }

    pub fn num_corners(&self) -> usize {
        // Is a bottom-right corner if right side
        let mut toreturn:usize = 0;
        for loc in self.plot_locations.iter() {
            let region_also_above: bool = self.plot_locations.contains(&(loc.0-1, loc.1));
            let region_also_right: bool = self.plot_locations.contains(&(loc.0, loc.1+1));
            let region_also_below: bool = self.plot_locations.contains(&(loc.0+1, loc.1));
            let region_also_left: bool = self.plot_locations.contains(&(loc.0, loc.1-1));

            // Is there a convex corner at top left?
            if !region_also_above && !region_also_left {
                toreturn += 1
            }

            // Is there a CONCAVE corner at top left?
            if region_also_above && region_also_left && !self.plot_locations.contains(&(loc.0-1, loc.1-1)) {
                toreturn += 1
            }

            // ----- Convex / concave cases follow in pairs ---------

            // Is there a convex corner at top right?
            if !region_also_above && !region_also_right {
                toreturn += 1
            }

            if region_also_above && region_also_right && !self.plot_locations.contains(&(loc.0-1, loc.1+1)) {
                toreturn += 1
            }

            // ---------------


            // Is there a convex corner at bottom left?
            if !region_also_below && !region_also_left {
                toreturn += 1
            }

            if region_also_below && region_also_left && !self.plot_locations.contains(&(loc.0+1, loc.1-1)) {
                toreturn += 1
            }

            // -------------

            // Is there a convex corner at bottom right?
            if !region_also_below && !region_also_right {
                toreturn += 1
            }
            if region_also_below && region_also_right && !self.plot_locations.contains(&(loc.0+1, loc.1+1)) {
                toreturn += 1
            }
        }
        println!("Region with representation '{}' has corner count {}", self.representation, toreturn);

        toreturn
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d12 = Day12::from_lines(input);    
    let toreturn = d12.part1();
    // d12.show();
    // println!("{:?}", d12.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d12 = Day12::from_lines(input);    
    let toreturn = d12.part2();
    // d12.show();
    // println!("{:?}", d12.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d12::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_parse() {
        let pbuf = input::get_input("2024_d12_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut d12 = Day12::from_lines(&content);
        d12.calc_regions();
        assert_eq!(11, d12.regions.len());
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d12::tests --nocapture
        {
            let pbuf = input::get_input("2024_d12_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 1930); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d12.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 1461806); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d12_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 1206); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d12.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 887932); // p2 skarp
        }
    }
}

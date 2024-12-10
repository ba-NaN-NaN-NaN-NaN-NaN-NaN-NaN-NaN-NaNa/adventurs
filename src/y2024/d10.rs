use std::{collections::HashSet, hash::Hash};
use std::collections::VecDeque;
use crate::{grids::basic2d::{self, Grid2d, GridDirs8}, input};

#[allow(dead_code)]
pub struct Day10 {
    tiles: Vec<Vec<i64>>,
    ratings: HashSet<Vec<(usize, usize)>>,
}

impl Day10 {
    pub fn from_lines(content: &str) -> Day10 {
        let mut tiles: Vec<Vec<i64>> = Vec::new();

        for line in content.split("\n") {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }

            let mut cells: Vec<i64> = trimmed.chars().map(|chr| chr.to_string().parse().unwrap()).collect();
            cells.insert(0, -99);
            cells.push(-99);

            // println!("line_to_intvec('{}')-> {:?}", line, cells);

            tiles.push(cells);

            
        }

        let header: Vec<i64> = tiles[0].iter().map(|_| -1).collect();
        tiles.insert(0, header.clone());
        tiles.push(header);

        let ratings: HashSet<Vec<(usize, usize)>> = HashSet::new();

        Day10 {
            tiles,
            ratings,
        }
    }

    pub fn part1(&mut self) -> i64 {
        
        let mut toreturn:i64 = 0;
        for th in self.find_trailheads().iter() {
            toreturn += self.score_from_trailhead(th.0, th.1) as i64;
        }
        toreturn
    }



    pub fn part2(&mut self) -> i64 {
        self.find_all_ratings() as i64
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

    #[allow(dead_code)]
    pub fn step_up(&self, row_nr: usize, col_nr: usize) -> HashSet<(usize, usize)> {
        let mut toreturn: HashSet<(usize, usize)> = HashSet::new();

        let val_locally = self.tiles[row_nr][col_nr];
        if self.tiles[row_nr+1][col_nr+0] == val_locally + 1 {
            toreturn.insert((row_nr+1, col_nr+0));
        }
        if self.tiles[row_nr+0][col_nr+1] == val_locally + 1 {
            toreturn.insert((row_nr+0, col_nr+1));
        }
        if self.tiles[row_nr-1][col_nr+0] == val_locally + 1 {
            toreturn.insert((row_nr-1, col_nr+0));
        }
        if self.tiles[row_nr+0][col_nr-1] == val_locally + 1 {
            toreturn.insert((row_nr+0, col_nr-1));
        }
        toreturn
    }

    pub fn ratings_from_trailhead(&self, row_nr: usize, col_nr: usize) -> HashSet<Vec<(usize, usize)>> {
        let mut toreturn :HashSet<Vec<(usize, usize)>>  = HashSet::new();
        let mut ratings_worklist: VecDeque<Vec<(usize, usize)>> = VecDeque::new();

        let mut paths_at_start: Vec<(usize, usize)> = Vec::new();
        paths_at_start.push((row_nr, col_nr));
        ratings_worklist.push_back(paths_at_start);

        while ratings_worklist.len() > 0 {
            let path_so_far = ratings_worklist.pop_back().unwrap();
            if path_so_far.len() == 10 {
                toreturn.insert(path_so_far);
                continue;
            }

            let end = path_so_far[path_so_far.len()-1];

            let row_nr = end.0;
            let col_nr = end.1;
            let current_altitute = self.tiles[row_nr][col_nr];

            if self.tiles[row_nr+1][col_nr+0] == current_altitute + 1 {
                let mut path_branch = path_so_far.clone();
                path_branch.push((row_nr+1, col_nr+0));
                ratings_worklist.push_back(path_branch);
            }
            if self.tiles[row_nr+0][col_nr+1] == current_altitute + 1 {
                let mut path_branch = path_so_far.clone();
                path_branch.push((row_nr+0, col_nr+1));
                ratings_worklist.push_back(path_branch);
                
            }
            if self.tiles[row_nr-1][col_nr+0] == current_altitute + 1 {
                let mut path_branch = path_so_far.clone();
                path_branch.push((row_nr-1, col_nr+0));
                ratings_worklist.push_back(path_branch);
                
            }
            if self.tiles[row_nr+0][col_nr-1] == current_altitute + 1 {
                let mut path_branch = path_so_far.clone();
                path_branch.push((row_nr+0, col_nr-1));
                ratings_worklist.push_back(path_branch);
            }
        }

        // println!("ratings_from_trailhead({}, {}) -> {} rating, paths: {:?}", row_nr, col_nr, toreturn.len(), toreturn);

        toreturn
    }


    #[allow(dead_code)]
    pub fn find_all_ratings(&mut self) -> usize {
        let mut ratings_found: HashSet<Vec<(usize, usize)>> = HashSet::new();

        for th in self.find_trailheads().iter() {
            ratings_found.extend(self.ratings_from_trailhead(th.0, th.1));
        }      

        self.ratings.clear();
        self.ratings.extend(ratings_found);
        self.ratings.len()
    }


    pub fn score_from_trailhead(&self, row_nr: usize, col_nr: usize) -> usize {
        if self.tiles[row_nr][col_nr] != 0 {
            panic!("lfvdjhg")
        }
        let height_1 = self.step_up(row_nr, col_nr);
        let mut height_2: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_1.iter() {
            height_2.extend(self.step_up(tile.0,tile.1));
        }

        let mut height_3: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_2.iter() {
            height_3.extend(self.step_up(tile.0,tile.1));
        }

        let mut height_4: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_3.iter() {
            height_4.extend(self.step_up(tile.0,tile.1));
        }

        let mut height_5: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_4.iter() {
            height_5.extend(self.step_up(tile.0,tile.1));
        }

        let mut height_6: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_5.iter() {
            height_6.extend(self.step_up(tile.0,tile.1));
        }

        let mut height_7: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_6.iter() {
            height_7.extend(self.step_up(tile.0,tile.1));
        }

        let mut height_8: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_7.iter() {
            height_8.extend(self.step_up(tile.0,tile.1));
        }

        let mut height_9: HashSet<(usize, usize)> = HashSet::new();
        for tile in height_8.iter() {
            height_9.extend(self.step_up(tile.0,tile.1));
        }

        // println!("Row {}, col {} has score {}", row_nr, col_nr, height_9.len());
        height_9.len()
    }


    fn find_trailheads(&self) -> HashSet<(usize, usize)> {
        let mut toreturn: HashSet<(usize, usize)> = HashSet::new();

        for row_nr in 1..self.tiles.len()-1 {
            for col_nr in 1..self.tiles[0].len()-1 {
                if self.tiles[row_nr][col_nr] == 0 {
                    toreturn.insert((row_nr, col_nr));
                }
            }
        }
        toreturn
    }



}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d10 = Day10::from_lines(input);    
    let toreturn = d10.part1();
    // d10.show();
    // println!("{:?}", d10.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d10 = Day10::from_lines(input);    
    let toreturn = d10.part2();
    // d10.show();
    // println!("{:?}", d10.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d10::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d10::tests --nocapture
        {
            let pbuf = input::get_input("2024_d10_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 36); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d10.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 593); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d10_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 81); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d10.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 1192); // p2 skarp
        }
    }
}

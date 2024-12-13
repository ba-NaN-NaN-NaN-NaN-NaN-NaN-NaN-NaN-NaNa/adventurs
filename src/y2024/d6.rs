use std::collections::HashSet;
use crate::grids::{basic2d::{self, Grid2d, GridDirs8}, walker::GridDirs};

#[allow(dead_code)]
pub struct Day6 {
    guard_row: usize,
    guard_col: usize,
    guard_direction: GridDirs,
    guard_visited: HashSet<(usize, usize)>,
    guard_visited_with_dir: HashSet<(usize, usize, GridDirs)>,
    tiles: Vec<Vec<char>>,
}


#[allow(dead_code)]
#[derive(Debug)]
pub enum TerminationMethod {
    MapExit,
    StuckInLoop,
}

impl Day6 {
    pub fn from_lines(content: &str) -> Day6 {
        let mut tiles: Vec<Vec<char>> = Vec::new();
        for line in content.split("\n") {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }

            let cells: Vec<char> = trimmed.chars().collect();
            tiles.push(cells);
        }

        let mut guard_row = 0;
        let mut guard_col = 0;
        let mut guard_direction: GridDirs = GridDirs::North;
        let mut guard_visited: HashSet<(usize, usize)> = HashSet::new();
        let mut guard_visited_with_dir: HashSet<(usize, usize, GridDirs)> = HashSet::new();

        for row_nr in 0..tiles.len() {
            for col_nr in 0..tiles[0].len() {
                let tile = tiles[row_nr][col_nr];
                
                match tile {
                    '#' => {},
                    '.' => {},
                    '^' => {
                        guard_row = row_nr;
                        guard_col = col_nr;
                        guard_direction = GridDirs::North;
                        guard_visited.insert((row_nr, col_nr));
                        guard_visited_with_dir.insert((row_nr, col_nr, GridDirs::North));

                    },
                    _ => {
                        panic!("Bad tile {}", tile)
                    }
                }
            }
        }

        Day6 {
            tiles,
            guard_col,
            guard_direction,
            guard_row,
            guard_visited,
            guard_visited_with_dir,
        }
    }

    pub fn forward_exits_map(&self) -> bool {
        match self.guard_direction {
            GridDirs::North => { self.guard_row == 0 },
            GridDirs::East => { self.guard_col == self.tiles[0].len() -1 },
            GridDirs::South=> { self.guard_row == self.tiles.len() -1  },
            GridDirs::West=> { self.guard_col == 0 },
        }
    }

    pub fn forward_has_obstacle(&self) -> bool {
        match self.guard_direction {
            GridDirs::North => { self.tiles[self.guard_row-1][self.guard_col] == '#' },
            GridDirs::East => { self.tiles[self.guard_row][self.guard_col+1] == '#' },
            GridDirs::South=> { self.tiles[self.guard_row+1][self.guard_col] == '#' },
            GridDirs::West=> { self.tiles[self.guard_row][self.guard_col-1] == '#' },
        }
    }

    pub fn guard_turn_right(&mut self) {
        match self.guard_direction {
            GridDirs::North => { self.guard_direction = GridDirs::East },
            GridDirs::East => { self.guard_direction = GridDirs::South },
            GridDirs::South=> { self.guard_direction = GridDirs::West },
            GridDirs::West=> { self.guard_direction = GridDirs::North },
        }

    }

    pub fn guard_step_forward(&mut self) {
        if self.forward_exits_map() || self.forward_has_obstacle() {
            panic!("fldjgh")
        }

        match self.guard_direction {
            GridDirs::North => { self.guard_row -= 1 },
            GridDirs::East => { self.guard_col += 1 },
            GridDirs::South=> { self.guard_row += 1 },
            GridDirs::West=> { self.guard_col -= 1 },
        }

        self.guard_visited.insert((self.guard_row, self.guard_col));
    }

    pub fn walk_until_exit_or_looping(&mut self) -> TerminationMethod {
        while !self.forward_exits_map() {
            if self.forward_has_obstacle() {
                self.guard_turn_right();
            } else {
                self.guard_step_forward();
            }

            if self.guard_visited_with_dir.contains(&(self.guard_row, self.guard_col, self.guard_direction)) {
                println!("Stuck in loop!");
                return TerminationMethod::StuckInLoop
            }

            self.guard_visited.insert((self.guard_row, self.guard_col));
            self.guard_visited_with_dir.insert((self.guard_row, self.guard_col, self.guard_direction));
        }

        println!("Exited map!");
        TerminationMethod::MapExit
    }

    pub fn part1(&mut self) -> i64 {
        self.walk_until_exit_or_looping();
        self.guard_visited.len() as i64
    }

    pub fn part2(&mut self) -> i64 {
        let mut p1result = Day6 {
            guard_col: self.guard_col,
            guard_direction:self.guard_direction,
            guard_row:self.guard_row,
            guard_visited: HashSet::new(),
            guard_visited_with_dir: HashSet::new(),
            tiles: self.tiles.clone(),    
        };
        p1result.walk_until_exit_or_looping();
        println!("Part 2 will check {} alternate universes", p1result.guard_visited.len());

        let mut toreturn = 0;
        for (row_nr, col_nr) in p1result.guard_visited {
            if self.tiles[row_nr][col_nr] != '.' {
                continue;
            }
            let mut alternate_universe = Day6 {
                guard_col: self.guard_col,
                guard_direction:self.guard_direction,
                guard_row:self.guard_row,
                guard_visited: HashSet::new(),
                guard_visited_with_dir: HashSet::new(),
                tiles: self.tiles.clone(),    
            };
            
            alternate_universe.tiles[row_nr][col_nr] = '#';
            println!("Checking alternate universe with blocker at row_nr={}, col_nr={}", row_nr, col_nr);
            let outcome = alternate_universe.walk_until_exit_or_looping();
            println!("Putting new obstacle at row_nr={}, col_nr={}, results in outcome {:#?} after {} steps.", row_nr, col_nr,  outcome, alternate_universe.guard_visited_with_dir.len());
            match outcome {
                TerminationMethod::MapExit => {},
                TerminationMethod::StuckInLoop => { toreturn += 1},
            };
        }
        toreturn
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d6 = Day6::from_lines(input);    
    let toreturn = d6.part1();
    // d6.show();
    // println!("{:?}", d6.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d6 = Day6::from_lines(input);    
    let toreturn = d6.part2();
    // d6.show();
    // println!("{:?}", d6.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d6::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d6::tests --nocapture
        {
            let pbuf = input::get_input("2024_d6_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 41); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d6.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 5461); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d6_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 6); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d6.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 1836); // p2 skarp
        }
    }
}

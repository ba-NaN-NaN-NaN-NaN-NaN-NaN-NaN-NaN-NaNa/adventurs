use std::collections::HashSet;
use crate::grids::basic2d::{self, Grid2d, GridDirs8};

#[allow(dead_code)]
pub struct Day4 {
    grid: Grid2d,
    used: Grid2d,
    found: HashSet<(GridDirs8, usize, usize)>,
    found_x_mas: HashSet<(usize, usize)>,
}

impl Day4 {
    pub fn from_lines(content: &str) -> Day4 {
        let grid = basic2d::Grid2d::from_lines(content);
        let mut used = basic2d::Grid2d::from_lines(content);
        for row_nr in 0..grid.num_rows {
            for col_nr in 0..grid.num_rows {
                used.tiles[row_nr][col_nr] = '.' as u8;
            }
        }    

        Day4 {
            grid,
            used,
            found: HashSet::new(),
            found_x_mas: HashSet::new(),
        }
    }

    pub fn part1(&mut self) -> i64 {
        for row_nr in 0..self.grid.num_rows-3 {
            for col_nr in 0..self.grid.num_rows {
                self.test_xmas(row_nr+3, col_nr, GridDirs8::North);
                self.test_xmas(row_nr, col_nr, GridDirs8::South);
            }
        }

        for row_nr in 0..self.grid.num_rows {
            for col_nr in 0..self.grid.num_rows-3 {
                self.test_xmas(row_nr, col_nr+3, GridDirs8::West);
                self.test_xmas(row_nr, col_nr, GridDirs8::East);
            }
        }

        for row_nr in 0..self.grid.num_rows-3 {
            for col_nr in 0..self.grid.num_rows-3 {
                self.test_xmas(row_nr+3, col_nr+3, GridDirs8::NorthWest);
                 self.test_xmas(row_nr+3, col_nr-0, GridDirs8::NorthEast);
                 self.test_xmas(row_nr-0, col_nr+3, GridDirs8::SouthWest);
                 self.test_xmas(row_nr-0, col_nr-0, GridDirs8::SouthEast);
            }
        }

        self.found.len() as i64
    }



    pub fn part2(&mut self) -> i64 {
        for row_nr in 1..self.grid.num_rows-1 {
            for col_nr in 1..self.grid.num_rows-1 {
                self.test_x_mas(row_nr, col_nr);
            }
        }

        self.found_x_mas.len() as i64
    }

    pub fn test_x_mas(&mut self, row_a: usize, col_a:usize) {
        // Test if a mas/mas X is at center of row_a, col_a
        if self.grid.tiles[row_a][col_a] != 'A' as u8 {
            return;
        }

        let found_se = self.grid.tiles[row_a-1][col_a-1] == 'M' as u8 &&
                             self.grid.tiles[row_a+1][col_a+1] == 'S' as u8;
        let found_nw = self.grid.tiles[row_a+1][col_a+1] == 'M' as u8 &&
                             self.grid.tiles[row_a-1][col_a-1] == 'S' as u8;


        let found_sw = self.grid.tiles[row_a-1][col_a+1] == 'M' as u8 &&
                             self.grid.tiles[row_a+1][col_a-1] == 'S' as u8;
        let found_ne = self.grid.tiles[row_a+1][col_a-1] == 'M' as u8 &&
                             self.grid.tiles[row_a-1][col_a+1] == 'S' as u8;

        
        if (found_se || found_nw) && (found_ne || found_sw) {
            self.used.tiles[row_a][col_a] = self.grid.tiles[row_a][col_a];

            self.used.tiles[row_a+1][col_a+1] = self.grid.tiles[row_a+1][col_a+1];
            self.used.tiles[row_a+1][col_a-1] = self.grid.tiles[row_a+1][col_a-1];
            self.used.tiles[row_a-1][col_a+1] = self.grid.tiles[row_a-1][col_a+1];
            self.used.tiles[row_a-1][col_a-1] = self.grid.tiles[row_a-1][col_a-1];
            self.found_x_mas.insert((row_a, col_a));
        }

    }

    pub fn show(&self) {
        for row in self.used.tiles.iter() {
            let chars: Vec<char> = row.iter().map( |ch| *ch as char).collect();
            let stringed = chars.iter().cloned().collect::<String>();
            println!("{}", stringed);
        }
    }


    pub fn test_xmas_2(&mut self, 
        row_x: usize, col_x:usize,
        row_m: usize, col_m:usize,
        row_a: usize, col_a:usize,
        row_s: usize, col_s:usize) -> bool {

        if self.grid.tiles[row_x][col_x] == 'X' as u8 &&
            self.grid.tiles[row_m][col_m] == 'M' as u8 &&
            self.grid.tiles[row_a][col_a] == 'A' as u8 &&
            self.grid.tiles[row_s][col_s] == 'S' as u8 
        {
            self.used.tiles[row_x][col_x] = 'X' as u8;
            self.used.tiles[row_m][col_m] = 'M' as u8;
            self.used.tiles[row_a][col_a] = 'A' as u8;
            self.used.tiles[row_s][col_s] = 'S' as u8;
            true
        } else {
            false
        }            
    }

    pub fn test_xmas(&mut self, row_nr: usize, col_nr: usize, direction: GridDirs8) {
        match direction {
            GridDirs8::North => {
                if self.test_xmas_2(
                    row_nr-0, col_nr+0,
                    row_nr-1, col_nr+0,
                    row_nr-2, col_nr+0,
                    row_nr-3, col_nr+0
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }
            },
            GridDirs8::East => {
                if self.test_xmas_2(
                    row_nr+0, col_nr+0,
                    row_nr+0, col_nr+1,
                    row_nr+0, col_nr+2,
                    row_nr+0, col_nr+3
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }

            },
            GridDirs8::West => {
                if self.test_xmas_2(
                    row_nr+0, col_nr+0,
                    row_nr+0, col_nr-1,
                    row_nr+0, col_nr-2,
                    row_nr+0, col_nr-3
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }

            },
            GridDirs8::South => {
                if self.test_xmas_2(
                    row_nr+0, col_nr+0,
                    row_nr+1, col_nr+0,
                    row_nr+2, col_nr+0,
                    row_nr+3, col_nr+0
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }
            },
            GridDirs8::NorthEast => {
                if self.test_xmas_2(
                    row_nr-0, col_nr+0,
                    row_nr-1, col_nr+1,
                    row_nr-2, col_nr+2,
                    row_nr-3, col_nr+3
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }
            },
            GridDirs8::NorthWest => {
                if self.test_xmas_2(
                    row_nr-0, col_nr-0,
                    row_nr-1, col_nr-1,
                    row_nr-2, col_nr-2,
                    row_nr-3, col_nr-3
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }
            },
            GridDirs8::SouthEast => {
                if self.test_xmas_2(
                    row_nr+0, col_nr+0,
                    row_nr+1, col_nr+1,
                    row_nr+2, col_nr+2,
                    row_nr+3, col_nr+3
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }
            },
            GridDirs8::SouthWest => {
                if self.test_xmas_2(
                    row_nr+0, col_nr+0,
                    row_nr+1, col_nr-1,
                    row_nr+2, col_nr-2,
                    row_nr+3, col_nr-3
                ) {
                    self.found.insert((direction, row_nr, col_nr));
                }
            },
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d4 = Day4::from_lines(input);    
    let toreturn = d4.part1();
    d4.show();
    println!("{:?}", d4.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d4 = Day4::from_lines(input);    
    let toreturn = d4.part2();
    d4.show();
    println!("{:?}", d4.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d4::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d4::tests --nocapture
        if false {
            let pbuf = input::get_input("2024_d4_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 18); // p1 sample
        }
        if false {
            let pbuf = input::get_input("2024_d4.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 2483); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d4_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 9); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d4.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 1925); // p2 skarp
        }
    }
}

use std::collections::{HashMap, HashSet};
use crate::grids::basic2d::{self, Grid2d, GridDirs8};


#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub struct Coord2d {
    row_nr: usize,
    col_nr: usize,
}

impl Coord2d {
    #[allow(dead_code)]
    fn antinode_behind(&self, other: &Coord2d) -> Option<Coord2d> {
        let delta_row = other.row_nr as i64 - self.row_nr as i64;
        let delta_col = other.col_nr as i64 - self.col_nr as i64;

        let antinode_row = other.row_nr as i64 + delta_row;
        if antinode_row < 0 { return None }
        let antinode_col = other.col_nr as i64 + delta_col;
        if antinode_col < 0 { return None }

        Some(Coord2d {
            row_nr: antinode_row as usize,
            col_nr: antinode_col as usize,
        })
    }

    #[allow(dead_code)]
    fn antinodes_in_line(&self, other: &Coord2d) -> HashSet<Coord2d> {
        let delta_row = other.row_nr as i64 - self.row_nr as i64;
        let delta_col = other.col_nr as i64 - self.col_nr as i64;
        
        let mut current_row: i64 = self.row_nr as i64;
        let mut current_col: i64 = self.col_nr as i64;

        let mut toreturn: HashSet<Coord2d> = HashSet::new();
        while 0 <= current_col && current_col <= 1000 && 
                0 <= current_row && current_row <= 1000 {
            toreturn.insert(Coord2d{
                col_nr: current_col as usize,
                row_nr: current_row as usize,
            });

            current_col += delta_col;
            current_row += delta_row;
        }
        toreturn
    }
}

#[allow(dead_code)]
pub struct Day8 {
    grid: Grid2d,
    frequencies: HashSet<char>,
    nodes: HashMap<char, HashSet<Coord2d>>,
    antinodes: HashMap<char, HashSet<Coord2d>>,
}

#[allow(dead_code)]
pub fn generate_antinodes(nodes: &HashSet<Coord2d>, map_height: usize, map_width: usize) -> HashSet<Coord2d> {
    nodes.clone()
}

impl Day8 {
    pub fn from_lines(content: &str) -> Day8 {
        let grid = basic2d::Grid2d::from_lines(content);
        let mut frequencies: HashSet<char> = HashSet::new();
        let mut nodes: HashMap<char, HashSet<Coord2d>> = HashMap::new();

        for row_nr in 0..grid.num_rows {
            for col_nr in 0..grid.num_cols {
                if grid.tiles[row_nr][col_nr] != '.' as u8 {
                    let content = grid.tiles[row_nr][col_nr] as char;
                    frequencies.insert(content);

                    if nodes.get(&content) == None {
                        nodes.insert(content, HashSet::new());
                    }

                    let node = Coord2d {
                        row_nr, 
                        col_nr
                    };
                    nodes.get_mut(&content).unwrap().insert(node);
                }
            }
        }

        let mut antinodes: HashMap<char, HashSet<Coord2d>> = HashMap::new();
        for freq in frequencies.iter() {
            let nodes_for_freq = nodes.get(freq).unwrap();
            println!("Nodes for '{}' are '{:?}'", freq, nodes_for_freq);

            let mut antinodes_for_freq: HashSet<Coord2d> = HashSet::new();
            for origin in nodes_for_freq.iter() {
                for other in nodes_for_freq.iter() {
                    if origin == other {
                        continue;
                    }

                    if let Some(antinode) = origin.antinode_behind(other) {
                        if antinode.row_nr < grid.num_rows && antinode.col_nr < grid.num_cols {
                            antinodes_for_freq.insert(antinode);
                        }
                    }
                }
            }

            println!("The {} antinodes for '{}' are '{:?}'", antinodes_for_freq.len(), freq, antinodes_for_freq);
            antinodes.insert(*freq, antinodes_for_freq);
        }

        Day8 {
            grid,
            frequencies,
            nodes, 
            antinodes
        }
    }

    pub fn part1(&mut self) -> usize {
        let mut uniqe_antinodes: HashSet<Coord2d> = HashSet::new();

        for (_, x) in self.antinodes.iter() {
            uniqe_antinodes.extend(x);
        }
        println!("Got {} unique antinodes: {:?}", uniqe_antinodes.len(), uniqe_antinodes);
        uniqe_antinodes.len()
    }



    pub fn part2(&mut self) -> usize {
        let mut unique_antinodes: HashSet<Coord2d> = HashSet::new();
        for freq in self.frequencies.iter() {
            let nodes_for_freq = self.nodes.get(freq).unwrap();
            // println!("Nodes for '{}' are '{:?}'", freq, nodes_for_freq);

            for origin in nodes_for_freq.iter() {
                for other in nodes_for_freq.iter() {
                    if origin == other {
                        continue;
                    }

                    unique_antinodes.extend(origin.antinodes_in_line(other));
                }
            }

            // println!("The {} antinodes for '{}' are '{:?}'", antinodes_for_freq.len(), freq, antinodes_for_freq);
            // unique_antinodes.insert(*freq, antinodes_for_freq);
        }


        let row_count = self.grid.tiles.len();
        let col_count = self.grid.tiles[0].len();
        unique_antinodes.retain( |antinode| {
            antinode.row_nr < row_count &&
            antinode.col_nr < col_count
        });
        unique_antinodes.len()
    }


    pub fn show(&self) {
        let mut tiles = self.grid.tiles.clone();

        let mut uniqe_antinodes: HashSet<Coord2d> = HashSet::new();
        for (_, x) in self.antinodes.iter() {
            uniqe_antinodes.extend(x);
        }

        for antinode in uniqe_antinodes {
            tiles[antinode.row_nr][antinode.col_nr] = '#' as u8;
        }

        for row in tiles.iter() {
            let chars: Vec<char> = row.iter().map( |ch| *ch as char).collect();
            let stringed = chars.iter().cloned().collect::<String>();
            println!("{}", stringed);
        }
    }


}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut d8 = Day8::from_lines(input);    
    
    let toreturn = d8.part1();
    d8.show();
    // d8.show();
    // println!("{:?}", d8.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut d8 = Day8::from_lines(input);    
    let toreturn = d8.part2();
    // d8.show();
    // println!("{:?}", d8.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d8::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d8::tests --nocapture
        {
            let pbuf = input::get_input("2024_d8_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 14); // p1 sample
        }
        {
            let pbuf = input::get_input("2024_d8.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 220); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d8_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 34); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d8.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 813); // p2 skarp
        }
    }
}

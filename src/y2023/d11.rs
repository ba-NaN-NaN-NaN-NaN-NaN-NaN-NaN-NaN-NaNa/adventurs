use core::num;
use std::collections::VecDeque;
use regex::Regex;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tile {
    appearance: char,
}

#[allow(dead_code)]
impl Tile {
    pub fn from_ch(ch: &char) -> Tile {
        Tile { appearance:*ch }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Universe {
    tiles: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Coord {
    line_nr: usize, 
    col_nr: usize,
}

impl Coord {
    #[allow(dead_code)]
    pub fn manh_dist_to(&self, other: &Coord) -> usize {
        let dx:usize = if self.line_nr < other.line_nr {
            other.line_nr-self.line_nr
        } else {
            self.line_nr-other.line_nr
        };

        let dy:usize = if self.col_nr < other.col_nr {
            other.col_nr-self.col_nr
        } else {
            self.col_nr-other.col_nr
        };

        dx+dy

    }
}


#[allow(dead_code)]
impl Universe {
    pub fn from_content(content: &str) -> Universe {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut tiles:Vec<Vec<Tile>> = Vec::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let mut tile_row: Vec<Tile> = Vec::new();
            for ch in trimmed.chars() {
                tile_row.push(Tile::from_ch(&ch));
            }
            tiles.push(tile_row);
        }

        Universe { 
            tiles: tiles,
        }
    }

    pub fn draw_glyphs(&self) -> String {
        let mut toreturn = "".to_string();

        for line in &self.tiles {
            let chars: String = line.iter().map( |t| t.appearance ).collect();
            toreturn = format!("{}\n{}", toreturn, chars);
        }
        toreturn.trim().to_string()
    }

    pub fn transpose(&self) -> Universe {
        let mut new_tiles: Vec<Vec<Tile>> = Vec::new();

        for col_nr in 0..self.tiles.get(0).unwrap().len() {
            let mut new_line: Vec<Tile> = Vec::new();
            for row_nr in 0..self.tiles.len() {
                new_line.push(self.tiles.get(row_nr).unwrap().get(col_nr).unwrap().clone())
            }
            new_tiles.push(new_line)
        }
        
        Universe { tiles: new_tiles }
    }

    pub fn print_transposed(&self) {
        // Used to get test data for transpose stuff
        println!(" --v-- Transposed! --v-- ");
        for col_nr in 0..self.tiles.get(0).unwrap().len() {
            for row_nr in 0..self.tiles.len() {
                print!("{}", self.tiles.get(row_nr).unwrap().get(col_nr).unwrap().appearance);
            }
            println!("");
        }
        println!(" --^-- Transposed! --^-- ");
    }

    pub fn expanded_lines(&self) -> Universe {
        let mut new_tiles: Vec<Vec<Tile>> = Vec::new();

        for line in &self.tiles {
            let mut has_galaxy = false;
            for tile in line {
                if tile.appearance == '#' {
                    has_galaxy = true;
                }
            }

            if has_galaxy {
                new_tiles.push(line.clone());
            }   else {
                new_tiles.push(line.clone());
                new_tiles.push(line.clone());
            }
        }
        
        Universe { tiles: new_tiles }        
    }

    pub fn singly_expanded(&self) -> Universe {
        // println!("Will expand universe {}", self.draw_glyphs());
        let mut toreturn = self.expanded_lines();

        // println!("Expanded once {}", toreturn.draw_glyphs());
        toreturn = toreturn.transpose();

        // println!("Transposed once {}", toreturn.draw_glyphs());
        toreturn = toreturn.expanded_lines();

        // println!("Expanded second time {}", toreturn.draw_glyphs());
        toreturn = toreturn.transpose();

        // println!("Transposed back {}", toreturn.draw_glyphs());
        toreturn
    }

    pub fn locate_galaxies(&self) -> Vec<Coord> {
        let mut toreturn: Vec<Coord> = Vec::new();

        for line_nr in 0..self.tiles.len() {
            for col_nr in 0..self.tiles.get(0).unwrap().len() {
                if self.tiles.get(line_nr).unwrap().get(col_nr).unwrap().appearance == '#' {
                    toreturn.push(Coord { line_nr, col_nr: col_nr });
                    // println!("Galaxy #{} found at ({}, {}))", toreturn.len(), line_nr, col_nr);
                }
            }
        }
        toreturn
    }

    pub fn sum_distances(&self) -> i64 {
        // This func is p1.
        let galaxies = self.locate_galaxies();
        return sum_distances(galaxies)
    }
}

pub fn sum_distances(galaxies: Vec<Coord>) -> i64 {
    let mut toreturn =  0;

    // println!("sum_distances(), the {} galaxies are @{:?}", galaxies.len(), galaxies);
    for x in 0..galaxies.len()-1 {
        for y in (x+1)..galaxies.len() {
            toreturn += galaxies.get(x).unwrap().manh_dist_to(galaxies.get(y).unwrap());
        }
    }

    toreturn.try_into().unwrap()
} 

#[allow(dead_code)]
pub fn multiplicated_expand(galaxies_orig: Vec<Coord>, galaxies_singly_expanded: Vec<Coord>, expand_mult: i64) -> Vec<Coord> {
    let mut toreturn : Vec<Coord> = Vec::new();
    if galaxies_orig.len() != galaxies_singly_expanded.len() {
        panic!("kjgfdhfvkdjhg");
    }
    for n in 0..galaxies_orig.len() {
        let orig = galaxies_orig.get(n).unwrap();
        let expanded = galaxies_singly_expanded.get(n).unwrap();

        let d_line: i64 = (expanded.line_nr - orig.line_nr).try_into().unwrap();
        let d_col: i64 = (expanded.col_nr - orig.col_nr).try_into().unwrap();

        let mut new_line_nr: i64 = orig.line_nr.try_into().unwrap();
        new_line_nr += d_line * expand_mult;

        let mut new_col_nr: i64 = orig.col_nr.try_into().unwrap();
        new_col_nr  += d_col * expand_mult;

        toreturn.push(Coord { 
            line_nr: new_line_nr.try_into().unwrap(),
            col_nr: new_col_nr.try_into().unwrap(),
        })
    }
    toreturn
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let univ_orig = Universe::from_content(input);
    let expanded = univ_orig.singly_expanded();
    expanded.sum_distances()
}

#[allow(dead_code)]
pub fn part2(input: &str, expand_mult: i64) -> i64 {
    let univ_orig = Universe::from_content(input);
    let galaxies_orig = univ_orig.locate_galaxies();

    let expanded = univ_orig.singly_expanded();
    let galaxies_singly_expanded = expanded.locate_galaxies();

    let galaxies_superexpanded = multiplicated_expand(galaxies_orig, galaxies_singly_expanded, expand_mult);
    sum_distances(galaxies_superexpanded)
}






#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d11::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_transpose() {
        let pbuf = input::get_input("2023_d11_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let galaxy = Universe::from_content(&content);
        let transposed = galaxy.transpose();
        //galaxy.print_transposed();

        assert_eq!("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", galaxy.draw_glyphs()); // p1 sample
        assert_eq!("..#......#
.....#....
..........
#.........
.........#
..........
....#.....
.#......#.
..........
......#...", transposed.draw_glyphs()); // p1 sample
        
    }

    #[test]
    fn test_expanded() {
        {
            let content = "#.\n..";
            let universe = Universe::from_content(&content);
            let expanded = universe.singly_expanded();
            assert_eq!("#..\n...\n...", expanded.draw_glyphs());
        }
        {
            let pbuf = input::get_input("2023_d11_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let universe = Universe::from_content(&content);
            let expanded = universe.singly_expanded();
            assert_eq!("....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......", expanded.draw_glyphs()); // p1 sample
        }
    }
    

    #[test]
    fn test_distances() {
        {
            let pbuf = input::get_input("2023_d11_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let universe = Universe::from_content(&content);
            let expanded = universe.singly_expanded();
            let galaxies = expanded.locate_galaxies();

            assert_eq!(15, galaxies.get(1-1).unwrap().manh_dist_to(galaxies.get(7-1).unwrap()));
            assert_eq!(15, galaxies.get(7-1).unwrap().manh_dist_to(galaxies.get(1-1).unwrap()));
            
            assert_eq!(17, galaxies.get(3-1).unwrap().manh_dist_to(galaxies.get(6-1).unwrap()));
            assert_eq!(17, galaxies.get(6-1).unwrap().manh_dist_to(galaxies.get(3-1).unwrap()));

            assert_eq!(5, galaxies.get(8-1).unwrap().manh_dist_to(galaxies.get(9-1).unwrap()));
            assert_eq!(5, galaxies.get(9-1).unwrap().manh_dist_to(galaxies.get(8-1).unwrap()));
        }
    }

    #[test]
    fn test_mult_expand() {
        {
            let galaxies_orig: Vec<Coord> = Vec::from([
                Coord{col_nr:0, line_nr:0},
                Coord{col_nr:8, line_nr:5},
                ]);
            let galaxies_singly_expanded: Vec<Coord> = Vec::from([
                Coord{col_nr:0, line_nr:0},
                Coord{col_nr:28, line_nr:6},
                ]);
            let expanded = multiplicated_expand(galaxies_orig, galaxies_singly_expanded, 2);
            let result = expanded.get(0).unwrap();
            assert_eq!(result.line_nr, 0);
            assert_eq!(result.col_nr, 0);

            let result = expanded.get(1).unwrap();
            assert_eq!(result.line_nr, 7);
            assert_eq!(result.col_nr, 48);
        }
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d11::tests --nocapture

        {
            let pbuf = input::get_input("2023_d11_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 374); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d11.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 9724940); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d11_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            {
                let actual = part2(&content, 10-1);
                assert_eq!(actual, 1030); // p2 sample
            }
            {
                let actual = part2(&content, 100-1);
                assert_eq!(actual, 8410); // p2 sample
            }
        }
        {
            let pbuf = input::get_input("2023_d11.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content, 1000000-1);
            assert_eq!(actual, 569052586852); // p2 skarp
        }
         
    }
}

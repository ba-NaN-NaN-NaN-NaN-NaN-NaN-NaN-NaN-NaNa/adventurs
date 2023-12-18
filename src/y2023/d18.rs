use std::{iter, collections::VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dir {
    North,
    East,
    South,
    West,
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Pool {
    dig_plan: Vec<Digline>,
    dig_plan_2: Vec<Digline>,
    perimeter: Vec<Hole>,
    tiles: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
impl Pool {
    pub fn from_content(content: &str) -> Pool {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut dig_plan:Vec<Digline> = Vec::new();
        let mut dig_plan_2:Vec<Digline> = Vec::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {continue;}
            let digline =  Digline::from_line(trimmed);
            dig_plan_2.push(digline.converted());
            dig_plan.push(digline);            
        }

        
        Pool { 
            dig_plan,
            dig_plan_2,
            perimeter: Vec::new(), // No initial holes.
            tiles: Vec::new(), // No initial tiles.
        }
    }

    pub fn draw_trench(&self) -> String {
        // Helper function only, to debug perimiter.
        let mut max_col: usize = 0;
        let mut max_row: usize = 0;

        for hole in self.perimeter.iter() {
            max_col = max_col.max(hole.col_nr.try_into().unwrap());
            max_row = max_row.max(hole.row_nr.try_into().unwrap());
        }

        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..max_row+1 {
            let row: Vec<char> = iter::repeat('.').take(max_col+1).collect();
            grid.push(row);
        }

        for hole in self.perimeter.iter() {
            let row_nr: usize = hole.row_nr.try_into().unwrap();
            let col_nr: usize = hole.col_nr.try_into().unwrap();
            let row: &mut Vec<char> = grid.get_mut(row_nr).unwrap();
            row[col_nr] = '#';  
        }

        let lines: Vec<String> = grid.iter().map( |chars| {
            let line: String = chars.iter().collect::<String>();
            line
        }).collect();

        lines.join("\n")
    }



    pub fn draw_with_lagoon(&self) -> String {
        

        if self.tiles.len() == 0 { panic!("Can not draw_with_lagoon() if tiles are not populated!")}
        
        let filled_lines: Vec<String> = self.tiles.iter().map( |row| {

            let chars: Vec<char> = row.iter().map( |t| {
                let ch: char = match &t.ttype {
                    TileType::Ground=>'.',
                    TileType::Hole=>'#',
                    TileType::Tbd=>'?',                
                };
                ch
            }).collect();

            chars.iter().collect::<String>()
        }).collect();

        filled_lines.join("\n")
    }

    pub fn dig_perimiter(&mut self) {
        self.perimeter.clear();
        self.perimeter.extend(dig_plan_2_holes(&self.dig_plan));
    }

    pub fn populate_tiles(&mut self) {
        // Using a known perimiter, convert to tiles.

        self.tiles.clear();
        let mut max_col: usize = 0;
        let mut max_row: usize = 0;

        for hole in self.perimeter.iter() {
            max_col = max_col.max(hole.col_nr.try_into().unwrap());
            max_row = max_row.max(hole.row_nr.try_into().unwrap());
        }

        for _ in 0..max_row+3 {
            let mut row:Vec<Tile> = Vec::new();
            for _ in 0..max_col+3 {
                row.push(Tile{
                    ttype: TileType::Tbd
                });
            }
            self.tiles.push(row);
        }
        
        for hole in self.perimeter.iter() {
            let row_nr: usize = hole.row_nr.try_into().unwrap();
            let col_nr: usize = hole.col_nr.try_into().unwrap();

            self.tiles.get_mut(row_nr+1).unwrap().get_mut(col_nr+1).unwrap().ttype = TileType::Hole;
        }

        let mut worklist : Vec<Coord> = Vec::new();
        worklist.push(Coord { row_nr: 0, col_nr: 0 });
        while worklist.len() > 0 {
            let todo = worklist.pop().unwrap();

            self.tiles.get_mut(todo.row_nr).unwrap().get_mut(todo.col_nr).unwrap().ttype = TileType::Ground;

            if todo.row_nr > 0 && self.tiles
                .get_mut(todo.row_nr-1).unwrap()
                .get_mut(todo.col_nr).unwrap().ttype == TileType::Tbd {
                    worklist.push(Coord { row_nr: todo.row_nr-1, col_nr: todo.col_nr });
            }

            if todo.col_nr > 0 && self.tiles
                .get_mut(todo.row_nr).unwrap()
                .get_mut(todo.col_nr-1).unwrap().ttype == TileType::Tbd {
                    worklist.push(Coord { row_nr: todo.row_nr, col_nr: todo.col_nr-1 });
            }

            if todo.row_nr < max_row+2 && self.tiles
                .get_mut(todo.row_nr+1).unwrap()
                .get_mut(todo.col_nr).unwrap().ttype == TileType::Tbd {
                    worklist.push(Coord { row_nr: todo.row_nr+1, col_nr: todo.col_nr });
            }

            if todo.col_nr < max_col+2 && self.tiles
                .get_mut(todo.row_nr).unwrap()
                .get_mut(todo.col_nr+1).unwrap().ttype == TileType::Tbd {
                    worklist.push(Coord { row_nr: todo.row_nr, col_nr: todo.col_nr+1 });
            }
        }


        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                if tile.ttype == TileType::Tbd {
                    tile.ttype = TileType::Hole;
                }
            }
        }


    }

    pub fn part1(&mut self) -> i64 {
        self.dig_perimiter();
        self.populate_tiles();

        let mut toreturn = 0;

        for row in self.tiles.iter() {
            for tile in row.iter() {
                if tile.ttype == TileType::Hole {
                    toreturn += 1;
                }
            }
        }

        toreturn
    }

    pub fn part1_try2(&mut self) -> i64 {
        let corners:Vec<Corner> = dig_plan_2_corners(&self.dig_plan);
        let mut horiz_start_stops: Vec<Hole> = corners_2_horiz_start_stops(&corners);

        if corners.len() < 30 {
            println!("Got corners (in travel order):");
            for corn in corners.iter() {
                println!("   {:?}", corn);
            }
        }

        horiz_start_stops.sort_by(|a, b| {
            if a.row_nr == b.row_nr {
                a.col_nr.cmp(&b.col_nr)
            } else {
                a.row_nr.cmp(&b.row_nr)
            }
        });

        if horiz_start_stops.len() < 50 {
            println!("Got horiz start/stops at {:?}", horiz_start_stops);
        }
        
        let mut to_sum_pairwise = VecDeque::from(horiz_start_stops);
        let mut toreturn: i64 = 0;

        while to_sum_pairwise.len() > 0 {
            let left = to_sum_pairwise.pop_front().unwrap();
            let right = to_sum_pairwise.pop_front().unwrap();
            if left.row_nr != right.row_nr {
                panic!("lfdjkh")
            }

            if right.col_nr < left.col_nr {
                panic!("gfdhgflhdg")
            }

            toreturn += right.col_nr - left.col_nr + 1
        }
        toreturn
    }

    pub fn part2(&mut self) -> i64 {
        /*
        self.dig_perimiter();
        self.populate_tiles();
        println!("Part 2 got converted dig_plan_2={:?}", self.dig_plan_2);
        -1
         */
        self.dig_plan.clear();
        self.dig_plan.extend(self.dig_plan_2.clone());
        self.part1_try2()
    }
}

#[allow(dead_code)]
pub fn dig_plan_2_holes(dig_plan: &Vec<Digline>) -> Vec<Hole> {
    // Dig & return a list of holes created by a dig plan.
    // All coords normalized such that the minimum is 0 on each end.
    let mut toreturn: Vec<Hole> = Vec::new();

    let mut curr_row:i64 = 0;
    let mut curr_col: i64 = 0;

    let mut min_row: i64 = 0;
    let mut min_col: i64 = 0;

    for dig_line in dig_plan.iter() {
        for _ in 0..dig_line.block_count {
            match dig_line.direction {
                Dir::North => { curr_row -= 1 },
                Dir::South => { curr_row += 1 },
                Dir::West => { curr_col -= 1 },
                Dir::East => { curr_col += 1 },
            }

            toreturn.push(Hole { 
                row_nr: curr_row,
                col_nr: curr_col,
            });

            min_row = min_row.min(curr_row);
            min_col = min_col.min(curr_col);
        };
    }

    for hole in toreturn.iter_mut() {
        hole.row_nr -= min_row;
        hole.col_nr -= min_col;
    }

    toreturn
}

pub fn corners_2_horiz_start_stops(corners: &Vec<Corner>) -> Vec<Hole> {
    let mut toreturn: Vec<Hole> = Vec::new();
    for corn in corners.iter() {
        if corn.next_dir == Dir::South {
            for i in 1..corn.distance_to_next {
                toreturn.push(Hole { row_nr: corn.row_nr+i , col_nr: corn.col_nr })
            }
        } else if corn.next_dir == Dir::North {
            for i in 1..corn.distance_to_next {
                toreturn.push(Hole { row_nr: corn.row_nr-i , col_nr: corn.col_nr })
            }

        } else {
            // Ignore east/west.
        }

        if corn.prev_dir == Dir::East && corn.next_dir == Dir::South {
            toreturn.push(Hole { row_nr: corn.row_nr, col_nr: corn.col_nr })
        }

        if corn.prev_dir == Dir::South && corn.next_dir == Dir::West {
            toreturn.push(Hole { row_nr: corn.row_nr, col_nr: corn.col_nr })
        }

        if corn.prev_dir == Dir::North && corn.next_dir == Dir::East {
            toreturn.push(Hole { row_nr: corn.row_nr, col_nr: corn.col_nr })
        }

        if corn.prev_dir == Dir::West && corn.next_dir == Dir::North {
            toreturn.push(Hole { row_nr: corn.row_nr, col_nr: corn.col_nr })
        }

    }

    toreturn
}


#[allow(dead_code)]
pub fn dig_plan_2_corners(dig_plan: &Vec<Digline>) -> Vec<Corner> {
    // Dig & return a list of holes created by a dig plan.
    // Coords are NOT normalized.
    let mut toreturn: Vec<Corner> = Vec::new();

    let mut curr_row:i64 = 0;
    let mut curr_col:i64 = 0;
    let mut prev_dir = dig_plan.last().unwrap().direction.clone();

    for dig_line in dig_plan.iter() {
        toreturn.push(Corner { 
            row_nr: curr_row,
            col_nr: curr_col,
            next_dir: dig_line.direction.clone(),
            prev_dir: prev_dir,
            distance_to_next: dig_line.block_count,
        });

        match dig_line.direction {
            Dir::North => { curr_row -= dig_line.block_count },
            Dir::South => { curr_row += dig_line.block_count },
            Dir::West => { curr_col -= dig_line.block_count },
            Dir::East => { curr_col += dig_line.block_count },
        }
        prev_dir = dig_line.direction.clone();
    }

    // let starting_point = toreturn.pop().unwrap();
    // toreturn.insert(0, starting_point);

    toreturn
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Hole {
    row_nr: i64, 
    col_nr: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Corner {
    row_nr: i64, 
    col_nr: i64,
    prev_dir: Dir,
    next_dir: Dir,
    distance_to_next: i64,
}




#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Coord {
    row_nr: usize, 
    col_nr: usize,
}



#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Ground,
    Hole,
    Tbd,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tile {
    ttype: TileType,
}



#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Digline {
    direction: Dir,
    block_count: i64,
    hexcode: String,
}

#[allow(dead_code)]
impl Coord {
    /*
    pub fn at_north(&self) -> Hole {
        Hole { row_nr: self.row_nr -1 , col_nr: self.col_nr , hexcode: "".to_string() }
    }

    pub fn at_east(&self) -> Hole {
        Hole { row_nr: self.row_nr, col_nr: self.col_nr + 1 , hexcode: "".to_string()}
    }

    pub fn at_south(&self) -> Hole {
        Hole { row_nr: self.row_nr+1, col_nr: self.col_nr  , hexcode: "".to_string()}
    }

    pub fn at_west(&self) -> Hole {
        Hole { row_nr: self.row_nr, col_nr: self.col_nr - 1 , hexcode: "".to_string()}
    }
     */
}

impl Digline {
    #[allow(dead_code)]
    pub fn from_line(content: &str) -> Digline {
        let frags: Vec<&str> = content.split(" ").collect();
        let direction = match *frags.get(0).unwrap() {
            "U" => Dir::North,
            "D" => Dir::South,
            "L" => Dir::West,
            "R" => Dir::East,
            _ => {
                println!("Digline::from_line('{}') failed to parse dir", content);
                panic!("Bad direction in digline from line")
            },
        };

        let block_count: i64 = frags.get(1).unwrap().parse().unwrap();

        Digline {
            direction: direction,
            block_count: block_count,
            hexcode: frags.get(2).unwrap().parse().unwrap(),
        }
    }

    pub fn converted(&self) -> Digline {
        let dir_char: &str = &self.hexcode[7..8];
        let count_chars = self.hexcode[2..7].to_string();

        Digline { 
            direction: match dir_char {
                "0" => Dir::East,
                "1" => Dir::South,
                "2" => Dir::West,
                "3" => Dir::North,
                _ => panic!("fldh rf f f  2")
            }, 
            block_count: i64::from_str_radix(&count_chars, 16).unwrap(),
            hexcode: "".to_string(), // This will cause error if we accidentally convert twice.
        }
    }
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d18::tests --nocapture

    use super::*;
    use crate::input;

    // #[test]
    #[allow(dead_code)]
    fn test_render() {
        let pbuf = input::get_input("2023_d18_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let pool = Pool::from_content(&content);
        let printed_map = pool.draw_trench();
        println!(" == Got map ==\n{}\n =========", printed_map);
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d18::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d18_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Pool::from_content(&content);
            let actual = pool.part1();

            let drawing = pool.draw_trench();
            println!(" == Got drawing of trench ==\n{}\n =^=======^=",drawing);

            pool.populate_tiles();
            let drawing = pool.draw_with_lagoon();
            println!(" == Got drawing of lagoon ==\n{}\n =^=======^=",drawing);
            assert_eq!(actual, 62); // p1 sample

            let actual = pool.part1_try2();
            assert_eq!(actual, 62); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d18.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Pool::from_content(&content);
            let actual = pool.part1_try2();
            assert_eq!(actual, 70026); // p1 skarp
        }

        {
            let pbuf = input::get_input("2023_d18_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Pool::from_content(&content);
            let actual = pool.part2();
            assert_eq!(actual, 952408144115); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d18.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Pool::from_content(&content);
            let actual = pool.part2();
            assert_eq!(actual, 68548301037382); // p2 skarp
        }
    }
}


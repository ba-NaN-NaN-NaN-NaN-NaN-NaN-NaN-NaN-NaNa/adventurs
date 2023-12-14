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

#[allow(dead_code)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
impl Grid {
    pub fn from_content(content: &str) -> Grid {
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

        Grid { 
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

    pub fn total_load(&self) -> i64 {
        let mut toreturn = 0;
        for row_nr in 0..self.tiles.len() {
            let line = self.tiles.get(row_nr).unwrap();
            let cost:i64 = (self.tiles.len() - row_nr).try_into().unwrap();
            for tile in line {
                if tile.appearance == 'O' {
                    toreturn += cost;
                }
            }
            // println!("total_load row_nr = {}, cost = {}. Total cost is now {}", row_nr, cost, toreturn);
        }

        toreturn
    }

    pub fn get_at(&self, coord: &Coord) -> char {
        let tile = self.tiles.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap().clone();
        tile.appearance
    }

    pub fn put_at(&mut self, coord: &Coord, appearance: &char) {
        self.tiles.get_mut(coord.line_nr).unwrap().get_mut(coord.col_nr).unwrap().appearance = *appearance
    }



    pub fn tilt_south(&mut self) -> bool {
        // Returns true if anything changed.
        let mut changed = false;
        let width = self.tiles.get(0).unwrap().len();
        let mut here = Coord{col_nr:0, line_nr:0};
        let mut south = here.at_south();

        for line_nr in (0..self.tiles.len()-1).rev() {
            here.line_nr = line_nr;
            south.line_nr = line_nr+1;
            for col_nr in 0..width {
                here.col_nr = col_nr;
                south.col_nr = col_nr;
                if self.get_at(&here) == 'O' && self.get_at(&south) == '.' {
                    self.put_at(&south, &'O');
                    self.put_at(&here, &'.');
                    changed = true
                }
            }
        }
        changed
    }

    pub fn tilt_north(&mut self) -> bool {
        // Returns true if anything changed.
        let mut changed = false;
        let width = self.tiles.get(0).unwrap().len();
        let mut here = Coord{col_nr:0, line_nr:1};
        let mut north = here.at_north();

        for line_nr in 1..self.tiles.len() {
            here.line_nr = line_nr;
            north.line_nr = line_nr-1;
            for col_nr in 0..width {
                here.col_nr = col_nr;
                north.col_nr = col_nr;
                if self.get_at(&here) == 'O' && self.get_at(&north) == '.' {
                    self.put_at(&north, &'O');
                    self.put_at(&here, &'.');
                    changed = true
                }
            }
        }
        changed
    }

    pub fn tilt_west(&mut self) -> bool {
        // Returns true if anything changed.
        let mut changed = false;
        let height = self.tiles.len();
        let width = self.tiles.get(0).unwrap().len();

        let mut here = Coord{col_nr:1, line_nr:0};
        let mut west = here.at_west();

        for line_nr in 0..height {
            here.line_nr = line_nr;
            west.line_nr = line_nr;
            for col_nr in 1..width {
                here.col_nr = col_nr;
                west.col_nr = col_nr-1;
                if self.get_at(&here) == 'O' && self.get_at(&west) == '.' {
                    self.put_at(&west, &'O');
                    self.put_at(&here, &'.');
                    changed = true
                }
            }
        }
        changed
    }

    pub fn tilt_east(&mut self) -> bool {
        // Returns true if anything changed.
        let mut changed = false;
        let height = self.tiles.len();
        let width = self.tiles.get(0).unwrap().len();

        let mut here = Coord{col_nr:1, line_nr:0};
        let mut east = here.at_east();

        for line_nr in 0..height {
            here.line_nr = line_nr;
            east.line_nr = line_nr;
            for col_nr in (0..width-1).rev() {
                here.col_nr = col_nr;
                east.col_nr = col_nr+1;
                if self.get_at(&here) == 'O' && self.get_at(&east) == '.' {
                    self.put_at(&east, &'O');
                    self.put_at(&here, &'.');
                    changed = true
                }
            }
        }
        changed
    }
    pub fn cycle(&mut self) {
        let mut changed = true;
        while changed {
            changed = self.tilt_north();
        }

        changed = true;
        while changed {
            changed = self.tilt_west();
        }

        changed = true;
        while changed {
            changed = self.tilt_south();
        }

        changed = true;
        while changed {
            changed = self.tilt_east();
        }
        
    }


}



#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Coord {
    line_nr: usize, 
    col_nr: usize,
}

#[allow(dead_code)]
impl Coord {
    pub fn at_south(&self) -> Coord {
        Coord { line_nr: self.line_nr + 1 , col_nr: self.col_nr  }
    }

    pub fn at_east(&self) -> Coord {
        Coord { line_nr: self.line_nr, col_nr: self.col_nr + 1 }
    }

    pub fn at_north(&self) -> Coord {
        Coord { line_nr: self.line_nr - 1, col_nr: self.col_nr  }
    }

    pub fn at_west(&self) -> Coord {
        Coord { line_nr: self.line_nr, col_nr: self.col_nr - 1 }
    }
}




#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut grid = Grid::from_content(&input);
    let mut changed = grid.tilt_north();
    while changed {
        changed = grid.tilt_north();
    }
    // let drawn = grid.draw_glyphs();
    // println!(" == Grid == \n{}", drawn);
    grid.total_load()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut grid = Grid::from_content(&input);
    let mut loads: Vec<i64> = Vec::new();
    loads.push(grid.total_load());
    
    for _ in 0..125 {
        grid.cycle();
        loads.push(grid.total_load());
    }

    for n in 0..1000 {
        if n % 30 == 0 {
            println!("Loads is {:?}", loads);
        }
        grid.cycle();
        loads.push(grid.total_load());

        let as_cyclic = into_cyclic(&loads);

        if let Some(res) = as_cyclic {
            println!("Cyclic result is {:?}", res);
    
            // let drawn = grid.draw_glyphs();
            // println!(" == Grid == \n{}", drawn);
            // grid.total_load()
            
            return *res.get(1000000000 % res.len()).unwrap()        
        }
    };
    -1
}



#[allow(dead_code)]
pub fn into_cyclic(sequence: &Vec<i64>) -> Option<Vec<i64>> {
    for length_to_test in 30..60 {
        // sequence.splice(0..20, []).collect();
        let mut tail: Vec<i64> = Vec::new();
        let mut pre_tail : Vec<i64> = Vec::new();
        for n in 0..length_to_test {
            tail.push(sequence.get(sequence.len() - 1 * length_to_test + n).unwrap().clone());
            pre_tail.push(sequence.get(sequence.len() - 2 * length_to_test + n).unwrap().clone());
        }

        if tail == pre_tail {
            let start_at = sequence.len() - length_to_test*2;
            println!("Found sequence of len {}, starting at offset {}", length_to_test, start_at);

            let mut toreturn:Vec<i64> = Vec::new();

            let cycles_to_chop = sequence.len() / length_to_test - 1;
            let offset:usize = cycles_to_chop * length_to_test;
            for loc_in_cycle in 0..length_to_test {
                let loc_in_seq = offset+loc_in_cycle;
                toreturn.push(*sequence.get(loc_in_seq).unwrap());
            }
            return Some(toreturn)
        }
    }

    None
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
    // $ cargo test --package adventurs --bin adventurs -- y2023::d14::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_tilt() {
        let pbuf = input::get_input("2023_d14_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();

        {
            let mut grid = Grid::from_content(&content);
            let drawn = grid.draw_glyphs();
            // println!(" == Grid before any tilt == \n{}", drawn);

            grid.tilt_north();
            let drawn = grid.draw_glyphs();
            // println!(" == Grid after north tilt == \n{}", drawn);
            assert_eq!("
O.OO.#....
O...#....#
OO..O##..O
.O.#...O..
O....O..#.
..#...O#.#
..O..#.O.O
..........
#OO..###..
#....#....            
".trim(), drawn);
        }
    
        {
            let mut grid = Grid::from_content(&content);
            grid.tilt_south();
            let drawn = grid.draw_glyphs();
            // println!(" == Grid after south tilt == \n{}", drawn);
            assert_eq!("
.....#....
O...#....#
O.OO.##...
...#......
OO..O..O#O
.O#..O.#.#
O....#....
..O...OO.O
#....###..
#OO..#....    
".trim(), drawn);
        }

        {
            let mut grid = Grid::from_content(&content);
            grid.tilt_east();
            let drawn = grid.draw_glyphs();
            // println!(" == Grid after east tilt == \n{}", drawn);
            assert_eq!("
.O...#....
.OOO#....#
.....##...
.OO#.O...O
..O....O#.
.O#...O#.#
...O.#.O.O
........O.
#....###..
#.OO.#....    
".trim(), drawn);
        }

        {
            let mut grid = Grid::from_content(&content);
            grid.tilt_west();
            let drawn = grid.draw_glyphs();
            // println!(" == Grid after west tilt == \n{}", drawn);
            assert_eq!("
O....#....
OOO.#....#
.....##...
OO.#O...O.
O.....O.#.
O.#.O..#.#
.O...#O.O.
......O...
#....###..
#OO..#....    
".trim(), drawn);
        }

    }



    #[test]
    fn test_into_cyclic() {


        let prefix300:Vec<i64> = Vec::from_iter(200..1300);
        let prefix400:Vec<i64> = Vec::from_iter(200..1400);
        let cycle55:Vec<i64> = Vec::from_iter(100..155);
        let cycle44:Vec<i64> = Vec::from_iter(100..144);

        {
            let mut foo:Vec<i64> = prefix300.clone();
            for _ in 0..10 {
                foo.extend(cycle55.clone().iter());
            }

            let cycled = into_cyclic(&foo).unwrap();

            for loc in foo.len()-200..foo.len()-10  {
                assert_eq!(foo.get(loc).unwrap(), cycled.get(loc % cycled.len()).unwrap());
            }
        }
        
        {
            let mut foo:Vec<i64> = prefix300.clone();
            for _ in 0..10 {
                foo.extend(cycle44.clone().iter());
            }
            let cycled = into_cyclic(&foo).unwrap();
            for loc in foo.len()-200..foo.len()-10  {
                assert_eq!(foo.get(loc).unwrap(), cycled.get(loc % cycled.len()).unwrap());
            }
        }

        {
            let mut foo:Vec<i64> = prefix400.clone();
            for _ in 0..10 {
                foo.extend(cycle55.clone().iter());
            }

            let cycled = into_cyclic(&foo).unwrap();

            for loc in foo.len()-200..foo.len()-10  {
                assert_eq!(foo.get(loc).unwrap(), cycled.get(loc % cycled.len()).unwrap());
            }
        }
        
        {
            let mut foo:Vec<i64> = prefix400.clone();
            for _ in 0..10 {
                foo.extend(cycle44.clone().iter());
            }
            let cycled = into_cyclic(&foo).unwrap();
            for loc in foo.len()-200..foo.len()-10  {
                assert_eq!(foo.get(loc).unwrap(), cycled.get(loc % cycled.len()).unwrap());
            }
        }

    }



    #[test]
    fn test_cycle() {
        let pbuf = input::get_input("2023_d14_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();

        let mut grid = Grid::from_content(&content);
        let drawn = grid.draw_glyphs();
        // println!(" == Grid before any tilt == \n{}", drawn);

        {
            grid.cycle();
            let drawn = grid.draw_glyphs();
            // println!(" == Grid after one cycle == \n{}", drawn);
            let expected = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
            assert_eq!(expected, drawn);
        }
        
        {
            grid.cycle();
        }

        {
            grid.cycle();

            let drawn = grid.draw_glyphs();
            // println!(" == Grid after three cycles == \n{}", drawn);
            let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O".trim();
            assert_eq!(expected, drawn);
        }


    
    }
    

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d14::tests --nocapture


        
        {
            let pbuf = input::get_input("2023_d14_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 136); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d14.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);

            // 107811 too low 
            assert!(actual > 107811);
            assert_eq!(actual, 108813); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d14_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 64); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d14.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            // 102 is too low.
            assert_eq!(actual, 104533); // p2 skarp
        }
         
    }
}

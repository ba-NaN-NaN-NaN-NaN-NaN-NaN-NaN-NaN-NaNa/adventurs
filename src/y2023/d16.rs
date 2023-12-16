#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tile {
    appearance: char,
    visitcount: i64,
    processed_n : bool,
    processed_e : bool,
    processed_s : bool,
    processed_w : bool,
}

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
pub struct Beam {
    dir: Dir,
    loc: Coord,
}


#[allow(dead_code)]
impl Tile {
    /*
    empty space (.), mirrors (/ and \), and splitters (| and -).
    */    
    pub fn beam_dir_processed(&mut self, dir: &Dir) -> bool {
        match dir {
            Dir::North => self.processed_n,
            Dir::East => self.processed_e,
            Dir::South => self.processed_s,
            Dir::West => self.processed_w,
        }
    }

    pub fn mark_beam_dir(&mut self, dir: &Dir) {
        match dir {
            Dir::North => self.processed_n = true,
            Dir::East => self.processed_e = true,
            Dir::South => self.processed_s = true,
            Dir::West => self.processed_w = true,
        }
    }

    pub fn from_ch(ch: &char) -> Tile {
        match ch {
            '/' => { Tile { appearance:*ch, visitcount:0, processed_e: false, processed_n:false, processed_s:false, processed_w:false}},
            '\\' => { Tile { appearance:*ch, visitcount:0, processed_e: false, processed_n:false, processed_s:false, processed_w:false }},
            '|' => { Tile { appearance:*ch, visitcount:0, processed_e: false, processed_n:false, processed_s:false, processed_w:false, }},
            '-' => { Tile { appearance:*ch,visitcount:0, processed_e: false, processed_n:false, processed_s:false, processed_w:false, }},
            '.' => { Tile { appearance:*ch,visitcount:0, processed_e: false, processed_n:false, processed_s:false, processed_w:false, }},
            _ => {panic!("fd")},
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    beams: Vec<Beam>,
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

        let mut costs: Vec<Vec<i64>> = Vec::new();

        for line in &tiles {
            let cost_line: Vec<i64> = line.iter().map( |t| { 
                match t.appearance {
                    'S' => 0,
                    _ => i64::MAX,
                }}).collect();
            costs.push(cost_line);
        }

        Grid { 
            tiles: tiles,
            beams: Vec::new(),
        }
    }

    pub fn print_glyphs(&self) -> String {
        let mut toreturn = "".to_string();

        for line in &self.tiles {
            let chars: String = line.iter().map( |t| t.appearance ).collect();
            toreturn = format!("{}\n{}", toreturn, chars);
        }
        toreturn.trim().to_string()
    }

    pub fn render_visited(&self, ) -> String {
        let mut frags: Vec<String> = Vec::new();

        for line in &self.tiles {
            let mut row = "   ".to_string();
            for tile in line {
                if tile.visitcount > 0 {
                    row = format!("{}#", row);
                } else {
                    row = format!("{}.", row);
                }
            }
            // let chars: String = line.iter().map( |t| t.appearance ).collect();
            frags.push(row);
        }
        frags.join("\n").to_string()
    }

    pub fn count_visited(&self) -> i64 {
        let mut toreturn = 0;
        for row in self.tiles.iter() {
            for tile in row.iter() {
                if tile.visitcount > 0 {
                    toreturn += 1;
                }
            }
        }
        toreturn
    }

    
    fn tile_copy_at(&self, coord: &Coord) -> Tile {
        let here = self.tiles.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap();
        here.clone()
    }
    
    fn step_beams(&mut self) -> bool {
        // All beams take one step.   
        let mut new_beams: Vec<Beam> = Vec::new();
        // println!("step {} beams.", self.beams.len());

        while self.beams.len() > 0 {
            let mut beam = self.beams.pop().unwrap();
            // Check leaves map.
            match beam.dir {
                Dir::North => {
                    if beam.loc.line_nr == 0 { 
                        continue; 
                    } else {
                        beam.loc.line_nr -= 1;
                    }
                },
                Dir::East => {
                    if beam.loc.col_nr == self.tiles.get(0).unwrap().len() - 1 { 
                        continue; 
                    } else {
                        beam.loc.col_nr += 1;
                    }
                },
                Dir::South => {
                    if beam.loc.line_nr == self.tiles.len() - 1 { 
                        continue; 
                    } else {
                        beam.loc.line_nr += 1;
                    }
                },
                Dir::West => {
                    if beam.loc.col_nr == 0 { 
                        continue; 
                    } else {
                        beam.loc.col_nr -= 1;
                    }
                },
            };
            new_beams.push(beam);
        }
        self.beams.extend(new_beams);
        self.beams.len() > 0
    }
    
    fn process_beams(&mut self) {
        // Process what happens to beams where they are.
        let mut new_beams: Vec<Beam> = Vec::new();
        // println!("step {} beams.", self.beams.len());

        while self.beams.len() > 0 {
            let mut beam = self.beams.pop().unwrap();            
            let tile = self.tiles.get_mut(beam.loc.line_nr).unwrap()
                      .get_mut(beam.loc.col_nr).unwrap();
            tile.visitcount += 1;
            // println!("Tile at {:?} now visited {}", beam.loc, tile.visitcount);

            if tile.beam_dir_processed(&beam.dir) {
                continue;
            }

            tile.mark_beam_dir(&beam.dir);

            match tile.appearance {
                '/' => { 
                    beam.dir = match beam.dir {
                        Dir::North => Dir::East,
                        Dir::East => Dir::North,

                        Dir::South => Dir::West,
                        Dir::West => Dir::South, 
                    };
                    new_beams.push(beam);
                },
                '\\' => { 
                    beam.dir = match beam.dir {
                        Dir::North => Dir::West,
                        Dir::West => Dir::North, 

                        Dir::East => Dir::South,
                        Dir::South => Dir::East,
                    };
                    new_beams.push(beam);
                },
                '|' => {
                    if beam.dir == Dir::West || beam.dir == Dir::East {
                        // Split
                        let mut splitted = beam.clone();
                        splitted.dir = Dir::North;
                        beam.dir = Dir::South;
                        new_beams.push(splitted);
                        new_beams.push(beam);
                    } else {
                        // Unchanged
                        new_beams.push(beam);
                    }
                },
                '-' => { 
                    if beam.dir == Dir::South || beam.dir == Dir::North {
                        // Split
                        let mut splitted = beam.clone();
                        splitted.dir = Dir::East;
                        beam.dir = Dir::West;
                        new_beams.push(splitted);
                        new_beams.push(beam);
                    } else {
                        // Unchanged
                        new_beams.push(beam);
                    }
                },
                '.' => { new_beams.push(beam); },
                _ => {panic!("fd")},
            };
        }

        self.beams.extend(new_beams);
    }

    fn propagate_beams(&mut self) {
        while self.beams.len() > 0 {
            self.process_beams();
            self.step_beams();
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
pub fn part1(content: &str) -> i64 {
    let mut grid = Grid::from_content(&content);
    grid.beams.push(Beam{dir:Dir::East, loc: Coord { line_nr: 0, col_nr: 0}});
    grid.propagate_beams(); 
    // let printed_map = grid.render_visited();
    // println!("\n == Got visited map ==\n{}\n =========", printed_map);
    grid.count_visited()
}

#[allow(dead_code)]
fn part2(content: &str) -> i64 {
    let mut toreturn = 0;
    let grid_ref = Grid::from_content(&content);

    let col_count = grid_ref.tiles.get(0).unwrap().len();
    let line_count = grid_ref.tiles.len();
    
    for col_nr in 0..col_count {
        // South-going
        let mut grid = grid_ref.clone();
        grid.beams.push(Beam{dir:Dir::South, loc: Coord { line_nr: 0, col_nr: col_nr}});        
        grid.propagate_beams();
        toreturn = toreturn.max(grid.count_visited());    

        // North-going
        let mut grid = grid_ref.clone();
        grid.beams.push(Beam{dir:Dir::North, loc: Coord { line_nr: line_count-1, col_nr: col_nr}});        
        grid.propagate_beams();
        toreturn = toreturn.max(grid.count_visited());        
    }

    for line_nr in 0..line_count {
        // East-going
        let mut grid = grid_ref.clone();
        grid.beams.push(Beam{dir:Dir::East, loc: Coord { line_nr, col_nr: 0}});        
        grid.propagate_beams();
        toreturn = toreturn.max(grid.count_visited());    

        // West-going
        let mut grid = grid_ref.clone();
        grid.beams.push(Beam{dir:Dir::West, loc: Coord { line_nr, col_nr: col_count - 1}});        
        grid.propagate_beams();
        toreturn = toreturn.max(grid.count_visited());        
    }
    
    // println!("part2() from compressed render: {} gave {} as toreturn", rendered_reach, toreturn);
    toreturn
}



#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d16::tests --nocapture

    use super::*;
    use crate::input;

    // #[test]
    #[allow(dead_code)]
    fn test_render() {
        let pbuf = input::get_input("2023_d16_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut grid = Grid::from_content(&content);
        let printed_map = grid.print_glyphs();
        println!("\n == Got map ==\n{}\n =========", printed_map);

        // let printed_costs = grid.render_visited();
        // println!(" == Got visited ==\n{}\n =========", printed_costs);

        grid.propagate_beams();

        // let printed_costs = grid.render_visited();
        // println!(" == Got visited ==\n{}\n =========", printed_costs);

    }


    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d16::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d16_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let visitcount = part1(&content);
            assert_eq!(visitcount, 46); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d16.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let visitcount = part1(&content);
            assert!(visitcount > 62);
            assert_eq!(visitcount, 7996); // p1 skarp
        }
        {
            let pbuf = input::get_input("2023_d16_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 51); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d16.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 8239); // p2 skarp
        }
    }
}

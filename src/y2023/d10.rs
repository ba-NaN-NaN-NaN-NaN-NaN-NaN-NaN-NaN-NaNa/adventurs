use std::collections::VecDeque;
use regex::Regex;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tile {
    appearance: char,
    con_north: bool,
    con_east: bool,
    con_south: bool,
    con_west: bool,
    in_loop: bool,
    east_in_loop: bool,
    south_in_loop: bool,
}

#[allow(dead_code)]
impl Tile {
    /*
    
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    */
    
    pub fn from_ch(ch: &char) -> Tile {
        match ch {
            'L' => { Tile { appearance:*ch, con_east: true, con_north:true, con_south:false, con_west:false, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            'F' => { Tile { appearance:*ch, con_east: true, con_north:false, con_south:true, con_west:false, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            '-' => { Tile { appearance:*ch, con_east: true, con_north:false, con_south:false, con_west:true, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            '|' => { Tile { appearance:*ch, con_east: false, con_north:true, con_south:true, con_west:false, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            'J' => { Tile { appearance:*ch, con_east: false, con_north:true, con_south:false, con_west:true, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            '7' => { Tile { appearance:*ch, con_east: false, con_north:false, con_south:true, con_west:true, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            '.' => { Tile { appearance:*ch, con_east: false, con_north:false, con_south:false, con_west:false, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            'S' => { Tile { appearance:*ch, con_east: false, con_north:false, con_south:false, con_west:false, 
                in_loop:false, south_in_loop:false, east_in_loop:false, } },
            _ => {panic!("fd")},
        }
    }

    pub fn is_tunnel(&self) -> bool {
        match self.appearance {
            'L' => { true },
            'F' => { true },
            '-' => { true },
            '|' => { true },
            'J' => { true },
            '7' => { true },
            '.' => { false },
            'S' => { true },
            _ => {panic!("fd")},
        }
    }
}

#[allow(dead_code)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    costs: Vec<Vec<i64>>,
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

            tile_row.push(Tile::from_ch(&'.'));
            tile_row.push(Tile::from_ch(&'.'));
            for ch in trimmed.chars() {
                tile_row.push(Tile::from_ch(&ch));
            }
            tile_row.push(Tile::from_ch(&'.'));
            tile_row.push(Tile::from_ch(&'.'));

            tiles.push(tile_row);
        }

        let mut pad_line: Vec<Tile> = Vec::new();
        for _ in 0..tiles.get(0).unwrap().len() {
            pad_line.push(Tile::from_ch(&'.'));
        }

        tiles.insert(0, pad_line.clone());
        tiles.insert(0, pad_line.clone());
        tiles.push(pad_line.clone());
        tiles.push(pad_line.clone());

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
            costs: costs,
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



    pub fn print_costs(&self, ) -> String {
        let mut toreturn = "".to_string();

        for line in &self.costs {
            let mut row = "   ".to_string();
            for cost in line {
                if *cost == i64::MAX {
                    row = format!("{:>2}  .", row);
                } else {
                    row = format!("{:>2} {:>2}", row, cost);
                }
            }
            // let chars: String = line.iter().map( |t| t.appearance ).collect();
            
            toreturn = format!("{}\n{}", toreturn, row);
        }
        toreturn.to_string()
    }

    
    pub fn find_creature(&self) -> Coord {
        for line_nr in 0..self.tiles.len() {
            let line = self.tiles.get(line_nr).unwrap();
            for col_nr in 0..line.len() {
                if line.get(col_nr).unwrap().appearance == 'S' {
                    return Coord { 
                        line_nr:line_nr,
                        col_nr: col_nr,
                    }
                }
            }
        }

        panic!("kfjdgfkjdgkfhdg")
    }

    pub fn propagate_costs(&mut self) {
        let mut worklist:Vec<Coord> = Vec::new();
        worklist.push(self.find_creature());

        while worklist.len() > 0 {
            let consider = worklist.pop().unwrap();
            if self.can_east(&consider) {
                let east = consider.at_east();
                if self.consider_new_cost(&east, self.cost_at(&consider)+1) {
                    worklist.push(east.clone());
                }
            }

            if self.can_south(&consider) {
                let south = consider.at_south();
                if self.consider_new_cost(&south, self.cost_at(&consider)+1){
                   worklist.push(south.clone());
                }
            }

            if self.can_west(&consider) {
                let west = consider.at_west();
                if self.consider_new_cost(&west, self.cost_at(&consider)+1){
                    worklist.push(west.clone());
                }
            }

            if self.can_north(&consider) {
                let north = consider.at_north();
                if self.consider_new_cost(&north, self.cost_at(&consider)+1) {
                    worklist.push(north.clone());
                }
            }
        }        
    }

    fn consider_new_cost(&mut self, coord: &Coord, candidate_cost: i64) -> bool {
        let curr = self.cost_at(&coord);
        if candidate_cost < curr {
            let mut cost_line = self.costs.get(coord.line_nr).unwrap().clone();
            cost_line[coord.col_nr] = candidate_cost;
            self.costs[coord.line_nr] = cost_line;
            return true
        }
        false
    }

    fn cost_at(&self, coord: &Coord) -> i64 {
        let here = self.costs.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap();
        *here
    }
    
    fn tile_copy_at(&self, coord: &Coord) -> Tile {
        let here = self.tiles.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap();
        here.clone()
    }
    

    pub fn can_east(&self, coord: &Coord) -> bool {
        let here = self.tiles.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap();
        let can_exit = here.appearance == 'S' || here.con_east;
        
        let at_east = coord.at_east();
        let there = self.tiles.get(at_east.line_nr).unwrap().get(at_east.col_nr).unwrap();
        let can_enter = there.con_west;

        can_exit && can_enter
    }

    pub fn can_west(&self, coord: &Coord) -> bool {
        let here = self.tiles.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap();
        let can_exit = here.appearance == 'S' || here.con_west;
        
        let at_west = coord.at_west();
        let there = self.tiles.get(at_west.line_nr).unwrap().get(at_west.col_nr).unwrap();
        let can_enter = there.con_east;

        can_exit && can_enter
    }

    pub fn can_north(&self, coord: &Coord) -> bool {
        let here = self.tiles.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap();
        let can_exit = here.appearance == 'S' || here.con_north;
        
        let at_north = coord.at_north();
        let there = self.tiles.get(at_north.line_nr).unwrap().get(at_north.col_nr).unwrap();
        let can_enter = there.con_south;

        can_exit && can_enter
    }

    pub fn can_south(&self, coord: &Coord) -> bool {
        let here = self.tiles.get(coord.line_nr).unwrap().get(coord.col_nr).unwrap();
        let can_exit = here.appearance == 'S' || here.con_south;
        
        let at_south = coord.at_south();
        let there = self.tiles.get(at_south.line_nr).unwrap().get(at_south.col_nr).unwrap();
        let can_enter = there.con_north;

        can_exit && can_enter
    }

    pub fn farthest_steps(&self) -> (Coord, i64) {
        // A.k.a part1.
        let mut toreturn_coord = Coord{col_nr:0, line_nr:0};
        let mut toreturn = 0;

        for line_nr in 0..self.costs.len() {
            let line = self.costs.get(line_nr).unwrap();
            for col_nr in 0..line.len() {
                let cost = line.get(col_nr).unwrap();
                if *cost != i64::MAX && *cost > toreturn {
                    toreturn = *cost;
                    toreturn_coord.col_nr = col_nr;
                    toreturn_coord.line_nr = line_nr;
                }
            }
        }
        (toreturn_coord, toreturn)
    }

    pub fn into_reach_grid(&self) -> ReachGrid {
        let mut grid: Vec<Vec<Reach>> = Vec::new();

        let mut default_row: Vec<Reach> = Vec::new(); 
        for _ in 0..self.tiles.get(0).unwrap().len() {
            default_row.push(Reach { tile_here: None, tile_east:None , tile_west:None , tile_south:None , tile_north:None , counts_as: CountsAs::Tbd });
            default_row.push(Reach { tile_here: None, tile_east:None , tile_west:None , tile_south:None , tile_north:None , counts_as: CountsAs::Tbd });
        }
        default_row.pop();

        for _ in 0..self.tiles.len() {
            grid.push(default_row.clone());
            grid.push(default_row.clone());
        }
        grid.pop();

        let mut toreturn = ReachGrid { grid: grid };

        for row_nr in 0..self.tiles.len() {
            for col_nr in 0..self.tiles.get(0).unwrap().len() {
                // Non-offset.
                let coord_tile = Coord{line_nr:row_nr, col_nr:col_nr};

                let coord_reach  = Coord{line_nr:row_nr*2, col_nr:col_nr*2};
                let coord_reach_south  = Coord{line_nr:row_nr*2+1, col_nr:col_nr*2};
                let coord_reach_east  = Coord{line_nr:row_nr*2, col_nr:col_nr*2+1};

                let tile_here = self.tile_copy_at(&coord_tile);

                toreturn.set_tile(&coord_reach, tile_here.clone());
                if tile_here.in_loop {
                    toreturn.set_as_loop(&coord_reach);
                }

                if tile_here.east_in_loop {
                    toreturn.set_as_loop(&coord_reach_east);
                }

                if tile_here.south_in_loop {
                    toreturn.set_as_loop(&coord_reach_south);
                }
                
            }
        }
        
        toreturn
    }

    pub fn set_in_loop(&mut self, coord: &Coord, new_value: bool) {
        let line = self.tiles.get_mut(coord.line_nr).unwrap();
        let mut tile = line.get_mut(coord.col_nr).unwrap().clone();
        tile.in_loop = new_value;
        line[coord.col_nr] = tile;
    }

    pub fn set_east_in_loop(&mut self, coord: &Coord, new_value: bool) {
        let line = self.tiles.get_mut(coord.line_nr).unwrap();
        let mut tile = line.get_mut(coord.col_nr).unwrap().clone();
        tile.east_in_loop = new_value;
        line[coord.col_nr] = tile;
    }

    pub fn set_south_in_loop(&mut self, coord: &Coord, new_value: bool) {
        let line = self.tiles.get_mut(coord.line_nr).unwrap();
        let mut tile = line.get_mut(coord.col_nr).unwrap().clone();
        tile.south_in_loop = new_value;
        line[coord.col_nr] = tile;
    }

    fn propagate_loop(&mut self, backtrack_from: Coord) {
        for line_nr in 0..self.tiles.len() {
            for col_nr in 0..self.tiles.get(0).unwrap().len() {
                self.set_in_loop(&Coord { line_nr: line_nr, col_nr: col_nr }, false);
            }
        }

        let mut current_cost = self.cost_at(&backtrack_from);
        let mut worklist: Vec<Coord> = Vec::new();
        self.set_in_loop(&backtrack_from, true);
        worklist.push(backtrack_from);

        while worklist.len() > 0 {
            let mut next_worklist: Vec<Coord> = Vec::new();

            for to_prop in worklist.iter() {
                // println!("Considering coord {:?} with cost {}", to_prop, current_cost);
                let coord_east = to_prop.at_east();
                let cost_east = self.cost_at(&coord_east);
                if self.can_east(&to_prop) || self.can_west(&coord_east){
                    // println!(" .... perhaps coord east with cost {} works?", cost_east);
                    if cost_east == current_cost -1 {
                        self.set_in_loop(&coord_east, true);
                        self.set_east_in_loop(&to_prop, true);
                        next_worklist.push(coord_east);
                    }
                }


                let coord_south = to_prop.at_south();
                let cost_south = self.cost_at(&coord_south);
                if self.can_south(&to_prop) || self.can_north(&coord_south){
                    if cost_south == current_cost -1 {
                        self.set_in_loop(&coord_south, true);
                        self.set_south_in_loop(&to_prop, true);
                        next_worklist.push(coord_south);
                    }
                }


                let coord_west = to_prop.at_west();
                let cost_west = self.cost_at(&coord_west);
                if self.can_west(&to_prop) || self.can_east(&coord_west){
                    if cost_west == current_cost -1 {
                        self.set_in_loop(&coord_west, true);
                        self.set_east_in_loop(&coord_west, true);
                        next_worklist.push(coord_west);
                    }
                }


                let coord_north = to_prop.at_north();
                let cost_north = self.cost_at(&coord_north);
                if self.can_north(&to_prop) || self.can_south(&coord_north){
                    if cost_north == current_cost -1 {
                        self.set_in_loop(&coord_north, true);
                        self.set_south_in_loop(&coord_north, true);
                        next_worklist.push(coord_north);
                    }
                }


            }

            worklist.clear();
            worklist.extend(next_worklist);
            current_cost -= 1;
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
    pub fn at_north(&self) -> Coord {
        Coord { line_nr: self.line_nr -1 , col_nr: self.col_nr  }
    }

    pub fn at_east(&self) -> Coord {
        Coord { line_nr: self.line_nr, col_nr: self.col_nr + 1 }
    }

    pub fn at_south(&self) -> Coord {
        Coord { line_nr: self.line_nr+1, col_nr: self.col_nr  }
    }

    pub fn at_west(&self) -> Coord {
        Coord { line_nr: self.line_nr, col_nr: self.col_nr - 1 }
    }
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


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Reach {
    tile_here: Option<Tile>, // If reach maps to tile.
    tile_east: Option<Tile>, // These are if reach are between tiles.
    tile_west: Option<Tile>,
    tile_south: Option<Tile>,
    tile_north: Option<Tile>,
    counts_as: CountsAs,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CountsAs {
    Loop,     // +
    Outside,  // .
    Inside,   // I
    Tbd,      //    (blank) 
}

#[allow(dead_code)]
pub struct ReachGrid {
    grid: Vec<Vec<Reach>>,
}

#[allow(dead_code)]
impl ReachGrid {

    pub fn render(&self) -> String {
        let mut toreturn = format!("<ReachGrid({}, {})>\n", self.grid.len(), self.grid.get(0).unwrap().len());

        for line in &self.grid {
            let chars: String = line.iter().map( |r| match r.counts_as.clone() {
                CountsAs::Loop => '+',
                CountsAs::Outside => '.',
                CountsAs::Inside => 'I',
                CountsAs::Tbd => '?',
            }).collect();
            toreturn = format!("{}\n{}", toreturn, chars);
        }
        toreturn.trim().to_string()
    }

    pub fn render_compressed(&self) -> String {
        // Render compressed (i.e. only 25% of tiles).
        let rows_to_render:usize = (self.grid.len()-1)/2;
        let cols_to_render:usize = (self.grid.get(0).unwrap().len()-1)/2;

        let mut toreturn = format!("<ReachGridCompressed({}, {})>\n", self.grid.len(), self.grid.get(0).unwrap().len());
        for row_nr in 0..rows_to_render {
            let mut selected :Vec<CountsAs> = Vec::new();
            for col_nr in 0..cols_to_render {
                selected.push(self.grid.get(row_nr*2).unwrap().get(col_nr*2).unwrap().counts_as.clone());
            }
            let chars: String = selected.iter().map( |ca| match ca {
                CountsAs::Loop => '+',
                CountsAs::Outside => '.',
                CountsAs::Inside => 'I',
                CountsAs::Tbd => '?',
            }).collect();
            toreturn = format!("{}\n{}", toreturn, chars);

        }

        toreturn

    }

    pub fn set_tile(&mut self, coord: &Coord, tile: Tile) {
        let line = self.grid.get_mut(coord.line_nr).unwrap();
        let mut reach = line.get_mut(coord.col_nr).unwrap().clone();

        reach.tile_here = Some(tile);        
        line[coord.col_nr] = reach.clone();
        self.grid[coord.line_nr] = line.clone();
    }

    
    pub fn set_as_loop(&mut self, coord: &Coord) {
        let line = self.grid.get_mut(coord.line_nr).unwrap();
        let mut reach = line.get_mut(coord.col_nr).unwrap().clone();

        reach.counts_as = CountsAs::Loop;
        line[coord.col_nr] = reach.clone();
        self.grid[coord.line_nr] = line.clone();
    }

    pub fn eval_as_outside(&mut self, coord: &Coord) -> bool {
        // Potentially flip a coord from tbd to outside.
        // Return true if there is any change.
        let line = self.grid.get_mut(coord.line_nr).unwrap();
        let mut reach = line.get_mut(coord.col_nr).unwrap().clone();
        match reach.counts_as {
            CountsAs::Tbd => {
                reach.counts_as = CountsAs::Outside;
                line[coord.col_nr] = reach.clone();
                self.grid[coord.line_nr] = line.clone();
                true
            },
            _ => false,
        }
    }

    pub fn propagate_outside(&mut self) {
        let mut worklist: Vec<Coord> = Vec::new();

        let max_line = self.grid.len();
        let max_col = self.grid.get(0).unwrap().len();

        worklist.push(Coord { line_nr: 0, col_nr: 0 });
        while worklist.len() > 0 {
            let curr = worklist.pop().unwrap();
            if self.eval_as_outside(&curr) {

                if curr.col_nr > 0 {
                    worklist.push(curr.at_west());
                }

                if curr.col_nr < max_col - 1 {
                    worklist.push(curr.at_east());
                }


                if curr.line_nr > 0 {
                    worklist.push(curr.at_north());
                }

                if curr.line_nr < max_line - 1 {
                    worklist.push(curr.at_south());
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn part1(content: &str) -> i64 {
    
    -1
}

#[allow(dead_code)]
fn part2(content: &str) -> i64 {
    let mut grid = Grid::from_content(&content);
    grid.propagate_costs();

    let printed_costs = grid.print_costs();
    // println!(" == Got costs ==\n{}\n =========", printed_costs);


    let (furthest, _) = grid.farthest_steps();
    grid.propagate_loop(furthest);
    let mut reachability = grid.into_reach_grid();
    reachability.propagate_outside();
    let rendered_reach = reachability.render();
    // println!(" == Rendered reach ==\n{}\n =========", rendered_reach);

    let rendered_reach = reachability.render_compressed();
    // println!(" == Rendered compressed reach ==\n{}\n =========", rendered_reach);

    let mut toreturn = 0;
    for char in rendered_reach.chars() {
        if char == '?' {
            toreturn += 1
        }
    }


    // println!("part2() from compressed render: {} gave {} as toreturn", rendered_reach, toreturn);
    toreturn
    

}



#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d10::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_render() {
        let pbuf = input::get_input("2023_d10_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut grid = Grid::from_content(&content);
        let printed_map = grid.print_glyphs();
        println!(" == Got map ==\n{}\n =========", printed_map);

        let printed_costs = grid.print_costs();
        println!(" == Got costs ==\n{}\n =========", printed_costs);

        grid.propagate_costs();

        let printed_costs = grid.print_costs();
        println!(" == Got costs ==\n{}\n =========", printed_costs);

    }

    #[test]
    fn test_encloses() {
        let pbuf = input::get_input("2023_d10_encloses_8.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut grid = Grid::from_content(&content);
        grid.propagate_costs();

        let printed_costs = grid.print_costs();
        println!(" == Got costs ==\n{}\n =========", printed_costs);

        let pbuf = input::get_input("2023_d10_encloses_10.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut grid = Grid::from_content(&content);
        grid.propagate_costs();

        let printed_costs = grid.print_costs();
        println!(" == Got costs ==\n{}\n =========", printed_costs);


        let (furthest, _) = grid.farthest_steps();
        grid.propagate_loop(furthest);
        let mut reachability = grid.into_reach_grid();
        reachability.propagate_outside();
        let rendered_reach = reachability.render();
        println!(" == Rendered reach ==\n{}\n =========", rendered_reach);

        let rendered_reach = reachability.render_compressed();
        println!(" == Rendered compressed reach ==\n{}\n =========", rendered_reach);
        
        
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d10::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d10_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut grid = Grid::from_content(&content);
            grid.propagate_costs();
            let (_, steps) = grid.farthest_steps();
            assert_eq!(steps, 8); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d10.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut grid = Grid::from_content(&content);
            grid.propagate_costs();
            let (_, steps) = grid.farthest_steps();
            assert_eq!(steps, 7063); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d10_encloses_8.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 8); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d10_encloses_10.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 10); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d10.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 589); // p2 skarp
        }
    }
}

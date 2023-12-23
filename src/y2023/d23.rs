use std::{collections::{HashSet, VecDeque, HashMap}, cmp::Ordering};
use std::fmt;



#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tile {
    appearance: char,
    tile_type: TileType,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Path,
    Forest,
    SlopeN,
    SlopeE,
    SlopeS,
    SlopeW,
}


#[allow(dead_code)]
impl Tile {
    pub fn from_ch(ch: &char) -> Tile {
        match ch {
            '.' => { Tile { appearance:*ch, tile_type: TileType::Path }},
            '#' => { Tile { appearance:*ch, tile_type: TileType::Forest }},
            '^' => { Tile { appearance:*ch, tile_type: TileType::SlopeN }},
            '>' => { Tile { appearance:*ch, tile_type: TileType::SlopeE }},
            'v' => { Tile { appearance:*ch, tile_type: TileType::SlopeS }},
            '<' => { Tile { appearance:*ch, tile_type: TileType::SlopeW }},
            _ => {
                println!("Tile {} unknown.", ch);
                panic!("fd")
            },
        }
    }

    pub fn exit_coords(&self, origin: &Coord) -> Vec<Coord> {
        match self.tile_type {
            TileType::Path => { 
                Vec::from_iter([
                    Coord{line_nr: origin.line_nr+1, col_nr: origin.col_nr},
                    Coord{line_nr: origin.line_nr-1, col_nr: origin.col_nr},
                    Coord{line_nr: origin.line_nr, col_nr: origin.col_nr+1},
                    Coord{line_nr: origin.line_nr, col_nr: origin.col_nr-1},
                ])
            },
            TileType::Forest => { panic!("you should not be here") },
            TileType::SlopeN => { Vec::from_iter([ Coord{line_nr: origin.line_nr-1, col_nr: origin.col_nr} ]) },
            TileType::SlopeE => { Vec::from_iter([ Coord{line_nr: origin.line_nr, col_nr: origin.col_nr+1} ]) },
            TileType::SlopeS => { Vec::from_iter([ Coord{line_nr: origin.line_nr+1, col_nr: origin.col_nr} ]) },
            TileType::SlopeW => { Vec::from_iter([ Coord{line_nr: origin.line_nr, col_nr: origin.col_nr-1} ]) },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Leg {
    start: Coord,
    end: Coord,
    steps_in_leg: usize,
    
    /* Vec<(Coord, Coord, usize)> */
}

#[allow(dead_code)]
pub struct Park {
    tiles: Vec<Vec<Tile>>,
    crossroads: Option<HashSet<Coord>>,
    start: Option<Coord>,
    end: Option<Coord>,
    find_legs_from_memoized: HashMap<Coord, Vec<Leg>>,

    p2set: HashSet<Coord>,
    p2stack: Vec<Coord>,
    p2steps: usize,
    p2step_record: usize,
}

#[allow(dead_code)]
impl Park {
    pub fn from_content(content: &str) -> Park {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut tiles:Vec<Vec<Tile>> = Vec::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let mut tile_row: Vec<Tile> = Vec::new();

            tile_row.push(Tile::from_ch(&'#'));
            tile_row.push(Tile::from_ch(&'#'));
            for ch in trimmed.chars() {
                tile_row.push(Tile::from_ch(&ch));
            }
            tile_row.push(Tile::from_ch(&'#'));
            tile_row.push(Tile::from_ch(&'#'));

            tiles.push(tile_row);
        }

        let mut pad_line: Vec<Tile> = Vec::new();
        for _ in 0..tiles.get(0).unwrap().len() {
            pad_line.push(Tile::from_ch(&'#'));
        }

        tiles.insert(0, pad_line.clone());
        tiles.insert(0, pad_line.clone());
        tiles.push(pad_line.clone());
        tiles.push(pad_line.clone());

        let find_legs_from_memoized: HashMap<Coord, Vec<Leg>> = HashMap::new();
        let mut toreturn = Park { 
            tiles: tiles,
            crossroads: None,
            start: None,
            end:None,
            find_legs_from_memoized,

            p2set: HashSet::new(),
            p2stack: Vec::new(),
            p2steps: 0,
            p2step_record: 0,
        };

        toreturn.crossroads = Some(toreturn.find_crossroads());
        toreturn.start = Some(toreturn.find_start());
        toreturn.end = Some(toreturn.find_end());

        toreturn
    }

    pub fn print_glyphs(&mut self) -> String {

        let mut appearances: Vec<Vec<char>> = self.tiles.iter().map(|tile_row| 
            {
                let chars = tile_row.iter().map( |t| t.appearance);
                chars.collect()
            }
        ).collect();

        println!("appearances is {:?}", appearances);
        for coord in self.find_crossroads() {
            appearances[coord.line_nr][coord.col_nr] = 'X';
        }

        let lines: Vec<String> = appearances.iter().map( |char_row| {
            let line_as_str: String = char_row.iter().collect();
            line_as_str
        }).collect();

        lines.join("\n").to_string()
    }

    pub fn ttype_at(&self, loc: &Coord) -> TileType {
        self.tiles[loc.line_nr][loc.col_nr].tile_type.clone()
    }

    pub fn enterable_neighs(&self, origin: &Coord) -> Vec<Coord> {
        let origin_type = self.tiles[origin.line_nr][origin.col_nr].tile_type.clone();
        match origin_type {
            TileType::Path => { 
                let coord_north = origin.at_north();
                let coord_south = origin.at_south();
                let coord_east = origin.at_east();
                let coord_west = origin.at_west();
                
                let ttype_north = self.ttype_at(&coord_north);
                let ttype_south = self.ttype_at(&coord_south);
                let ttype_east = self.ttype_at(&coord_east);
                let ttype_west = self.ttype_at(&coord_west);

                let mut toreturn: Vec<Coord> = Vec::new();

                if ttype_north != TileType::Forest && ttype_north != TileType::SlopeS {
                    // Do not walk into forest or opposing slope.
                    toreturn.push(coord_north);
                }

                if ttype_south != TileType::Forest && ttype_south != TileType::SlopeN {
                    // Do not walk into forest or opposing slope.
                    toreturn.push(coord_south);
                }

                if ttype_east != TileType::Forest && ttype_east != TileType::SlopeW {
                    // Do not walk into forest or opposing slope.
                    toreturn.push(coord_east);
                }

                if ttype_west != TileType::Forest && ttype_west != TileType::SlopeE {
                    // Do not walk into forest or opposing slope.
                    toreturn.push(coord_west);
                }

                toreturn

            },
            TileType::Forest => { panic!("you should not be here") },
            TileType::SlopeN => { Vec::from_iter([ Coord{line_nr: origin.line_nr-1, col_nr: origin.col_nr} ]) },
            TileType::SlopeE => { Vec::from_iter([ Coord{line_nr: origin.line_nr, col_nr: origin.col_nr+1} ]) },
            TileType::SlopeS => { Vec::from_iter([ Coord{line_nr: origin.line_nr+1, col_nr: origin.col_nr} ]) },
            TileType::SlopeW => { Vec::from_iter([ Coord{line_nr: origin.line_nr, col_nr: origin.col_nr-1} ]) 
            },
        }

    }

    pub fn is_a_crossroad(&self, candidate: &Coord) -> bool {
        if let Some(xroads) = &self.crossroads {
            return xroads.contains(candidate)
        }
        panic!("jgldhfjklgfd")
    }

    pub fn follow_path(&self, origin: &Coord, next_step: &Coord) -> Vec<Coord> {
        if self.is_start_coord(origin) || self.is_a_crossroad(origin) {
            // Ok.
        } else {
            println!("Refusing to try and follow path starting at {}", origin);
            panic!("fldh")
        }


        let mut visited: Vec<Coord> = Vec::new();
        let mut worklist: VecDeque<Coord> = VecDeque::new();

        visited.push(origin.clone());
        worklist.push_back(next_step.clone());

        while worklist.len() > 0 {            
            if worklist.len() > 1 {
                println!("follow_path expected to split at crossroads, refusing to continue. origin={:?}, next_step={:?}, visited={:?}, worklist={:?}", origin, next_step, visited, worklist);
            }

            let curr = worklist.pop_front().unwrap();
            visited.push(curr.clone());
            
            let mut enterable_neighs = self.enterable_neighs(&curr);
            let mut valid_neighs: Vec<Coord> = Vec::new();
            while enterable_neighs.len() > 0 {
                let cand = enterable_neighs.pop().unwrap();
                if visited.contains(&cand) {
                    continue;
                }
                valid_neighs.push(cand);
            }             

            if valid_neighs.len() == 0 {
                // Dead end.
            }  else if valid_neighs.len() == 1 {
                let neigh_coord = &valid_neighs[0];
                if self.is_end_coord(neigh_coord) {
                    // End of segment is end of maze. No more worklist.
                    visited.push(neigh_coord.clone())
                } else if self.crossroads.as_ref().unwrap().contains(neigh_coord) {
                    // End of segment is a crossroads. No more worklist.
                    visited.push(neigh_coord.clone())
                } else {
                    // Keep walking the path.
                    worklist.push_back(neigh_coord.clone());
                }
            } else {
                println!("follow_path({}, {}) valid_neighs makes no sense. Got {:?} for {} with visited {:?}", origin, next_step, valid_neighs, curr, visited);
                panic!("follow_path failed")
            }
        }


        // println!("follow_path() done: {:?}", visited);
        visited
    }


    pub fn find_legs_from(&mut self, origin: &Coord) -> Vec<Leg> { //  -> Vec<(Coord, Coord, usize)> {
        if self.find_legs_from_memoized.contains_key(origin) {
            return self.find_legs_from_memoized.get(origin).unwrap().clone()
        }

        // From one crossroads or starting point, where can we go?
        let mut toreturn: Vec<Leg> = Vec::new();

        for dir in self.enterable_neighs(origin) {
            if self.tiles[dir.line_nr][dir.col_nr].tile_type != TileType::Forest {
                let path = self.follow_path(origin, &dir);
                let leg = Leg {
                    start: origin.clone(),
                    end:path[path.len()-1].clone(),
                    steps_in_leg: path.len()-1,
                };
                toreturn.push(leg);
            }
        }

        self.find_legs_from_memoized.insert(origin.clone(), toreturn.clone());
        toreturn
    }

    pub fn is_start_coord(&self, candidate: &Coord) -> bool {
        if let Some(start_coord) = &self.start {
            start_coord.line_nr == candidate.line_nr &&
            start_coord.col_nr == candidate.col_nr
        } else {
            panic!("start_coord not populated!")
        }
    }

    pub fn is_end_coord(&self, candidate: &Coord) -> bool {
        if let Some(end_coord) = &self.end {
            end_coord.line_nr == candidate.line_nr &&
            end_coord.col_nr == candidate.col_nr
        } else {
            panic!("end_coord not populated!")
        }
    }

    pub fn find_crossroads(&mut self) -> HashSet<Coord> {
        if let Some(toreturn) = &self.crossroads {
            return toreturn.clone()
        }

        let mut toreturn: HashSet<Coord> = HashSet::new();
        for line_nr in 1..self.tiles.len()-2 {
            for col_nr in 1..self.tiles[line_nr].len()-2 {
                if self.tiles[line_nr][col_nr].tile_type != TileType::Path {
                    continue;
                }

                let mut neigh_path_count = 0;
                if self.tiles[line_nr-1][col_nr].tile_type != TileType::Forest {
                    neigh_path_count += 1;
                }

                if self.tiles[line_nr+1][col_nr].tile_type != TileType::Forest {
                    neigh_path_count += 1;
                }

                if self.tiles[line_nr][col_nr-1].tile_type != TileType::Forest {
                    neigh_path_count += 1;
                }

                if self.tiles[line_nr][col_nr+1].tile_type != TileType::Forest {
                    neigh_path_count += 1;
                }
 
                if neigh_path_count >= 3 {
                    toreturn.insert(Coord { line_nr, col_nr });
                }

            }
        }
        self.crossroads = Some(toreturn.clone());
        toreturn
    }

    pub fn find_start(&self) -> Coord {
        if let Some(toreturn) = &self.start {
            return toreturn.clone()
        }

        for n in 0..self.tiles[2].len() {
            if self.tiles[2][n].tile_type == TileType::Path {
                return Coord{ line_nr:2, col_nr:n }
            }
        }
        panic!("fd")
    }

    pub fn find_end(&self) -> Coord {
        if let Some(toreturn) = &self.end {
            return toreturn.clone()
        }

        let line_nr = self.tiles.len()-3;
        for n in 0..self.tiles[line_nr].len() {
            if self.tiles[line_nr][n].tile_type == TileType::Path {
                return Coord{ line_nr:line_nr, col_nr:n }
            }
        }
        panic!("fd")
    }

    pub fn not_worklist(&mut self) -> i64 {
        self.p2set.clear();
        self.p2stack.clear();
        self.p2steps = 0;
        self.p2step_record = 0;

        self.p2set.insert(self.find_start());
        self.p2stack.push(self.find_start());

        self.not_worklist_recur();

        self.p2step_record.try_into().unwrap()
    }

    pub fn not_worklist_recur(&mut self) -> i64 {
        // Recur from end of p2stack.
        let end = self.p2stack[self.p2stack.len()-1].clone();
        let legs = self.find_legs_from(&end);
        for leg in legs {
            if self.p2set.contains(&leg.end) {
                continue;
            }

            if self.is_end_coord(&leg.end) {
                self.p2steps += leg.steps_in_leg;
                if self.p2steps > self.p2step_record {
                    println!("Got new record {}", self.p2steps);
                    self.p2step_record = self.p2steps;
                }

                self.p2steps -= leg.steps_in_leg;
            } else {
                self.p2set.insert(leg.end.clone());
                self.p2stack.push(leg.end.clone());
                self.p2steps += leg.steps_in_leg;

                self.not_worklist_recur();

                self.p2steps -= leg.steps_in_leg;
                self.p2set.remove(&leg.end);
                self.p2stack.pop();
            }
        }

        -1
    }

    pub fn part1(&mut self) -> i64 {
        // This takes > 45m for part2. Rewrite as not_worklist().
        let mut longest: usize = 0;
        let mut worklist: Vec<HikeOption> = Vec::new();
        worklist.push(HikeOption::new(self.find_start(), self.find_end()));

        while worklist.len() > 0 {
            let hike = worklist.pop().unwrap();
            if hike.is_at_end() {
                // println!("Got hike of len {}: {:?}", hike.steps_total, hike.waypoints);
                if hike.steps_total > longest {
                    println!("Got valid hike of len {}", hike.steps_total);
                    longest = longest.max(hike.steps_total.try_into().unwrap());
                }
                continue;
            }

            let hike_end = hike.waypoints[hike.waypoints.len()-1].clone();
            let legs = self.find_legs_from(&hike_end);
            for leg in legs {
                
                if let Some(continuation) = hike.maybe_follow_leg(leg.start, leg.end, leg.steps_in_leg) {
                    worklist.push(continuation);
                }
            }

            worklist.sort_by( |a, b| 
                if a.fill_rate < b.fill_rate {
                    Ordering::Less
                } else if a.fill_rate > b.fill_rate {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            );
        }
        longest.try_into().unwrap()
    }


}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct HikeOption {
    maze_end: Coord,
    waypoints: Vec<Coord>,
    steps_total: usize,
    fill_rate: f64,
}

impl HikeOption {

    #[allow(dead_code)]
    pub fn new(maze_start: Coord, maze_end: Coord) -> HikeOption {
        HikeOption { 
            maze_end,
            waypoints: Vec::from_iter([maze_start]),
            steps_total: 0,
            fill_rate: 0.0,
        }
    }

    pub fn update_fill_rate(&mut self) {
        let tail = &self.waypoints[self.waypoints.len()-1];
        let filled:f64 = f64::try_from(i32::try_from(self.steps_total).unwrap()).unwrap();
        let area:f64 = f64::try_from(i32::try_from(tail.col_nr * tail.line_nr).unwrap()).unwrap(); 
        self.fill_rate = filled/area;
    }

    pub fn is_at_end(&self) -> bool {
        return self.waypoints[self.waypoints.len()-1] == self.maze_end
    }

    pub fn maybe_follow_leg(&self, start: Coord, end: Coord, added_steps: usize) -> Option<HikeOption> {
        if start.line_nr != self.waypoints[self.waypoints.len()-1].line_nr ||
            start.col_nr != self.waypoints[self.waypoints.len()-1].col_nr {
            panic!("refusing to follow leg that does not step off from current location");
        }

        if self.is_at_end() {
            None
        } else if self.waypoints.contains(&end) {
            None
        } else {
            let mut toreturn = self.clone();
            toreturn.waypoints.push(end);
            toreturn.steps_total += added_steps;
            toreturn.update_fill_rate();
            Some(toreturn)
        }
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.line_nr, self.col_nr)
    }
}


#[allow(dead_code)]
pub fn part1(content: &str) -> i64 {
    let mut garden = Park::from_content(&content);
    garden.part1()
}

#[allow(dead_code)]
fn part2(content: &str) -> i64 {
    let dry_content = content.replace(">", ".")
    .replace("<", ".")
    .replace("^", ".")
    .replace("v", ".");
    let mut garden = Park::from_content(&dry_content);
    let xroads = garden.find_crossroads();
    println!("Part 2 got {} crossroads.", xroads.len());
    // garden.part1()
    let toreturn = garden.not_worklist();
    println!("Part 2 got {} result.", toreturn);
    toreturn
}



#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d23::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_render() {
        let pbuf = input::get_input("2023_d23_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut garden = Park::from_content(&content);
        let printed_map = garden.print_glyphs();
        println!(" == Got map ==\n{}\n =========", printed_map);

        assert_eq!(garden.find_start(), Coord{ line_nr:2, col_nr: 3});
        assert_eq!(garden.find_end(), Coord{ line_nr:24, col_nr: 23});
    }


    #[test]
    fn find_segments_from() {
        let pbuf = input::get_input("2023_d23_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut garden = Park::from_content(&content);
        let segments = garden.find_legs_from(&Coord { line_nr: 2, col_nr: 3 });
        assert_eq!(1, segments.len());
        let segments = garden.find_legs_from(&Coord { line_nr: 7, col_nr: 5 });
        assert_eq!(2, segments.len());
    }


    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d23::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d23_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content);
            assert_eq!(steps, 94); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d23.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content);
            assert_eq!(steps, 2010); // p1 skarp
        }

        {
            let pbuf = input::get_input("2023_d23_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part2(&content);
            assert_eq!(steps, 154); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d23.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert!(actual > 5914);
            assert!(actual > 6022);
            assert!(actual >= 6182); // Brute force using worklist.
            
            assert_eq!(actual, 6318); // p2 skarp
        }
    }
}

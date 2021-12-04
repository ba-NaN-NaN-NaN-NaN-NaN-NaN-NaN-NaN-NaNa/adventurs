use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

// Compass directions in a 2d grid.
pub enum GridDirs {
    North,
    East,
    South,
    West,
}

pub struct GridWalker {
    origin_x: i64,
    origin_y: i64,
    curr_x: i64,
    curr_y:i64,
    heading: GridDirs,
}


pub trait Walking {
    fn ingest_moves(&mut self, moves: &str);
    fn ingest_a_move(&mut self, m: &str);
    fn apply_a_move(&mut self, action: &str, count: i64);
}

impl Walking for GridWalker {
    fn ingest_moves(&mut self, moves: &str) {
        let owned = String::from(moves);
        let parts = owned.split("\n");
        for mut part in parts {
            // print!("Got a part {}\n", part);
            part = part.trim();
            if part.len() == 0 {
                continue
            }
            self.ingest_a_move(part);
        }
    }

    fn ingest_a_move(&mut self, m: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([NESWFLR])(\d+)$").unwrap();
        }

        let m2 = m.trim();

        match RE.captures(m2) {
            Some(t) => {
                let action = &t[1];
                let count = &t[2];
                let n = i64::from_str(count).unwrap();
                self.apply_a_move(action, n)
            } 
            None => println!("PARSE ERROR FOR A MOVE '{}'", m2)
        }
    }

    fn apply_a_move(&mut self, action: &str, count: i64) {
        if count == 0 {
            return
        }
        match action {
            "N" => self.curr_y += count,
            "E" => self.curr_x += count,
            "S" => self.curr_y -= count,
            "W" => self.curr_x -= count,

            // Remap forward into cardinal direction move.
            "F" => {
                let dir = match &self.heading {
                    GridDirs::North => "N",
                    GridDirs::East => "E",
                    GridDirs::South => "S",
                    GridDirs::West => "W",
                };
                self.apply_a_move(dir, count)
            }

            "R" => { match &self.heading {
                GridDirs::North => self.heading = GridDirs::East,
                GridDirs::East => self.heading = GridDirs::South,
                GridDirs::South => self.heading = GridDirs::West,
                GridDirs::West => self.heading = GridDirs::North,
                };
                self.apply_a_move("R", count-90)
            }
            "L" => { match &self.heading {
                    GridDirs::North => self.heading = GridDirs::West,
                    GridDirs::East => self.heading = GridDirs::North,
                    GridDirs::South => self.heading = GridDirs::East,
                    GridDirs::West => self.heading = GridDirs::South,
                };
                self.apply_a_move("L", count-90)
            }

            _ => println!("Unable to apply action {} with count {}", action, count)
        }
    }
}

impl GridWalker {
    #[allow(dead_code)]
    pub fn new(origin_x: i64, origin_y: i64) -> GridWalker {
        GridWalker{
            origin_x: origin_x,
            origin_y: origin_y,
            curr_x: origin_x,
            curr_y: origin_y,
            heading: GridDirs::East,
        }
    }

    #[allow(dead_code)]
    pub fn manhattan_dist_origin(&self) -> i64 {
        (self.origin_x - self.curr_x).abs() + (self.origin_y - self.curr_y).abs()
    }
}


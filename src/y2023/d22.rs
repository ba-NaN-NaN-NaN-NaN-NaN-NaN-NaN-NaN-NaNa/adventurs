use std::iter;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Brick {
    label: String,
    x_lower: usize,
    x_upper: usize,
    y_lower: usize,
    y_upper: usize,
    z_lower: usize,
    z_upper: usize,
    has_fallen: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BrickStack {
    bricks: Vec<Brick>,
}


#[allow(dead_code)]
impl Brick {
    fn from_line(input: &str) -> Brick  {
        let trimmed = input.trim().replace("~", ",");
        let frags: Vec<&str> = trimmed.split(",").collect();

        // println!("frags is {:?}", frags);
        return Brick { 
            label: "#".to_string(),
            x_lower: frags[0].parse::<usize>().unwrap(),
            y_lower: frags[1].parse::<usize>().unwrap(),
            z_lower: frags[2].parse::<usize>().unwrap(),

            x_upper: frags[3].parse::<usize>().unwrap(),
            y_upper: frags[4].parse::<usize>().unwrap(),
            z_upper: frags[5].parse::<usize>().unwrap(),

            has_fallen: false,
        }
    }

    pub fn volume(&self) -> usize {
        let toreturn: usize = 
        (self.x_upper - self.x_lower + 1) *
        (self.y_upper - self.y_lower + 1) *
        (self.z_upper - self.z_lower + 1);
        toreturn.try_into().unwrap()
    }
}



impl BrickStack {

    #[allow(dead_code)]
    pub fn from_content(input: &str) -> BrickStack {
        let mut bricks : Vec<Brick> = Vec::new();

        for line in input.split("\n") {
            if line.trim().len() == 0 {
                 continue;
            }
 
            let brick = Brick::from_line(line.trim());
            bricks.push(brick);
        }

        bricks[0].label = "A".to_string();
        bricks[1].label = "B".to_string();
        bricks[2].label = "C".to_string();
        bricks[3].label = "D".to_string();
        bricks[4].label = "E".to_string();
        bricks[5].label = "F".to_string();
        bricks[6].label = "G".to_string();


        BrickStack { 
            bricks: bricks,
        }
    }

    #[allow(dead_code)]
    pub fn draw_from_y(&self) -> String {
        // col_nr is x.
        // row_nr is z, reversed.

        let max_x: usize = self.bricks.iter().fold(0, |max_so_far, brick| {
            max_so_far.max(brick.x_upper)
        });
        let max_z: usize = self.bricks.iter().fold(0, |max_so_far, brick| {
            max_so_far.max(brick.z_upper)
        });

        let row: Vec<String> = iter::repeat(".").take(max_x+1).map(|elem| elem.to_string()).collect();
        let mut grid: Vec<Vec<String>> = iter::repeat(row).take(max_z+1).collect();

        for x in 0..=max_x {
            grid[max_z][x] = "-".to_string();
        }
        for brick in self.bricks.iter() {
            for x in brick.x_lower..=brick.x_upper {
                for z in brick.z_lower..=brick.z_upper  {
                    if grid[max_z-z][x] == "." {
                        grid[max_z-z][x] = brick.label.clone();
                    } else {
                        grid[max_z-z][x] = "?".to_string();
                    }
                    
                }
            }
        }

        // println!("Grid is now {:?}", grid);

        let drawn_rows: Vec<String> = grid.iter().map(|row| row.join("")).collect();
        return drawn_rows.join("\n").to_string();
    }


    #[allow(dead_code)]
    pub fn sort_bricks(&mut self) {
        self.bricks.sort_by(|a, b| {
            if a.z_lower != b.z_lower {
                return a.z_lower.cmp(&b.z_lower)
            } 

            if a.x_lower != b.x_lower {
                return a.x_lower.cmp(&b.x_lower)
            } 

            if a.y_lower != b.y_lower {
                return a.y_lower.cmp(&b.y_lower)
            } 

            if a.z_upper != b.z_upper {
                return a.z_upper.cmp(&b.z_upper)
            } 

            if a.x_upper != b.x_upper {
                return a.x_upper.cmp(&b.x_upper)
            } 

            return a.y_upper.cmp(&b.y_upper)
        });
    }


    pub fn brick_is_supported(&self, brick_nr: usize) -> bool {
        let layer_to_check: usize = self.bricks[brick_nr].z_lower;
        if layer_to_check == 1 {
            // Bricks right above layer 0 are always supported by layer 0.
            return true
        }

        let own_x_lower = self.bricks[brick_nr].x_lower;
        let own_x_upper = self.bricks[brick_nr].x_upper;
        let own_y_lower = self.bricks[brick_nr].y_lower;
        let own_y_upper = self.bricks[brick_nr].y_upper;
    

        for brick_nr_other in 0..self.bricks.len() {
            if brick_nr == brick_nr_other {
                // Do not check brick with itself.
                continue;
            }

            let brick_other = &self.bricks[brick_nr_other];

            if brick_other.z_upper != layer_to_check -1 {    
                // Upper layer of other brick must be lower of our own -1.
                continue;
            }

            if own_x_lower > brick_other.x_upper {
                continue;
            }

            if own_y_lower > brick_other.y_upper {
                continue;
            }

            if own_x_upper < brick_other.x_lower {
                continue;
            }

            if own_y_upper < brick_other.y_lower {
                continue;
            }
            
            return true // This other brick supports us.
        }
        false
    }

    #[allow(dead_code)]
    pub fn apply_gravity(&mut self) {
        self.sort_bricks();
        let mut done = false;
        while !done {
            done = true;
            for brick_nr in 0..self.bricks.len() {
                if !self.brick_is_supported(brick_nr) {
                    self.bricks[brick_nr].z_lower -=1;
                    self.bricks[brick_nr].z_upper -=1;
                    self.bricks[brick_nr].has_fallen = true;
                    done = false;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn can_disintegrate(&mut self, brick_nr: usize) -> bool {
        let brick_z_upper = self.bricks[brick_nr].z_upper;
        let mut candidate_supported_brick_nrs: Vec<usize> = Vec::new();
        for candidate_brick_nr in 0..self.bricks.len() {
            if candidate_brick_nr == brick_nr {
                continue;
            }

            if self.bricks[candidate_brick_nr].z_lower == brick_z_upper + 1 {
                candidate_supported_brick_nrs.push(candidate_brick_nr);
            }
        }

        let delta = usize::MAX/2;
        self.bricks[brick_nr].z_lower += delta;
        self.bricks[brick_nr].z_upper += delta; // Test what happens if we move our brick

        for cand_nr in candidate_supported_brick_nrs.iter() {
            if !self.brick_is_supported(*cand_nr) {
                // NOPE, disintegrating will make another brick unsupported.
                self.bricks[brick_nr].z_lower -= delta;
                self.bricks[brick_nr].z_upper -= delta;
                return false
            }
        }

        // Restore the move.
        self.bricks[brick_nr].z_lower -= delta;
        self.bricks[brick_nr].z_upper -= delta; 
        true
    }

    #[allow(dead_code)]
    pub fn count_destroyable_bricks(&mut self) -> i64 {
        let mut toreturn = 0;

        for brick_nr in 0..self.bricks.len() {
            if self.can_disintegrate(brick_nr) {
                toreturn += 1;
            }
        }

        toreturn
    }

    #[allow(dead_code)]
    pub fn count_fallen_bricks(&self) -> i64 {
        let mut toreturn = 0;
        for brick in self.bricks.iter() {
            if brick.has_fallen {
                toreturn += 1;
            }
        }        
        toreturn
    }

    #[allow(dead_code)]
    pub fn reset_fallen(&mut self) {        
        for brick in self.bricks.iter_mut() {
            brick.has_fallen = false;
        }       
    }

    #[allow(dead_code)]
    pub fn draw_from_x(&self) -> String {
        // col_nr is x.
        // row_nr is z, reversed.

        let max_y: usize = self.bricks.iter().fold(0, |max_so_far, brick| {
            max_so_far.max(brick.y_upper)
        });
        let max_z: usize = self.bricks.iter().fold(0, |max_so_far, brick| {
            max_so_far.max(brick.z_upper)
        });

        let row: Vec<String> = iter::repeat(".").take(max_y+1).map(|elem| elem.to_string()).collect();
        let mut grid: Vec<Vec<String>> = iter::repeat(row).take(max_z+1).collect();

        for y in 0..=max_y {
            grid[max_z][y] = "-".to_string();
        }
        for brick in self.bricks.iter() {
            for y in brick.y_lower..=brick.y_upper {
                for z in brick.z_lower..=brick.z_upper  {
                    if grid[max_z-z][y] == "." {
                        grid[max_z-z][y] = brick.label.clone();
                    } else {
                        grid[max_z-z][y] = "?".to_string();
                    }
                }
            }
        }

        // println!("Grid is now {:?}", grid);

        let drawn_rows: Vec<String> = grid.iter().map(|row| row.join("")).collect();
        return drawn_rows.join("\n").to_string();
    }


 
}


#[allow(dead_code)]
pub fn part1(content: &str) -> i64 {
    let mut stack = BrickStack::from_content(&content);
    stack.apply_gravity();
    stack.count_destroyable_bricks()
}

#[allow(dead_code)]
pub fn part2(content: &str) -> i64 {
    let mut stack = BrickStack::from_content(&content);
    stack.apply_gravity();

    let mut toreturn = 0;

    for brick_nr in 0..stack.bricks.len() {
        let mut cloned_stack = stack.clone();
        cloned_stack.bricks[brick_nr].z_lower = 1;
        cloned_stack.bricks[brick_nr].z_upper = 1;

        // Yeet, much easier than changing length of bricks vector.
        cloned_stack.bricks[brick_nr].x_lower = usize::MAX/2;
        cloned_stack.bricks[brick_nr].x_upper = usize::MAX/2;
        cloned_stack.bricks[brick_nr].y_lower = usize::MAX/2;
        cloned_stack.bricks[brick_nr].y_upper = usize::MAX/2;

        cloned_stack.reset_fallen();
        cloned_stack.apply_gravity();
        toreturn += cloned_stack.count_fallen_bricks();
    }
    
    toreturn
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d22::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_from_line() {
        let brick = Brick::from_line("0,0,10~0,1,10");
        assert_eq!(brick.volume(), 2);

        let brick = Brick::from_line("2,2,2~2,2,2");
        assert_eq!(brick.volume(), 1);
    }

    #[test]
    fn test_draw_from_y() {
        let pbuf = input::get_input("2023_d22_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut stack = BrickStack::from_content(&content);
        let drawn = stack.draw_from_y();
        assert_eq!(drawn, ".G.\n.G.\n...\nFFF\n..E\nD..\nCCC\nBBB\n.A.\n---");
        // println!(" ==v== Got drawn from y stack ==v==\n{}\n ==^========^===", drawn);
        let drawn = stack.draw_from_x();
        assert_eq!(drawn, ".G.\n.G.\n...\n.F.\nEEE\nDDD\n..C\nB..\nAAA\n---");
        // println!(" ==v== Got drawn from x stack ==v==\n{}\n ==^========^===", drawn);

        stack.apply_gravity();

        let drawn = stack.draw_from_y();
        assert_eq!(drawn, ".G.\n.G.\nFFF\nD.E\n???\n.A.\n---");
        // println!(" ==v== Got drawn from y stack ==v==\n{}\n ==^========^===", drawn);
        let drawn = stack.draw_from_x();
        assert_eq!(drawn, ".G.\n.G.\n.F.\n???\nB.C\nAAA\n---");
        // println!(" ==v== Got drawn from x stack ==v==\n{}\n ==^========^===", drawn);

    }
    
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d22::tests --nocapture


        {
            let pbuf = input::get_input("2023_d22_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 5); // p1 sample
        }
        if false {
            let pbuf = input::get_input("2023_d22.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 480); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d22_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 7); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d22.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 84021); // p2 skarp
        }
    }
}

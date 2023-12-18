#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Block {
    // One city block, a.k.a one digit from input.
    entry_cost: i64,
    lowest_known_from_ns: i64,
    lowest_known_from_ew: i64,
}

#[allow(dead_code)]
impl Block {
    pub fn from_input(content: char) -> Block {
        let parse_try = content.to_string().parse::<i64>();
        if parse_try.is_err() {
            let err = parse_try.err();
            {
                println!("Bad input '{}' gave parse error '{:?}'", content, err);
                panic!("Block::from_input() bad content on parse attempt");
            }
        }

        let local_cost = parse_try.unwrap();
        if local_cost < 0 || 9 < local_cost {
            println!("Bad input {}", content);
            panic!("Block::from_input() bad content");
        }
        
        return Block { 
            entry_cost:local_cost,
            lowest_known_from_ns: 999999999999999999,
            lowest_known_from_ew: 999999999999999999,
        }
    }
}

#[allow(dead_code)]
pub struct Grid {
    blocks: Vec<Vec<Block>>,
}

#[allow(dead_code)]
impl Grid {
    pub fn from_content(content: &str) -> Grid {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut blocks:Vec<Vec<Block>> = Vec::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {continue;}
            let chars = trimmed.chars();
            
            // println!("fragmended line '{}' -> '{:?}'", trimmed, frags);
            let city_line: Vec<Block> = chars.map(|f| Block::from_input(f) ).collect();
            blocks.push(city_line);
        }
        Grid { 
            blocks,
        }
    }

    pub fn print_glyphs(&self) -> String {
        let mut toreturn = "".to_string();

        for line in &self.blocks {
            let chars: String = line.iter().map( |t| t.entry_cost.to_string() ).collect();
            toreturn = format!("{}\n{}", toreturn, chars);
        }
        toreturn.trim().to_string()
    }

    pub fn print_costs(&self) -> String {
        let mut toreturn: Vec<String> = Vec::new();
        for line in &self.blocks {
            let mut frags: Vec<String> = Vec::new();
            for col in line {
                let best = col.lowest_known_from_ew.min(col.lowest_known_from_ns) % 1000;

                frags.push(format!("{:>3}", best));
            }
            toreturn.push(frags.join(" "));
        }
        toreturn.join("\n")
    }


    pub fn heat_loss_bottom_right(&self) -> i64 {
        self.blocks.last().unwrap().last().unwrap().lowest_known_from_ew.min(
            self.blocks.last().unwrap().last().unwrap().lowest_known_from_ns
        )
    }

    pub fn propagate_costs(&mut self, minsteps: usize, maxsteps: usize) {
        { // This code block limits scope we bottow self.blocks as mut.
            let start_block = self.blocks.get_mut(0).unwrap().get_mut(0).unwrap();
            start_block.lowest_known_from_ew = 0;
            start_block.lowest_known_from_ns = 0;
        }

        let city_height: usize = self.blocks.len();
        let city_width: usize = self.blocks.get(0).unwrap().len();        

        let mut worklist: Vec<Coord> = Vec::new();
        worklist.push(Coord { line_nr: 0, col_nr: 0 });

        while worklist.len() > 0 {
            let mut new_worklist: Vec<Coord> = Vec::new();
            for todo in worklist.iter() {
                let line_nr:usize = todo.line_nr.try_into().unwrap();
                let col_nr:usize = todo.col_nr.try_into().unwrap();
                let local_cost_ew = self.blocks
                                             .get(line_nr).unwrap()
                                             .get(col_nr).unwrap()
                                             .lowest_known_from_ew;

                let local_cost_ns = self.blocks
                                             .get(line_nr).unwrap()
                                             .get(col_nr).unwrap()
                                             .lowest_known_from_ns;

                let mut enroute_entry_costs:i64;

                // Check southwards
                enroute_entry_costs = 0;
                for step_count in 1..maxsteps+1 {
                    let new_line_nr = line_nr+step_count;
                    if new_line_nr >= city_height {
                        continue;
                    }

                    let at_south: &mut Block = self.blocks
                                                  .get_mut(new_line_nr).unwrap()
                                                  .get_mut(col_nr).unwrap();

                    enroute_entry_costs += at_south.entry_cost;

                    if step_count >= minsteps {
                        let candidate_cost = local_cost_ew + enroute_entry_costs;
                        
                        if at_south.lowest_known_from_ns > candidate_cost {
                            at_south.lowest_known_from_ns = candidate_cost;
                            new_worklist.push(Coord { line_nr: new_line_nr, col_nr: col_nr });
                        }
                    }
                }

                // Check eastwards
                enroute_entry_costs = 0;
                for step_count in 1..maxsteps+1 {
                    let new_col_nr = col_nr+step_count;
                    if new_col_nr >= city_width {
                        continue;
                    }

                    let at_east: &mut Block = self.blocks
                                                  .get_mut(line_nr).unwrap()
                                                  .get_mut(new_col_nr).unwrap();
                    enroute_entry_costs += at_east.entry_cost;

                    if step_count >= minsteps {
                        let candidate_cost = local_cost_ns + enroute_entry_costs;
                        
                        if at_east.lowest_known_from_ew > candidate_cost {
                            at_east.lowest_known_from_ew = candidate_cost;
                            new_worklist.push(Coord { line_nr: line_nr, col_nr: new_col_nr });
                        }
                    }
                }

                // Check westwards
                enroute_entry_costs = 0;
                for step_count in 1..maxsteps+1 {
                    if step_count > col_nr {
                    continue
                    }
                    let new_col_nr = col_nr-step_count;

                    let at_west: &mut Block = self.blocks
                                                  .get_mut(line_nr).unwrap()
                                                  .get_mut(new_col_nr).unwrap();
                    enroute_entry_costs += at_west.entry_cost;

                    if step_count >= minsteps {
                        let candidate_cost = local_cost_ns + enroute_entry_costs;
                        
                        if at_west.lowest_known_from_ew > candidate_cost {
                            at_west.lowest_known_from_ew = candidate_cost;
                            new_worklist.push(Coord { line_nr: line_nr, col_nr: new_col_nr });
                        }
                    }
                }



                // Check northwards
                enroute_entry_costs = 0;
                for step_count in 1..maxsteps+1 {
                    if step_count > line_nr {
                        continue;
                    }
                    let new_line_nr = line_nr-step_count;

                    let at_north: &mut Block = self.blocks
                                                  .get_mut(new_line_nr).unwrap()
                                                  .get_mut(col_nr).unwrap();

                    enroute_entry_costs += at_north.entry_cost;

                    if step_count >= minsteps {
                        let candidate_cost = local_cost_ew + enroute_entry_costs;
                        
                        if at_north.lowest_known_from_ns > candidate_cost {
                            at_north.lowest_known_from_ns = candidate_cost;
                            new_worklist.push(Coord { line_nr: new_line_nr, col_nr: col_nr });
                        }
                    }
                }


            }

            worklist.clear();
            worklist.append(&mut new_worklist);
        }



        // self.propagate_costs_ns();
        // self.propagate_costs_we();

        println!("Propagated costs");
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




#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d17::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_render() {
        let pbuf = input::get_input("2023_d17_sample1.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let grid = Grid::from_content(&content);
        let printed_map = grid.print_glyphs();
        println!(" == Got map ==\n{}\n =========", printed_map);

        /*
        let printed_costs = grid.print_costs();
        println!(" == Got costs ==\n{}\n =========", printed_costs);

        grid.propagate_costs();

        let printed_costs = grid.print_costs();
        println!(" == Got costs ==\n{}\n =========", printed_costs);
        */
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d17::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d17_sample1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut grid = Grid::from_content(&content);           
            grid.propagate_costs(1, 3);
            let printed_costs = grid.print_costs();
            println!(" == Got costs ==\n{}\n =========", printed_costs);
            
            let loss = grid.heat_loss_bottom_right();
            assert_eq!(loss, 102); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d17.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut grid = Grid::from_content(&content);
            grid.propagate_costs(1, 3);
            let loss = grid.heat_loss_bottom_right();
            assert_eq!(loss, 970); // p1 skarp
        }

        // U-U-U-U-U-U-U-LTRAAAAAAAAAAAAA C-C-C-C-C-C-C-C-C-C-C-RUCIBLEEEEEEEEEEEEEEE
        {
            let pbuf = input::get_input("2023_d17_sample1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut grid = Grid::from_content(&content);
            grid.propagate_costs(4, 10);
            let loss = grid.heat_loss_bottom_right();
            assert_eq!(loss, 94); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d17_sample2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut grid = Grid::from_content(&content);
            grid.propagate_costs(4, 10);
            let loss = grid.heat_loss_bottom_right();
            assert_eq!(loss, 71); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d17.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut grid = Grid::from_content(&content);
            grid.propagate_costs(4, 10);
            let loss = grid.heat_loss_bottom_right();
            assert_eq!(loss, 1149); // p2 skarp
        }
    }
}

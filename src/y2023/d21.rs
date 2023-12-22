use core::num;
use std::collections::{VecDeque, HashSet};
use std::iter;
use regex::Regex;

use crate::input;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Plot {
    appearance: char,
}

#[allow(dead_code)]
impl Plot {
    pub fn from_ch(ch: &char) -> Plot {
        match ch {
            'S' => { Plot { appearance:*ch }},
            'O' => { Plot { appearance:*ch }},
            '.' => { Plot { appearance:*ch }},
            '#' => { Plot { appearance:*ch }},
            _ => {panic!("fd")},
        }
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HyperPlot {
    appearance: char,
    presence: Vec<bool>,
}

#[allow(dead_code)]
impl HyperPlot {
    pub fn from_ch(ch: &char, stepcount: usize) -> HyperPlot {
        let alloc: Vec<bool> = iter::repeat(false).take(stepcount).collect();
        match ch {
            'S' => { HyperPlot { appearance:*ch, presence:alloc }},
            'O' => { HyperPlot { appearance:*ch, presence:alloc }},
            '.' => { HyperPlot { appearance:*ch, presence:alloc }},
            '#' => { HyperPlot { appearance:*ch, presence:alloc }},
            _ => {panic!("fd")},
        }
    }
}



#[allow(dead_code)]
pub struct Garden {
    tiles: Vec<Vec<Plot>>,
}

#[allow(dead_code)]
impl Garden {
    pub fn from_content(content: &str) -> Garden {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut tiles:Vec<Vec<Plot>> = Vec::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let mut tile_row: Vec<Plot> = Vec::new();

            tile_row.push(Plot::from_ch(&'#'));
            tile_row.push(Plot::from_ch(&'#'));
            for ch in trimmed.chars() {
                tile_row.push(Plot::from_ch(&ch));
            }
            tile_row.push(Plot::from_ch(&'#'));
            tile_row.push(Plot::from_ch(&'#'));

            tiles.push(tile_row);
        }

        let mut pad_line: Vec<Plot> = Vec::new();
        for _ in 0..tiles.get(0).unwrap().len() {
            pad_line.push(Plot::from_ch(&'#'));
        }

        tiles.insert(0, pad_line.clone());
        tiles.insert(0, pad_line.clone());
        tiles.push(pad_line.clone());
        tiles.push(pad_line.clone());

        Garden { 
            tiles: tiles,
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
   
    pub fn step(&mut self) {
        let mut new_tiles = self.tiles.clone();

        for line_nr in 1..new_tiles.len()-1 {
            for col_nr in 1..new_tiles.get(0).unwrap().len()-1 {
                let new_tile =  new_tiles.get_mut(line_nr).unwrap().get_mut(col_nr).unwrap();
                if new_tile.appearance == '#' {
                    continue;
                }

                if self.tiles.get(line_nr-1).unwrap().get(col_nr+0).unwrap().appearance == 'O' || 
                   self.tiles.get(line_nr+1).unwrap().get(col_nr+0).unwrap().appearance == 'O' || 
                   self.tiles.get(line_nr+0).unwrap().get(col_nr-1).unwrap().appearance == 'O' || 
                   self.tiles.get(line_nr+0).unwrap().get(col_nr+1).unwrap().appearance == 'O' {
                    // Block
                    new_tile.appearance = 'O';
                } else {
                    new_tile.appearance = '.';
                }
            }
        }

        self.tiles.clear();
        self.tiles.extend(new_tiles);
    }
}

#[allow(dead_code)]
pub struct HyperGarden {
    hyper_plots: Vec<Vec<HyperPlot>>,
    worklist_locations: HashSet<(i64, i64)>,
    worklist_iters: usize,
    lazy_count: Vec<i64>,
    lazy_deltas: Vec<i64>,
    lazy_delta_deltas: Vec<i64>,
}

#[allow(dead_code)]
impl HyperGarden {
    pub fn from_content(content: &str, steps: usize) -> HyperGarden {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut plots:Vec<Vec<HyperPlot>> = Vec::new();
        let mut locations: HashSet<(i64, i64)> = HashSet::new();

        let mut line_nr:usize = 0;
        let mut col_nr:usize;
        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let mut tile_row: Vec<HyperPlot> = Vec::new();
            col_nr = 0;
            for ch in trimmed.chars() {
                tile_row.push(HyperPlot::from_ch(&ch, steps));
                if ch == 'S' {
                    locations.insert((line_nr.try_into().unwrap(), col_nr.try_into().unwrap()));
                }
                col_nr += 1;
            }
            plots.push(tile_row);
            line_nr +=1;
        }

        println!("Constructing counts");
        let lazy_counts: Vec<i64> = iter::repeat(-1).take(steps).collect();
        println!("Constructing deltas");
        let lazy_deltas: Vec<i64> = iter::repeat(-1).take(steps).collect();
        println!("Constructing delta deltas");
        let lazy_delta_deltas: Vec<i64> = iter::repeat(-1).take(steps).collect();

        HyperGarden { 
            hyper_plots: plots,
            worklist_iters: 0,
            worklist_locations: locations,
            lazy_deltas,
            lazy_count: lazy_counts,
            lazy_delta_deltas,
        }
    }

    pub fn precence_count_at_step(&self, stepnr: usize ) -> i64 {
        let counts = self.hyper_plots.iter().fold(0, |line_acc, line| {
            line_acc + line.iter().fold(0, |row_acc, cell| {
                row_acc + 0
            } )
        });
        counts
    }

    pub fn populate_lazy(&mut self, periodicity: usize) {
        while self.worklist_iters < self.hyper_plots.len() * 5 {
            // println!("Stepping worklist.");
            self.step_worklist()
        }

        while self.worklist_iters < periodicity * 5 {
            // println!("Stepping worklist.");
            self.step_worklist()
        }

        for n in periodicity..self.worklist_iters {
            self.lazy_delta_deltas[n] = self.lazy_deltas[n] - self.lazy_deltas[n-periodicity];
        }

        // println!("Populated head of lazy_delta_deltas: {:?}", self.lazy_delta_deltas);


        // Validate delta delta periodicity.
        let end = self.worklist_iters;
        for p in  2..periodicity+5 {
            if self.lazy_delta_deltas[end - p] != self.lazy_delta_deltas[end - p - periodicity] {
                panic!("fdkhfd")
            }
        }

        println!("peridicity {} verified", periodicity);

        for n in self.worklist_iters-3..self.lazy_delta_deltas.len() {
            self.lazy_delta_deltas[n] = self.lazy_delta_deltas[n - periodicity]; 
        }

        // println!("Populated rest of lazy_delta_deltas: {:?}", self.lazy_delta_deltas);

        // Populate deltas.
        for n in self.worklist_iters-3..self.lazy_delta_deltas.len() {
            self.lazy_deltas[n] = self.lazy_deltas[n-periodicity] + self.lazy_delta_deltas[n];
        }

        // println!("Populated lazy_deltas: {:?}", self.lazy_deltas);

        for n in self.worklist_iters-3..self.lazy_delta_deltas.len() {
            self.lazy_count[n] = self.lazy_count[n-1] + self.lazy_deltas[n];
        }


    }
    
    /*
    pub fn step_hyperplot(&mut self) {

        // IGNORE THIS APPROACH
        let mut new_counts = self.hyper_plots.clone();
        let height: i64 = self.hyper_plots.len().try_into().unwrap();
        let width: i64 = self.hyper_plots.get(0).unwrap().len().try_into().unwrap();

        for line_nr in 0..new_counts.len() {
            let line_nr_up: usize = usize::try_from({
                let as_int = i64::try_from(line_nr).unwrap();
                (as_int + 1) % height                
            }).unwrap();
            
            
            // ((i64::try_from(line_nr).unwrap() - 1) % height).try_into().unwrap();
            let line_nr_down: usize = usize::try_from({
                let as_int = i64::try_from(line_nr).unwrap();
                (height + as_int - 1) % height                
            }).unwrap();

            for col_nr in 0..new_counts.get(0).unwrap().len() {
                let tile =  self.hyper_plots.get_mut(line_nr).unwrap().get_mut(col_nr).unwrap();
                if tile.appearance == '#' {
                    // *new_counts.get_mut(line_nr).unwrap().get_mut(col_nr).unwrap() = 0;
                    continue;
                }

                let col_nr_left: usize = ((width + i64::try_from(col_nr).unwrap() - 1) % width).try_into().unwrap();
                let col_nr_right: usize = ((i64::try_from(col_nr).unwrap() + 1) % width).try_into().unwrap();

                /*
                let count = self.hyper_plots.get(line_nr_up).unwrap().get(col_nr).unwrap() +0;
                   self.hyper_plots.get(line_nr_down).unwrap().get(col_nr).unwrap() +
                   self.hyper_plots.get(line_nr).unwrap().get(col_nr_left).unwrap() + 
                   self.hyper_plots.get(line_nr).unwrap().get(col_nr_right).unwrap();

                   *new_counts.get_mut(line_nr).unwrap().get_mut(col_nr).unwrap() = count;
                    */
            }
        }

        self.hyper_plots.clear();
        self.hyper_plots.extend(new_counts);
    }
 */
    pub fn step_worklist(&mut self) {
        let mut new_worklist: HashSet<(i64, i64)> = HashSet::new();
        let len_before = self.worklist_locations.len();

        for (line_nr, col_nr) in self.worklist_locations.iter() {
            if !self.has_hyperstone_at(line_nr-1, *col_nr) {
                new_worklist.insert((line_nr-1, *col_nr));
            }
            if !self.has_hyperstone_at(line_nr+1, *col_nr) {
                new_worklist.insert((line_nr+1, *col_nr));
            }
            if !self.has_hyperstone_at(*line_nr, col_nr+1) {
                new_worklist.insert((*line_nr, col_nr+1));
            }
            if !self.has_hyperstone_at(*line_nr, col_nr-1) {
                new_worklist.insert((*line_nr, col_nr-1));
            }
        }

        self.worklist_locations.clear();
        self.worklist_locations.extend(new_worklist);
        self.worklist_iters += 1;

        let delta: i64 = i64::try_from(self.worklist_locations.len()).unwrap() - i64::try_from(len_before).unwrap();
        println!("After {} steps, step worklist is {}. Delta is {}", self.worklist_iters, self.worklist_locations.len(), delta);
        self.lazy_count[self.worklist_iters] = self.worklist_locations.len().try_into().unwrap();
        self.lazy_deltas[self.worklist_iters] = delta;
    }

    pub fn has_hyperstone_at(&self, line_nr: i64, col_nr: i64) -> bool {
        let line_nr_usize: usize = line_nr.rem_euclid(self.hyper_plots.len().try_into().unwrap()).try_into().unwrap();
        let col_nr_usize: usize = col_nr.rem_euclid(self.hyper_plots.get(0).unwrap().len().try_into().unwrap()).try_into().unwrap();

        return self.hyper_plots.get(line_nr_usize).unwrap().get(col_nr_usize).unwrap().appearance == '#'
    }

    pub fn lazy_delta(&self, step_nr: usize) -> i64 {
        *self.lazy_deltas.get(step_nr).unwrap()
    }

    pub fn lazy_counts(&self, step_nr: usize) -> i64 {
        *self.lazy_count.get(step_nr).unwrap()
    }
}


#[allow(dead_code)]
pub struct Part2Solver {
    loc_count: Vec<i64>,  // As read from file
    val_deltas: Vec<i64>, // As read from file. These are deltas between loc with mis-indexing 1.
    oct_deltas: Vec<i64>, // As inferred from file. These are deltas between val deltas with mis-indexing matching periodicity.

    // val_delta_origin: usize,
    // val_delta_offsets: Vec<i64>,

    periodicity: usize,

    octavian_delta_origin: usize,
    octavian_delta_at_origin: i64,
    octavian_delta_offsets: Vec<i64>,
}


impl Part2Solver {
    #[allow(dead_code)]
    pub fn from_filename(filename: &str) -> Part2Solver {
        let pbuf = input::get_input(filename).unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let lines: Vec<&str> = content.split("\n").collect();

        let mut loc_count: Vec<i64> = Vec::new();
        let mut val_deltas: Vec<i64> = Vec::new();
        let mut oct_deltas: Vec<i64> = Vec::new();
    

        let re = Regex::new(r"^After (?P<n>[0-9]+) steps, step worklist is (?P<loc_count>[0-9]+). Delta is (?P<delta>[0-9]+)$").unwrap();       

        for line_nr in 1..lines.len() {
        // for line_nr in 1..100 {            
            let trimmed = lines[line_nr].trim();
            if trimmed.len() == 0 {continue;};

            let groups = match re.captures(trimmed) {
                Some(stuff) => stuff,
                None => { 
                    println!("Could not regexp rule from content : '{}'", trimmed);
                    panic!("Bad regexp for part line")
                },
            };
    
            let n = groups["n"].to_string().parse::<usize>().unwrap();
            let count = groups["loc_count"].to_string().parse::<i64>().unwrap();
            let delta = groups["delta"].to_string().parse::<i64>().unwrap();

            while loc_count.len() < n+1 {
                loc_count.push(-1);
            };
            loc_count[n] = count;

            while val_deltas.len() < n+1 {
                val_deltas.push(-1);
            };
            val_deltas[n] = delta;

            while oct_deltas.len() < n+1 {
                oct_deltas.push(-1);
            };
        }

        let mut p2solver = Part2Solver {
            loc_count,
            val_deltas,
            oct_deltas,

            // val_delta_origin: 0,
            // val_delta_offsets: Vec::new(),
            periodicity: usize::MAX,

            octavian_delta_origin: 0,
            octavian_delta_at_origin: -1,
            octavian_delta_offsets: Vec::new(),
        };

        /*
        let is_130 = p2solver.check_periodicity(130);
        let is_131 = p2solver.check_periodicity(131);
        let is_132 = p2solver.check_periodicity(132);
        let is_10 = p2solver.check_periodicity(10);
        let is_11 = p2solver.check_periodicity(11);
        let is_12 = p2solver.check_periodicity(12);
        println!("Peridocity checks: {}, {}, {}, {}, {}, {}", is_10, is_11, is_12, is_130, is_131, is_132);
        */

        if p2solver.check_periodicity(11) {
            // Nothing
        } else if p2solver.check_periodicity(131) {
            // Nothing
        } else {
            panic!("Bad peridicity")
        }


        p2solver
    }

    #[allow(dead_code)]
    pub fn check_periodicity(&mut self, periodicity: usize) -> bool {

        for n in periodicity..self.val_deltas.len() {
            self.oct_deltas[n] = self.val_deltas[n] - self.val_deltas[n-periodicity];
        }

        if 2 * periodicity > self.oct_deltas.len() {
            println!("Can not check periodicity {} for oct_deltas with len {}", periodicity, self.oct_deltas.len());
            return false;
        }

        for n in self.oct_deltas.len() - 5 - periodicity..self.oct_deltas.len() - 3 {
            if self.oct_deltas[n-periodicity] != self.oct_deltas[n] {
                println!("Rejecting periodicity {}", periodicity);
                println!("for delta_deltas {:?}", self.oct_deltas);
                return false
            }
        }

        self.periodicity = periodicity;

        let octavian_delta_origin: usize = self.oct_deltas.len() - 5 - periodicity;
        let octavian_delta_at_origin: i64 = self.oct_deltas[octavian_delta_origin];
        let octavian_delta_offsets: Vec<i64> = self.oct_deltas[octavian_delta_origin..octavian_delta_origin+periodicity].iter().map( |val| {
            *val - octavian_delta_at_origin
        }).collect();

        println!("oct_deltas octave at {} is {} with offsets {:?} of periodicity {}", 
            octavian_delta_origin, 
            octavian_delta_at_origin, 
            octavian_delta_offsets,
            periodicity);

        self.octavian_delta_origin = octavian_delta_origin;
        self.octavian_delta_at_origin = octavian_delta_at_origin;
        self.octavian_delta_offsets = octavian_delta_offsets;

        return true
    }


    pub fn deXXlta_delta(&self, n: usize) -> i64 {
        let mut offset: usize = n - self.octavian_delta_origin;
        let periodicity: usize = self.octavian_delta_offsets.len();

        offset = offset.div_euclid(periodicity);
        let toreturn: i64 = self.octavian_delta_offsets[offset];
        toreturn
    }

    pub fn delta(&mut self, n: usize) -> i64 {
        /*
        let periodicity = self.delta_offsets.len();

        let mut steps_to_go: usize = n-self.delta_origin;
        let mut toreturn = self.delta_at_origin;
        while steps_to_go >= periodicity {
            steps_to_go -= periodicity;
            toreturn += self.delta_octave_step;
        }

        toreturn += self.delta_offsets[steps_to_go];

        // println!("Delta built for n={}. {} + {}*{} + {}", n, self.delta_at_origin, num_octaves_to_add, self.delta_octave_step, self.delta_offsets[offset_in_octave]);
        toreturn        
         */

        
        if self.val_deltas.len() < self.periodicity {
            println!("Val deltas is {:?}, need to be at least length of periodicity {}", self.val_deltas, self.periodicity);
            panic!("fjldgh val deltas bad")
        }
        
        /*
        if n >= self.val_deltas.len() {
            let new_n = n-self.periodicity;
            // println!("recur delta({}) to delta {}", n, new_n );


            let delta_new_n = self.delta(new_n);
            let oct_delta = self.octavian_delta(n);
            let toreturn = delta_new_n + oct_delta;
            println!("delta({}) ---recur--> delta({}) + octavian_delta({}) ==> {} = {} + {}", n, new_n, n, toreturn, delta_new_n, oct_delta);
            return toreturn
        }
         */

        // Rewrite without recur:
        if n >= self.val_deltas.len() || self.val_deltas[n] == -1 {
            let mut new_n = n-self.periodicity;
            let mut periods_to_skip = 1;

            while new_n >= self.val_deltas.len() {
                new_n = new_n-self.periodicity;
                periods_to_skip += 1;
            };



            // println!("recur delta({}) to delta {}", n, new_n );


            let delta_new_n = self.delta(new_n);
            let oct_delta = self.octavian_delta(n);
            let toreturn = delta_new_n + periods_to_skip * oct_delta;
            // println!("delta({}) ---recur--> delta({}) + octavian_delta({}) ==> {} = {} + {}", n, new_n, n, toreturn, delta_new_n, oct_delta);

            while self.val_deltas.len() <= n {
                self.val_deltas.push(-1)
            }

            self.val_deltas[n] = toreturn;
            
            return toreturn
        }


        self.val_deltas[n]
    }

    pub fn octavian_delta(&self, n: usize) -> i64 {
        
        if self.periodicity < 3 { panic!("fldh")}
        let mut idx = n - self.octavian_delta_origin;
        
        idx = idx.rem_euclid(self.periodicity);
        /*
        while idx >= self.periodicity {
            idx -= self.periodicity;
        } 
        */

        // self.oct_deltas[idx]
        self.octavian_delta_at_origin + self.octavian_delta_offsets[idx]
        
    }
    

    #[allow(dead_code)]
    pub fn reach_after(&mut self, steps: usize) -> i64 {
        if steps >= self.loc_count.len() {
            // println!("Need to recur, want reach@{}, but loc_count.len() is only {}", steps, self.loc_count.len());
            let toreturn = self.reach_after(steps-1) + self.delta(steps);
            let number_to_add = steps + 4 - self.loc_count.len();
            for _ in 0..number_to_add {
                self.loc_count.push(-1);
            }

            self.loc_count[steps] = toreturn;
            return toreturn
        }

        if self.loc_count[steps] == -1 {
            let toreturn = self.reach_after(steps-1) + self.delta(steps);
            self.loc_count[steps] = toreturn;
            return toreturn
        }

        self.loc_count[steps]
    }
}


#[allow(dead_code)]
pub fn part1(content: &str, stepcount: i64) -> i64 {
    let mut garden = Garden::from_content(&content.replace("S", "O"));
    for _ in 0..stepcount {
        garden.step();
    } 

    let mut toreturn = 0;

    for row in garden.tiles.iter()  {
        for tile in row.iter() {
            if tile.appearance == 'O' {
                toreturn += 1
            }
        }
    }
    toreturn
}

#[allow(dead_code)]
fn part2(content: &str, steps: usize) -> i64 {
    let mut garden = HyperGarden::from_content(&content, steps+5);
    println!("Hypergarden built");
    garden.populate_lazy(garden.hyper_plots.len());
    garden.lazy_counts(steps)
}



#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d21::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_render() {
        let pbuf = input::get_input("2023_d21_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap().replace("S", "O");
        let mut garden = Garden::from_content(&content);
        let printed_map = garden.print_glyphs();
        println!(" == Got map ==\n{}\n =========", printed_map);
        for _ in 0..3 {
            garden.step();
            let printed_map = garden.print_glyphs();
            println!(" == Got map ==\n{}\n =========", printed_map);
        }       
    }

    #[test]
    fn test_render2() {
        let pbuf = input::get_input("2023_d21_sample2.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap().replace("S", "O");
        let mut garden = Garden::from_content(&content);
        let printed_map = garden.print_glyphs();
        println!(" == Got map ==\n{}\n =========", printed_map);
        for _ in 0..19 {
            garden.step();
            let printed_map = garden.print_glyphs();
            println!(" == Got map ==\n{}\n =========", printed_map);
        }       
    }

    #[test]
    fn test_worklist_approach() {
        let pbuf = input::get_input("2023_d21_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut garden = HyperGarden::from_content(&content, 1);

        for _ in 0..505 {
            garden.step_worklist();
        }

        assert_eq!(2,5);
        
    }

    #[test]
    fn test_lazy_approach() {
        let pbuf = input::get_input("2023_d21_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut garden = HyperGarden::from_content(&content, 600);

        garden.populate_lazy(11);

        assert_eq!(605, garden.lazy_delta(480));
        assert_eq!(784, garden.lazy_delta(481));
        assert_eq!(694, garden.lazy_delta(482));
        assert_eq!(399, garden.lazy_delta(483));
        assert_eq!(490, garden.lazy_delta(484));
        assert_eq!(620, garden.lazy_delta(485));
        assert_eq!(708, garden.lazy_delta(486));
        assert_eq!(619, garden.lazy_delta(487));
        assert_eq!(801, garden.lazy_delta(488));
        assert_eq!(704, garden.lazy_delta(489));
        assert_eq!(706, garden.lazy_delta(490));
        assert_eq!(619, garden.lazy_delta(491));
        assert_eq!(802, garden.lazy_delta(492));
        assert_eq!(710, garden.lazy_delta(493));
        assert_eq!(408, garden.lazy_delta(494));
        assert_eq!(501, garden.lazy_delta(495));

        assert_eq!(153909, garden.lazy_counts(480));
        assert_eq!(154693, garden.lazy_counts(481));
        assert_eq!(155387, garden.lazy_counts(482));
        assert_eq!(155786, garden.lazy_counts(483));
        assert_eq!(156276, garden.lazy_counts(484));
        assert_eq!(156896, garden.lazy_counts(485));
        assert_eq!(157604, garden.lazy_counts(486));
        assert_eq!(158223, garden.lazy_counts(487));
        assert_eq!(159024, garden.lazy_counts(488));
        assert_eq!(159728, garden.lazy_counts(489));
        assert_eq!(160434, garden.lazy_counts(490));
        assert_eq!(161053, garden.lazy_counts(491));
        assert_eq!(161855, garden.lazy_counts(492));
        assert_eq!(162565, garden.lazy_counts(493));
        assert_eq!(162973, garden.lazy_counts(494));
        assert_eq!(163474, garden.lazy_counts(495));
        assert_eq!(164108, garden.lazy_counts(496));
        assert_eq!(164832, garden.lazy_counts(497));
        assert_eq!(165465, garden.lazy_counts(498));
        assert_eq!(166284, garden.lazy_counts(499));
        assert_eq!(167004, garden.lazy_counts(500));
        assert_eq!(167726, garden.lazy_counts(501));
        assert_eq!(168359, garden.lazy_counts(502));
        assert_eq!(169179, garden.lazy_counts(503));
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d21::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d21_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content, 6);
            assert_eq!(steps, 16); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d21.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content, 64);
            assert_eq!(steps, 3782); // p1 skarp
        }


        {
            let table: Vec<(usize, i64)> = Vec::from_iter([
                // (6, 16),
                // (10, 50),
                // (50, 1594), // Fails here.
                // (100, 6536),
                (500, 167004),
                (1000, 668697),
                (5000, 16733044),
            ]);

            let mut p2solver = Part2Solver::from_filename("2023_d21_sample_log.txt");
            for n in 180..220 {
                // println!("p2solver delta @{} for sample is {}", n, p2solver.delta(n));
            }

            assert_eq!(605, p2solver.delta(480));
            assert_eq!(784, p2solver.delta(481));
            assert_eq!(694, p2solver.delta(482));
            assert_eq!(399, p2solver.delta(483));
            assert_eq!(490, p2solver.delta(484));
            assert_eq!(620, p2solver.delta(485));
            assert_eq!(708, p2solver.delta(486));
            assert_eq!(619, p2solver.delta(487));
            assert_eq!(801, p2solver.delta(488));
            assert_eq!(704, p2solver.delta(489));
            assert_eq!(706, p2solver.delta(490));
            assert_eq!(619, p2solver.delta(491));
            assert_eq!(802, p2solver.delta(492));
            assert_eq!(710, p2solver.delta(493));
            assert_eq!(408, p2solver.delta(494));
            assert_eq!(501, p2solver.delta(495));
    
            for (steps, plots) in table {
                let actual = p2solver.reach_after(steps);
                println!("Verifying {} == {}", actual, plots);
                assert_eq!(actual, plots); // p2 sample
            }
            
                /*
            let pbuf = input::get_input("2023_d21_sample2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            for (steps, plots) in table {
                let actual = part2(&content, steps.try_into().unwrap());
                assert_eq!(actual, plots); // p2 sample
            }
             */

        }
        {
            /*
            let pbuf = input::get_input("2023_d21.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            // let actual = part2(&content, 26501365);
            let actual = part2(&content, 1000);
             */

            let mut p2solver = Part2Solver::from_filename("2023_d21_log.txt");
            
            
            let mut n = 0;
            while n < 26501365 {
                n += 1000;
                println!("Trying reach {}", n);
                for _ in 0..1000 {
                    n += 1000;
                    p2solver.reach_after(n);
                }
            }
            
            let foo = p2solver.reach_after(110000);
            println!("foo is {}", foo);

            let actual = p2solver.reach_after(26501365);
            assert_eq!(actual, 630661863455116); // p2 skarp
        }
    }
}

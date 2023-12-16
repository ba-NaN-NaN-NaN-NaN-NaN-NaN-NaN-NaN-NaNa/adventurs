use std::time::Duration;
use std::thread;
use std::{collections::{HashSet, HashMap}, fmt::format};
use std::rc::Rc;
use std::time::Instant;
use std::sync::mpsc;
use std::sync::Arc;


#[allow(dead_code)]
#[derive(Debug)]
pub struct SpringSolver {
    solve_options: HashMap<String, Rc<HashSet<String>>>,
    memoized_calls: i64,
    non_memoized_calls: i64,
    empty_solutions: Rc<HashSet<String>>,
    ways: HashMap<String, HashMap<String,i64>>,
}

#[allow(dead_code)]
impl SpringSolver {
    pub fn new() -> SpringSolver {
        SpringSolver {
            solve_options: HashMap::new(),
            memoized_calls: 0,
            non_memoized_calls: 0,
            empty_solutions: Rc::new(HashSet::new()),
            ways: HashMap::new(),
        }
    }

    pub fn solutions_for(&self, line: &String) -> Rc<HashSet<String>> {
        return self.solve_options.get(line).unwrap().clone()
    }

    pub fn solcount_for(&self, line: &String) -> usize {
        return self.solve_options.get(line).unwrap().len()
    }

    pub fn populate_solutions(&mut self, line: &String) {

        return self.populate_solutions2(&ProgressMarker::from_content(line))

        /*
        // 
        // Memoized calc of solutions for a given input line.
        // Possible content values in the hashset:
        //   None <- Not calculated yet.
        //    [] <- Calculated, no solutions.
        //    [a,b,c..] <- Calculated, contains solutions.  Some of those solutions may be empty strings.
        // None of the solutions contain '?'. Only the keys do.
        //
        if self.solve_options.contains_key(line) {
            self.memoized_calls += 1;
            // return self.solve_options.get(&line).unwrap().clone()
            return 
        }

        let mark = Instant::now();
        let mut sol_inserts = 0;
        self.non_memoized_calls += 1;

        // println!("Will populate solutions for line '{}'", line);
        if line.trim().len() == 0 {
            panic!("Malformed input")
        }

        let pm = ProgressMarker::from_content(&line);

        // Some special cases.
        if pm.formatted == ". 1" {
            // let solutions: HashSet<String> = HashSet::from_iter([]);
            self.solve_options.insert(line.to_string(), self.empty_solutions.clone());

            // self.solve_options.insert(line.to_string(), Rc::new(solutions.clone()));
            // println!("Not solveable based on hard-coded pm.formatted='{}' filter.", pm.formatted);
            // return solutions;
            return
        }


        // Quick exit
        if pm.constraints.len() == 0 {
            // Case: Only working gears.
            let replaced = pm.pattern.replace("?", ".");
            if replaced.contains("#") {
                // Can not solve pattern with #, but no allocateable gears.
                // let solutions: HashSet<String> = HashSet::from_iter([]);
                // self.solve_options.insert(line.clone(), Rc::new(solutions));
                self.solve_options.insert(line.to_string(), self.empty_solutions.clone());

                // println!("Can not solve, want empty space, but still gear chars mandated, got '{}' -> {:?}", line, solutions.clone());
                // return solutions;
                return
            } else {
                // Solution is empty space.
                let solutions: HashSet<String> = HashSet::from_iter([replaced]);
                self.solve_options.insert(line.clone(), Rc::new(solutions));
                // println!("Solveable via empty space, got '{}' -> {:?}", line, solutions.clone());
                // return solutions;
                return
            }
        }


        if pm.constraints.len() == 1 {
            if pm.pattern.len() == *pm.constraints.get(0).unwrap() {
                let replaced = pm.pattern.replace("?", "#");
                if replaced == "#".repeat(pm.pattern.len()) {
                    // Solution is a single gear of same length as pattern.
                    let solutions: HashSet<String> = HashSet::from_iter([replaced]);
                    self.solve_options.insert(line.clone(), Rc::new(solutions));

                    // println!("Solveable by single gear approach, got '{}' -> {:?}", line, solutions.clone());
                    // return solutions;
                    return
                }
            }
        }


        // Solve check for constraints are larger than size of pattern.
        let mut pattern_len_needed = pm.constraints.get(0).unwrap().to_owned();
        for cons_nr in 1..pm.constraints.len() {
            pattern_len_needed = pattern_len_needed + 1 + pm.constraints.get(cons_nr).unwrap().to_owned()
        }

        if pattern_len_needed > pm.pattern.len() {
            // Not solveable.
            // let solutions: HashSet<String> = HashSet::from_iter([]);
            // self.solve_options.insert(line.to_string(), Rc::new(solutions));
            self.solve_options.insert(line.to_string(), self.empty_solutions.clone());

            // println!("Not solveable, got {} -> {:?}", line, solutions.clone());
            // return solutions;
            return
        }


        let mut solutions: HashSet<String> = HashSet::new();

        for spacer_pos in 0..pm.pattern.len() {
            let patt_at_spacer = pm.pattern[spacer_pos..spacer_pos+1].to_string();
            // println!("Considering solutions for '{}-{}-{}' with constraint {:?}", patt_to_left, patt_at_spacer, patt_to_right, pm.constraints);
            if patt_at_spacer == "#" {
                continue;
            }

            let patt_to_left = pm.pattern[0..spacer_pos].to_string();
            let patt_to_right = pm.pattern[spacer_pos+1..].to_string();

            // Four cases here:
            // spacer_pos is the whole string.
            // spacer_pos is at beginning. Left part is empty string, and right part is original line but trimmed by 1 char on left.
            // spacer_pos is at end. Right part is empty string, and left part is original line but trimmed by 1 char on right.
            // spacer_pos is somewhere in the middle. Solutions are (as below)

            // CASE: spacer_pos is the whole string.
            
            // Ignore, should be handled above.

            // CASE: spacer_pos is at beginning.
            if spacer_pos == 0 {
                // let pm_right = ProgressMarker::from_parsed(&patt_to_right, pm.constraints.clone());
                let pm_right_formatted = unparse(&patt_to_right, &pm.constraints);
                
                // let sols_only_right = self.populate_solutions(pm_right.formatted);
                self.populate_solutions(&pm_right_formatted);
                //let sols_only_right = self.solutions_for(&pm_right.formatted);
                // let sols_only_right = self.solutions_for(&pm_right.formatted);
                let sols_only_right = self.solve_options.get(&pm_right_formatted).unwrap().iter();

                for right in sols_only_right {
                    let sol = format!(".{}", right);
                    // println!("A non-gear start solution to '{}' is \"_{}\" -> '{}'", line, right, sol);
                    solutions.insert(sol);
                    sol_inserts+=1;
                }
                continue;
            }


            // CASE: spacer_pos is at end.
            if spacer_pos == pm.pattern.len()-1 {
                let pm_left = ProgressMarker::from_parsed(&patt_to_left, pm.constraints.clone());
                self.populate_solutions(&pm_left.formatted);
                // let sols_only_left = self.solutions_for(&pm_left.formatted);
                let sols_only_left = self.solve_options.get(&pm_left.formatted).unwrap().iter();

                for left in sols_only_left {
                    let sol = format!("{}.", left);
                    // println!("A non-gear end solution to '{}' is \"{}_\" -> '{}'", line, left, sol);
                    solutions.insert(sol);
                    sol_inserts+=1;
                }
                continue;
            }


            // CASE: spacer_pos is somewhere in the middle. Try all combos of constraints to pattern sides.
            for cons_split_pos in 0..pm.constraints.len() {                
                //
                // if constraints on left have a solution for left part of pattern
                //     AND
                // if constraints at right have a solution for right part of pattern
                // then for each combo of left/right solutions
                // a solution exists that is 
                // 'left'+'.'+'right'
                let (cons_to_left, cons_to_right) = pm.constraints.split_at(cons_split_pos);

                let pm_left = ProgressMarker::from_parsed(&patt_to_left, Vec::from(cons_to_left));
                self.populate_solutions(&pm_left.formatted);

                let pm_right = ProgressMarker::from_parsed(&patt_to_right, Vec::from(cons_to_right));
                self.populate_solutions(&pm_right.formatted);

                
                let sols_left = self.solve_options.get(&pm_left.formatted).unwrap().iter();
                // let sols_right = self.solutions_for(&pm_right.formatted);
                let sols_right = self.solve_options.get(&pm_right.formatted).unwrap();

                for left in sols_left {
                    for right in sols_right.iter() {
                        let sol = format!("{}.{}", left, right);
                        // println!("A split solution to '{}' is \"{}_{}\" -> '{}'", line, left, right, sol);
                        solutions.insert(sol);
                        sol_inserts+=1;
                    }
                }
            }
        }

        // println!("Done with solution creation, got {} -> {:?}", line, solutions.clone());
        // println!("finding solution, did {} solve inserts in {}ms, for {} resulting solution.", sol_inserts, mark.elapsed().as_millis(), solutions.len());
        self.solve_options.insert(line.to_string(), Rc::new(solutions));
        // solutions

        */
    }

    pub fn populate_solutions2(&mut self, pm: &ProgressMarker) {
        // 
        // Memoized calc of solutions for a given input line.
        // Possible content values in the hashset:
        //   None <- Not calculated yet.
        //    [] <- Calculated, no solutions.
        //    [a,b,c..] <- Calculated, contains solutions.  Some of those solutions may be empty strings.
        // None of the solutions contain '?'. Only the keys do.
        //
        if self.solve_options.contains_key(&pm.formatted) {
            self.memoized_calls += 1;
            // return self.solve_options.get(&line).unwrap().clone()
            return 
        }

        let mut solutions: HashSet<String> = HashSet::new();

        // Eat options are:
        // n>1 whitespaces, then fill that + all solutions to trailing part.
        // a gear, followed by a whitespace or end of input.
        
        let max_whitespaces_to_eat = pm.max_whitespaces_to_eat();
        // println!("Will try to eat up to {} whitespaces from {:?}", max_whitespaces_to_eat, pm);
        for n in 1..max_whitespaces_to_eat+1 {
            let (head, tail) = pm.pattern.split_at(n);

            // Case: Whole pattern can be zeroed. But we need to check constraints.
            if tail.len() == 0 {
                if pm.constraints.len() == 0 {
                    solutions.insert(".".repeat(head.len()));
                }
            } else {
                if tail.starts_with(".") {
                    // Don't consider this option, try to eat more instead.
                    continue;
                }
                
                let pm_tail = ProgressMarker::from_parsed(&tail.to_string(), pm.constraints.clone());
                // println!("Will permute eat {} whitespaces from pattern '{}'. pm_tail ={:?}", n, pm.pattern, pm_tail);
                self.populate_solutions2(&pm_tail);
                let subsols = self.solve_options.get(&pm_tail.formatted.clone()).unwrap();

                let prefix = ".".repeat(head.len());
                for subsol in subsols.iter() {
                    solutions.insert(format!("{}{}", prefix, subsol));
                }
            }

            /*
            let prefix = ".".repeat(n);
            let leftovers = pm.pattern[n..].to_string();
            let pm = ProgressMarker::from_parsed(&leftovers, pm.constraints.clone());
            self.populate_solutions2(&pm);
            let subsols = self.solve_options.get(&pm.formatted.clone()).unwrap();
            for subsol in subsols.iter() {
                solutions.insert(format!("{}{}", prefix, subsol));
            }
             */
        }

        // println!("Will check constraints.len()==1 for pm {:?}", pm);
        if pm.constraints.len() == 1 {
            let n = pm.constraints.get(0).unwrap();

            if pm.pattern.len() < *n {
                // Impossible. We do not have enough pattern to populate our desired gear.
                self.solve_options.insert(pm.formatted.clone(), self.empty_solutions.clone());
                return
            }

            // Try eat gear + end of pattern.
            if pm.pattern.len() == *n {
                let mut space_found = false;
                for ch in pm.pattern.chars() {
                    if ch == '.' {
                        space_found = true;
                        break
                    }
                }

                if !space_found {
                    solutions.insert("#".repeat(*n));
                }
            }

            // Try eat gear + whitespace rest of pattern.
            if pm.pattern.len() > *n {
                /*
                if '.' in first part of pattern || '#' in second part of patten {
                    not possible 
                }
                */
                let (head, tail) = pm.pattern.split_at(*n);
                let blocked = head.contains(".") || tail.contains("#");
                let possible = !blocked;
                if possible {
                    // We can return here since we eat the whole pattern unambiguously.
                    solutions.insert(format!("{}{}", "#".repeat(head.len()), ".".repeat(tail.len())));
                    self.solve_options.insert(pm.formatted.clone(), Rc::new(solutions));
                    return
                }
            }
        }

        // Try to eat gear + one whitespace + permute with any following possible solutions.
        // println!("Will check constraints.len()>1 for pm {:?}", pm);
        if pm.constraints.len() > 1 {
            let n = pm.constraints.get(0).unwrap();
            
            if pm.pattern.len() < *n+1 {
                // Impossible. We do not have enough pattern length left to populate our desired gear .
                self.solve_options.insert(pm.formatted.clone(), self.empty_solutions.clone());
                return
            }

            let (head, tail) = pm.pattern.split_at(*n);
            let mut space_found = false;
            for ch in head.chars() {
                if ch == '.' {
                    space_found = true;
                    break
                }
            }

            if !space_found && !tail.starts_with("#") && tail.len() > 1 {
                // if tail starts with non-gear, then permute with rest of solutions.
                let tail_pattern = tail.get(1..).unwrap().to_string();
                   
                let tail_constraints = pm.constraints.get(1..).unwrap();

                let tail_pm = ProgressMarker::from_parsed(&tail_pattern, Vec::from(tail_constraints));
                self.populate_solutions2(&tail_pm);
                let tail_solves = self.solutions_for(&tail_pm.formatted);

                let head_gear = "#".repeat(*n);
                for tail_solve in tail_solves.iter() {
                    solutions.insert(format!("{}.{}", head_gear, tail_solve));
                }
            
            }


        }

        self.solve_options.insert(pm.formatted.clone(), Rc::new(solutions));


    }


    pub fn memoized_ways_to_solve(&self, pattern: &String, constraints: &String) -> Option<i64> {
        match self.ways.get(pattern) {
            Some(row) => {
                match row.get(constraints) {
                    Some(n) => Some(*n),
                    None => None,
                }
            },
            None => { None }
        }
    }

    pub fn update_ways_to_solve(&mut self, pattern: &String, constraints: &String, count: i64) {
        match self.ways.get_mut(pattern) {
            Some(row) => {
                match row.get_mut(constraints) {
                    Some(n) => {
                        if *n != count {
                            println!("Trying to update ways to solve for {}  {} -> {}, but already have {}", pattern, constraints, count, *n);
                            panic!("Sanity check failed?? ")
                        }
                    },
                    None => {
                        row.insert(constraints.clone(), count);
                    },
                }
            },
            None => { 
                let mut new_row: HashMap<String, i64> = HashMap::new();
                new_row.insert(constraints.clone(), count);
                self.ways.insert(pattern.clone(), new_row);
            }
        };
    }


    pub fn ways_to_solve(&mut self, pattern: &String, constraints: &String) -> i64 {
        if let Some(n) = self.memoized_ways_to_solve(pattern, constraints) {
            return n
        }
        

        -1
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ProgressMarker {
    pattern: String, // Only the pattern part
    constraints: Vec<usize>,
    formatted: String, // Formatted like the original line.
}

#[allow(dead_code)]
impl ProgressMarker {
    pub fn from_content(content: &String) -> ProgressMarker {
        let parts: Vec<&str> = content.trim().split(" ").collect();

        let constraints_unparsed: Vec<&str> = match parts.get(1) {
            Some(frag) => { frag.split(",").collect() },
            None => { Vec::new() },
        };
            
        let constraint_ints: Vec<usize> = constraints_unparsed.iter().map(|l| { 
            l.trim().parse::<usize>().unwrap() 
        }).collect();

        ProgressMarker::from_parsed(&parts.get(0).unwrap().trim().to_string(), constraint_ints)
        /*
        let constraint_strs: Vec<String> = constraint_ints.iter().map( |i| i.to_string() ).collect();
        let left_to_eat: String = parts.get(0).unwrap().to_string();
        let constraint_string: String = constraint_strs.join(",");

        ProgressMarker {
            pattern: parts.get(0).unwrap().trim().to_string(),
            constraints: Vec::from(constraint_ints),
            formatted: format!("{} {}", left_to_eat, constraint_string),
        }
         */
    }

    pub fn format(&self) -> String {
        self.formatted.clone()
    }

    pub fn from_parsed(pattern: &String, constraints: Vec<usize>) -> ProgressMarker {
        if pattern.trim().len() == 0 {
            panic!("Can not build from parsed pattern '{}'", pattern);
        }

        let formatted = unparse(pattern, &constraints);
        ProgressMarker {
            pattern: pattern.to_string(),
            constraints: constraints,
            formatted: formatted,
        }
    }

    pub fn unroll(&self, count: usize) -> ProgressMarker {
        
        let mut new_constraints: Vec<usize> = Vec::new();
        let mut patt_frags: Vec<String> = Vec::new();
        for _ in 0..count {
            let mut clone = self.constraints.clone();
            new_constraints.append(&mut clone);
            patt_frags.push(self.pattern.clone());
        }

        ProgressMarker::from_parsed(&patt_frags.join("?"), new_constraints)
    }

    pub fn max_whitespaces_to_eat(&self) -> usize {
        match self.pattern.find("#") {
            Some(n) => n,
            None => self.pattern.len(),
        }
    }




}


#[allow(dead_code)]
pub fn unparse(pattern: &String, constraints: &Vec<usize>) -> String {
    let trimmed = pattern.trim();
    if trimmed.len() == 0 {
        panic!("Can not build from parsed pattern '{}'", pattern);
    }

    let constraint_strs: Vec<String> = constraints.iter().map( |i| i.to_string() ).collect();
    let constraint_string: String = constraint_strs.join(",");
    format!("{} {}", trimmed, constraint_string)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut ss = SpringSolver::new();
    let lines: Vec<&str> = input.split("\n").collect();
    let mut toreturn = 0;

    for line in lines {
        let mark = Instant::now();
        let trimmed = line.trim();
        if trimmed.len() == 0 { continue; }
        ss.populate_solutions(&trimmed.to_string());
        // ss.solcount_for(trimmed.to_string());
        let solcount = ss.solcount_for(&trimmed.to_string());
        let duration = mark.elapsed().as_millis();
        println!("part1: Line '{}' got {} solutions in {}ms", trimmed, solcount, duration);
        toreturn += solcount
    }
    toreturn.try_into().unwrap()
}


#[allow(dead_code)]
pub fn part2_5(input: &str) -> i64 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut toreturn = 0;

    for line_nr in 0..lines.len() {
        let line = lines.get(line_nr).unwrap();
        let mut ss = SpringSolver::new();
        let mark = Instant::now();
        let trimmed = line.trim();
        if trimmed.len() == 0 { continue; }
        let pm = ProgressMarker::from_content(&trimmed.to_string());
        
        ss.populate_solutions2(&ProgressMarker::from_content(&pm.unroll(5).formatted));
        // ss.solcount_for(trimmed.to_string());
        let solcount = ss.solcount_for(&pm.unroll(5).formatted);
        let duration = mark.elapsed().as_millis();
        println!("part2: Line nr {} = '{}' got {} solutions in {}ms", line_nr, trimmed, solcount, duration);
        toreturn += solcount
    }
    toreturn.try_into().unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut toreturn = 0;

    for line_nr in 0..lines.len() {
        let line = lines.get(line_nr).unwrap();
        let mut ss = SpringSolver::new();
        let mark = Instant::now();
        let trimmed = line.trim();
        if trimmed.len() == 0 { continue; }
        // let pm = ProgressMarker::from_content(&trimmed.to_string());
        
        // ss.populate_solutions2(&ProgressMarker::from_content(&pm.unroll(5).formatted));
        // ss.solcount_for(trimmed.to_string());
        // let solcount = ss.solcount_for(&pm.unroll(5).formatted);

        let frags: Vec<&str> = trimmed.split(" ").collect();
        let pattern = frags.get(0).unwrap().to_string();
        let constraints =frags.get(1).unwrap().to_string();
        let ways_to_solve = ss.ways_to_solve(&pattern, &constraints);

        let duration = mark.elapsed().as_millis();
        println!("part2: Line nr {} = '{}' got {} solutions in {}ms", line_nr, trimmed, ways_to_solve, duration);
        toreturn += ways_to_solve
    }
    toreturn.try_into().unwrap()
}



#[allow(dead_code)]
struct P2sync {
    line_nr: usize,
    line: String,
    count: i64,
}








#[allow(dead_code)]
pub fn part2x(input: &str) -> i64 {
    let known_solcounts: HashMap<&str, i64> = HashMap::from([
        ("???.### 1,1,3", 1),
        ("????.######..#####. 1,6,5", 2500),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 16),
        (".??..??...?##. 1,1,3", 16384),
        ("?###???????? 3,2,1", 506250),
        (".###..#?#?? 3,4", 1),
        (".??##???##.? 9,1", 16),
        ("???###..##? 4,2", 1),
        ("????.?####? 4,5", 162),
        ("...###???#?. 4,2", 32),
        ("?????#?#?#?# 2,5", 1),
        ("??##??.#.? 4,1", 243),
        ("??#?#????#.... 3,2", 1),
        (".#????#.?#?##?# 1,3,7", 16),
        ("???#????????#. 7,1", 2500),
        ("??#???#???##???? 4,7", 1875),
        ("#???#???#?.#?? 1,6,1", 32),
        ("..????#?.???.?#?. 4,3", 32),
        ("????#?#???#.# 5,2,1,1", 1),
        ("??.?#.?#?#?#? 1,2,4,2", 32),
        (".??##??.????#? 4,5", 39366),
        ("??#??#?.?#?? 1,2,2", 1024),
        ("?.#.?#??#???? 1,1,3,4", 16),
        ("???#?###????#??#.?? 1,13,1", 10706),
        (".???#?.??..#. 3,1,1", 9604),
        ("?????#.#?. 1,1,1", 18928),
        ("?#?#???.###?#.? 1,1,2,3,1", 81),
        ("?..???.?#?. 2,2", 32064),
        ("???#?#??#.#..???? 7,1,1,3", 162),
        ("??#???..#?? 2,1,1", 43053),
        ("??.#??#??? 2,3", 1),
        ("??#??#?.?.#. 4,1", 1),
        ("??##??##?. 1,2,3", 512),
        ("??#??#?#???????# 9,3", 3888),
        ("???#?????#???#.#? 2,8,2", 32),
        ("???#?##????. 1,6", 67864),
        ("?.???????#????.? 4,6", 243),
        ("#???.?#????#...? 3,4,2,1", 1),
        ("?..??#????#.? 1,2,3", 8358),
        ("???#??#??#.? 2,2,1,1", 54106),
        ("?#?#.#????.?#??# 4,1,1,2,1", 7776),
        ("????.??###?????##.?# 3,5,5,2", 5184),
        ("?????##???? 2,5", 242008),
        (".?.????#????# 2,4,1", 20394),
        ("?#?.#?.#???.???##?# 1,1,1,1,7", 512),
        ("????.???#?#?? 2,6", 413712),
        ("?#??#??#????#???## 5,1,1,1,1,2", 1024),


    ]);
    
    
    let lines: Vec<&str> = input.split("\n").collect();
    let mut toreturn = 0;

    for line_nr in 0..60 {
        let line = lines.get(line_nr).unwrap().clone();
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }

        if known_solcounts.contains_key(trimmed) {
            continue;
        }

        let (tx, rx) = mpsc::channel();
        thread::spawn( move || {

            let todo: Arc<P2sync> = rx.recv().unwrap();
            let line_nr = todo.line_nr;
            let trimmed = todo.line.clone();

            let mut ss = SpringSolver::new();
            // let line = lines.get(line_nr).unwrap();
            let mark = Instant::now();    
            // let pm = ProgressMarker::from_content(&trimmed.to_string());
            let pm = ProgressMarker::from_content(&trimmed.to_string());
            ss.populate_solutions(&pm.unroll(5).formatted);
            let solcount = ss.solcount_for(&pm.unroll(5).formatted);
            let duration = mark.elapsed().as_millis();
            
            println!("part2: Line nr {} '{}' got {} solutions in {}ms", line_nr, trimmed, solcount, duration);
        });

        tx.send(Arc::new(P2sync{line:trimmed.to_string(), line_nr:line_nr, count:0})).unwrap();

        // toreturn += solcount
    }
    thread::sleep(Duration::from_millis(400000000));

    toreturn.try_into().unwrap()
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d12::tests --nocapture

    use super::*;
    use crate::input;


    #[test]
    fn test_unroll() {
        let pm = ProgressMarker::from_content(&"???.### 1,1,3".to_string());
        assert_eq!("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", pm.unroll(5).formatted);

        let pm = ProgressMarker::from_content(&".# 1".to_string());
        assert_eq!(".#?.#?.#?.#?.# 1,1,1,1,1", pm.unroll(5).formatted);
    }

    #[test]
    fn test_solver() {
        let mut ss = SpringSolver::new();

        ss.populate_solutions(&"# 1".to_string());
        assert_eq!(1, ss.solcount_for(&"# 1".to_string()));

        ss.populate_solutions(&"... 1".to_string());
        // println!("Solutions map is '{:#?}'", ss.solve_options.get(&"... 1".to_string()).unwrap());
        assert_eq!(0, ss.solcount_for(&"... 1".to_string()));

        ss.populate_solutions(&"#.# 1,1".to_string());
        // println!("Solutions map is '{:#?}'", ss.solve_options.get(&"#.# 1,1".to_string()).unwrap());
        assert_eq!(1, ss.solcount_for(&"#.# 1,1".to_string()));

        ss.populate_solutions(&"?.# 1,1".to_string());
        assert_eq!(1, ss.solcount_for(&"?.# 1,1".to_string()));

        let mut ss = SpringSolver::new();
        ss.populate_solutions(&"?.? 1,1".to_string());
        println!("Solutions map is '{:#?}'", ss.solve_options);
        assert_eq!(1, ss.solcount_for(&"?.? 1,1".to_string()));

        ss.populate_solutions(&"??? 1,1".to_string());
        assert_eq!(1, ss.solcount_for(&"??? 1,1".to_string()));

        ss.populate_solutions(&"?? 1,1".to_string());
        assert_eq!(0, ss.solcount_for(&"?? 1,1".to_string()));

        ss.populate_solutions(&"...??... 1,1".to_string());
        assert_eq!(0, ss.solcount_for(&"...??... 1,1".to_string()));


        ss.populate_solutions(&"?###???????? 3,2,1".to_string());
        assert_eq!(10, ss.solcount_for(&"?###???????? 3,2,1".to_string()));


        // Verify memoization.
        let pm = ProgressMarker::from_content(&"????.#...#... 4,1,1".to_string());
        println!("ss has {} memoized and {} non-memoized calls before populating solutions, with {} entries in solve dictionary.", ss.memoized_calls, ss.non_memoized_calls, ss.solve_options.len());
        ss.populate_solutions(&pm.unroll(3).formatted);
        println!("ss has {} memoized and {} non-memoized calls after populating solutions, with {} entries in solve dictionary.", ss.memoized_calls, ss.non_memoized_calls, ss.solve_options.len());
       
        let mark = Instant::now();
        // Same as above.
        println!("ss has {} memoized and {} non-memoized calls before memoized bench, with {} entries in solve dictionary.", ss.memoized_calls, ss.non_memoized_calls, ss.solve_options.len());
        ss.populate_solutions(&pm.unroll(3).formatted);
        println!("ss has {} memoized and {} non-memoized calls after memoized bench, with {} entries in solve dictionary.", ss.memoized_calls, ss.non_memoized_calls, ss.solve_options.len());
        let duration = mark.elapsed().as_millis();
        assert!(duration < 4);

        // assert_eq!(3,5);
        
        // println!("{:?}", ss.solve_options);

    }

    #[test]

    fn test_unroll_solver() {
        let actual = part2("???.### 1,1,3");
        assert_eq!(actual, 1);

        let actual = part2("????.#...#... 4,1,1");
        assert_eq!(actual, 16);

               

        // let actual = part2(".??..??...?##. 1,1,3");
        // assert_eq!(actual, 16384);

        let actual = part2("???.### 1,1,3");
        assert_eq!(actual, 1);
        let actual = part2("???.### 1,1,3");
        assert_eq!(actual, 1);

    }


    
    #[test]
    fn test_speed() {
        let mut ss = SpringSolver::new();
        let line = "??..???.?#????????? 1,3,2,1,1,1";
        let mark = Instant::now();
        let trimmed = line.trim();
        println!("ss has {} memoized and {} non-memoized calls before populating solutions, with {} entries in solve dictionary.", ss.memoized_calls, ss.non_memoized_calls, ss.solve_options.len());
        ss.populate_solutions(&trimmed.to_string());
        println!("ss has {} memoized and {} non-memoized calls after populating solutions, with {} entries in solve dictionary.", ss.memoized_calls, ss.non_memoized_calls, ss.solve_options.len());
        // ss.solcount_for(trimmed.to_string());
        let solcount = ss.solcount_for(&trimmed.to_string());
        let duration = mark.elapsed().as_millis();
        println!("test_speed: Line '{}' got {} solutions in {}ms", trimmed, solcount, duration);
        // println!("{:#?}", ss.solve_options);
        assert!(duration < 10);
        //  got 60 solutions in 78ms
    }


    #[test]
    fn test_ways_to_solve() {
        let expected: HashMap<&str, i64> = HashMap::from([
            ("???.### 1,1,3", 1),
            ("????.######..#####. 1,6,5", 2500),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            (".??..??...?##. 1,1,3", 16384),
            ("?###???????? 3,2,1", 506250),
            (".###..#?#?? 3,4", 1),
            (".??##???##.? 9,1", 16),
            ("???###..##? 4,2", 1),
            ("????.?####? 4,5", 162),
            ("...###???#?. 4,2", 32),
            ("?????#?#?#?# 2,5", 1),
            ("??##??.#.? 4,1", 243),
            ("??#?#????#.... 3,2", 1),
            (".#????#.?#?##?# 1,3,7", 16),
            ("???#????????#. 7,1", 2500),
            ("??#???#???##???? 4,7", 1875),
            ("#???#???#?.#?? 1,6,1", 32),
            ("..????#?.???.?#?. 4,3", 32),
            ("????#?#???#.# 5,2,1,1", 1),
            ("??.?#.?#?#?#? 1,2,4,2", 32),
            (".??##??.????#? 4,5", 39366),
            ("??#??#?.?#?? 1,2,2", 1024),
            ("?.#.?#??#???? 1,1,3,4", 16),
            ("???#?###????#??#.?? 1,13,1", 10706),
            (".???#?.??..#. 3,1,1", 9604),
            ("?????#.#?. 1,1,1", 18928),
            ("?#?#???.###?#.? 1,1,2,3,1", 81),
            ("?..???.?#?. 2,2", 32064),
            ("???#?#??#.#..???? 7,1,1,3", 162),
            ("??#???..#?? 2,1,1", 43053),
            ("??.#??#??? 2,3", 1),
            ("??#??#?.?.#. 4,1", 1),
            ("??##??##?. 1,2,3", 512),
            ("??#??#?#???????# 9,3", 3888),
            ("???#?????#???#.#? 2,8,2", 32),
            ("???#?##????. 1,6", 67864),
            ("?.???????#????.? 4,6", 243),
            ("#???.?#????#...? 3,4,2,1", 1),
            ("?..??#????#.? 1,2,3", 8358),
            ("???#??#??#.? 2,2,1,1", 54106),
            ("?#?#.#????.?#??# 4,1,1,2,1", 7776),
            ("????.??###?????##.?# 3,5,5,2", 5184),
            ("?????##???? 2,5", 242008),
            (".?.????#????# 2,4,1", 20394),
            ("?#?.#?.#???.???##?# 1,1,1,1,7", 512),
            ("????.???#?#?? 2,6", 413712),
            ("?#??#??#????#???## 5,1,1,1,1,2", 1024)]);



        for (line, expected_ways) in expected.iter() {
            let trimmed = line.trim();
            let mut ss = SpringSolver::new();
            let frags: Vec<&str> = trimmed.split(" ").collect();
            let pattern = frags.get(0).unwrap().to_string();
            let constraints =frags.get(1).unwrap().to_string();            

            let actual = ss.ways_to_solve(&pattern, &constraints);
            if actual != *expected_ways {
                println!("ways_to_solve('{}', '{}') -> {}, expected {}.", pattern, constraints, actual, expected_ways)
            }
            assert_eq!(*expected_ways, ss.ways_to_solve(&pattern, &constraints));
        }
    }


    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d12::tests --nocapture

        if false {
            let pbuf = input::get_input("2023_d12_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 21); // p1 sample
        }
        if false {
            let pbuf = input::get_input("2023_d12.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 7379); // p1 skarp
        }


        if false {
            let pbuf = input::get_input("2023_d12_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 525152); // p2 sample
          
        }
        {
            let pbuf = input::get_input("2023_d12.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 202020202); // p2 skarp
        }    
    }
}



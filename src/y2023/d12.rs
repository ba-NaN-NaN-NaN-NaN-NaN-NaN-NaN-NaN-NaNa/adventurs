use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::time::Instant;


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
        

        if constraints.len() == 0 {
            if pattern.contains("#") {
                self.update_ways_to_solve(pattern, constraints, 0);
                return self.ways_to_solve(pattern, constraints);
            } else {
                self.update_ways_to_solve(pattern, constraints, 1);
                return self.ways_to_solve(pattern, constraints);
            }
        }

        if !constraints.contains(",")  {
            let constraint0 = constraints.parse::<usize>().unwrap();
            if pattern.len() == constraint0 {
                // We match length of pattern exactly. Nothing to chop before/after.
                if pattern.contains(".") {
                    self.update_ways_to_solve(pattern, constraints, 0);
                    return self.ways_to_solve(pattern, constraints);
                } else {
                    self.update_ways_to_solve(pattern, constraints, 1);
                    return self.ways_to_solve(pattern, constraints);
                }
            }

            if pattern.len() < constraint0 {
                // Can not fulfill.
                self.update_ways_to_solve(pattern, constraints, 0);
                return self.ways_to_solve(pattern, constraints);
            }
        }

        if pattern.len() == 0 {
            if constraints.len() == 0 {
                self.update_ways_to_solve(pattern, constraints, 1);
                return self.ways_to_solve(pattern, constraints);        
            } else {
                self.update_ways_to_solve(pattern, constraints, 0);
                return self.ways_to_solve(pattern, constraints);
            }
        }

        let (head, tail) = pattern.split_at(1);
        let ways = match head {
            "." => {
                self.ways_to_solve(&pattern.trim_start_matches(".").to_string(), constraints)
            },
            "#" => {
                let (constraint0, remaining_constraints) = destem_first_constraint(constraints);
                if constraint0 > pattern.len() {
                    0
                } else {
                    let must_be_gearable = pattern.get(0..constraint0).unwrap();
                    if must_be_gearable.contains(".") {
                        0
                    } else {
                        if pattern.len() == must_be_gearable.len() {
                            if constraints.contains(",") {
                                0 // Full input pattern exactly matches our FIRST constraint, BUT we have more.
                            } else {
                                1 // Full input pattern exactly matches our only constraint.
                            }
                        } else {
                            let must_be_spaceable = pattern.get(constraint0..constraint0+1).unwrap();
                            if must_be_spaceable == "#" {
                                0
                            } else {
                                let remaining_pattern: String = pattern.get(constraint0+1..).unwrap().to_string();
                                // println!("ways_to_solve({}, {}) ate {} worth of gear, will now recur with ways_to_solve({},{})", pattern, constraints, must_be_gearable, remaining_pattern, remaining_constraints);
                                self.ways_to_solve(&remaining_pattern, &remaining_constraints)
                            }
                        }
                    }
                }
            },
            "?" => { 
                // println!("Will disambiguate leading '?' of (?{}, {})", tail, constraints); 
                self.ways_to_solve(&format!(".{}", tail).to_string(), constraints) + 
                self.ways_to_solve(&format!("#{}", tail).to_string(), constraints)
            },
            _ => { 
                println!("Unhandled head {} in pattern {}.", head, pattern);
                panic!("jklfgd");
            },
        };

        
        self.update_ways_to_solve(pattern, constraints, ways);
        self.ways_to_solve(pattern, constraints)
    }
}

#[allow(dead_code)]
fn destem_first_constraint(constraints: &String) -> (usize, String) {
    match constraints.find(",") {
        Some(n) => {
            let (head, tail) = constraints.split_at(n);
            (head.parse::<usize>().unwrap(), tail.get(1..).unwrap().to_string())
        },
        None => {
            (constraints.parse::<usize>().unwrap(), "".to_string())
        },
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
    let lines: Vec<&str> = input.split("\n").collect();
    let mut toreturn = 0;

    for line in lines {
        let mark = Instant::now();
        let trimmed = line.trim();
        if trimmed.len() == 0 { continue; }
        let mut ss = SpringSolver::new();
        let frags: Vec<&str> = trimmed.split(" ").collect();
        let pattern = frags.get(0).unwrap().to_string();
        let constraints = frags.get(1).unwrap().to_string();
        let solcount = ss.ways_to_solve(&pattern, &constraints);
        let duration = mark.elapsed().as_millis();
        // println!("part1: Line '{}' got {} solutions in {}ms", trimmed, solcount, duration);
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
        let mark = Instant::now();
        let trimmed = line.trim();
        if trimmed.len() == 0 { continue; }
        let pm = ProgressMarker::from_content(&trimmed.to_string()); 
        let unrolled = pm.unroll(5);
        let mut ss = SpringSolver::new();
        let frags: Vec<&str> = unrolled.formatted.split(" ").collect();
        let pattern = frags.get(0).unwrap().to_string();
        let constraints = frags.get(1).unwrap().to_string();

        let ways_to_solve = ss.ways_to_solve(&pattern, &constraints);

        let duration = mark.elapsed().as_millis();
        // println!("part2: Line nr {} = '{}' got {} solutions in {}ms", line_nr, trimmed, ways_to_solve, duration);
        toreturn += ways_to_solve
    }
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
        assert_eq!(10, ss.ways_to_solve(&"?###????????".to_string(), &"3,2,1".to_string()));
        assert_eq!(0, ss.ways_to_solve(&"..#".to_string(), &"1,1".to_string()));
        assert_eq!(1, ss.ways_to_solve(&"#".to_string(), &"1".to_string()));
        assert_eq!(0, ss.ways_to_solve(&"...".to_string(), &"1".to_string()));
        assert_eq!(1, ss.ways_to_solve(&"#.#".to_string(), &"1,1".to_string()));
        assert_eq!(0, ss.ways_to_solve(&"?..".to_string(), &"1,1".to_string()));
        assert_eq!(1, ss.ways_to_solve(&"?.#".to_string(), &"1,1".to_string()));
        assert_eq!(1, ss.ways_to_solve(&"?.?".to_string(),&"1,1".to_string()));
        assert_eq!(1, ss.ways_to_solve(&"???".to_string(), &"1,1".to_string()));
        assert_eq!(0, ss.ways_to_solve(&"??".to_string(), &"1,1".to_string()));
        assert_eq!(0, ss.ways_to_solve(&"...??...".to_string(), &"1,1".to_string()));
    }

    #[test]

    fn test_unroll_solver() {
        let actual = part2("???.### 1,1,3");
        assert_eq!(actual, 1);

        let actual = part2("????.#...#... 4,1,1");
        assert_eq!(actual, 16);

        let actual = part2("???.### 1,1,3");
        assert_eq!(actual, 1);
        let actual = part2("???.### 1,1,3");
        assert_eq!(actual, 1);
    }   

    #[test]
    fn test_ways_to_solve() {
        // These test datas taken from brute-force attempt in other file.
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

            let pm = ProgressMarker::from_content(&trimmed.to_string()); 
            let unrolled = pm.unroll(5);
            let mut ss = SpringSolver::new();
            let frags: Vec<&str> = unrolled.formatted.split(" ").collect();
            let pattern = frags.get(0).unwrap().to_string();
            let constraints = frags.get(1).unwrap().to_string();

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

        {
            let pbuf = input::get_input("2023_d12_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 21); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d12.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 7379); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d12_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 525152); // p2 sample
          
        }
        {
            let pbuf = input::get_input("2023_d12.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 7732028747925); // p2 skarp
        }    
    }
}



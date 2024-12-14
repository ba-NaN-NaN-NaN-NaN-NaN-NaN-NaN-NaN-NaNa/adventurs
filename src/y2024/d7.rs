use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash, thread::panicking};
use regex::Regex;
use std::str::FromStr;

use lazy_static::lazy_static;

use crate::{grids::basic2d::{self, Grid2d, GridDirs8}, input};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Equation {
    value: i64,
    terms: Vec<i64>
}

pub fn possible_values(values: &Vec<i64>) -> HashSet<i64> {
    let mut toreturn: HashSet<i64> = HashSet::new();

    if values.len() == 1 {
        toreturn.insert(values[0]);
        return toreturn
    }

    if values.len() == 2 {
        toreturn.insert(values[0]+values[1]);
        toreturn.insert(values[0]*values[1]);

        let concated: String = format!("{}{}", values[0], values[1]);
        toreturn.insert(i64::from_str(concated.as_str()).unwrap());

        return toreturn
    }

    let (head, tail) = values.split_at(2);
    let mut possible_1: Vec<i64> = Vec::new();
    possible_1.push(head[0]*head[1]);
    possible_1.extend(tail);

    let mut possible_2: Vec<i64> = Vec::new();
    possible_2.push(head[0]+head[1]);
    possible_2.extend(tail);

    let mut possible_3: Vec<i64> = Vec::new();
    let concated: String = format!("{}{}", values[0], values[1]);
    possible_3.push(i64::from_str(concated.as_str()).unwrap());
    possible_3.extend(tail);

    toreturn.extend(possible_values(&possible_1));
    toreturn.extend(possible_values(&possible_2));
    toreturn.extend(possible_values(&possible_3));

    toreturn
}

impl Equation {
    pub fn from_line(line: &str) -> Equation {
        let mut ints = input::line_to_intvec(line.replace(":", " ").as_str());
        let (v, t) = ints.split_at_mut(1);
        let toreturn = Equation { 
            value: v[0],
            terms: t.to_vec(),
        };
        println!("Parsed '{}' -> '{:?}'. Built into {:?}", line, ints, toreturn);
        toreturn   
    }

    
}

#[allow(dead_code)]
pub fn permute_expr(digits:Vec<String>) -> HashSet<String> {
    let mut toreturn: HashSet<String> = HashSet::new();
    if digits.len() == 1 {
        toreturn.insert(format!("{}", digits[0]));
        return toreturn
    } else if digits.len() == 2 {
        toreturn.insert(format!("{}+{}", digits[0], digits[1]));
        toreturn.insert(format!("{}*{}", digits[0], digits[1]));
        toreturn.insert(format!("{}{}", digits[0], digits[1]));
    } else {
        let eval1 = format!("{}+{}", digits[0], digits[1]);
        let eval2 = format!("{}*{}", digits[0], digits[1]);
        let eval3 = format!("{}{}", digits[0], digits[1]);

        let mut new_vec_1: Vec<String> = Vec::new();
        new_vec_1.push(eval1);

        let mut new_vec_2: Vec<String> = Vec::new();
        new_vec_2.push(eval2);

        let mut new_vec_3: Vec<String> = Vec::new();
        new_vec_3.push(eval3);

        let (_, trailing) = digits.split_at(2);
        new_vec_1.extend(trailing.iter().map(|l| l.to_owned()));
        new_vec_2.extend(trailing.iter().map(|l| l.to_owned()));
        new_vec_3.extend(trailing.iter().map(|l| l.to_owned()));

        toreturn.extend(permute_expr(new_vec_1));
        toreturn.extend(permute_expr(new_vec_2));
        toreturn.extend(permute_expr(new_vec_3));
    }

    // if digits.len
    toreturn
}


pub fn evaluated(expr: &String) -> i64 {
    lazy_static! {
        static ref RE_FIRST: Regex = Regex::new(r"^(\d+)(.*)").unwrap();
        static ref RE_FOLLOWING: Regex = Regex::new(r"^([^0-9])(\d+)(.*)").unwrap();
    }

    let mut accumulator: i64 = 0;
    let mut rest: String = "".to_string();
    if let Some(str) = RE_FIRST.captures(&expr) {
        let digit_to_parse: String = str[1].to_string();
        accumulator = i64::from_str(digit_to_parse.as_str()).unwrap();
        rest = str[2].to_string();
    } else {
        panic!("gölfkh")
    }

    while rest.len() > 0 {
        if let Some(str) = RE_FOLLOWING.captures(&rest) {
            let operator: String = str[1].to_string();
            let digit_to_parse: String = str[2].to_string();
            // accumulator = i64::from_str(digit_to_parse.as_str()).unwrap();
            if operator == '+'.to_string() {
                accumulator += i64::from_str(digit_to_parse.as_str()).unwrap();
            } else if operator == '*'.to_string() {
                accumulator *= i64::from_str(digit_to_parse.as_str()).unwrap();
            }
            rest = str[3].to_string();
        } else {
            panic!("p9hp7tr")
        }

    }

    accumulator
}



#[allow(dead_code)]
pub struct Day4 {
    equations: Vec<Equation>,
    memoized_possibilities: HashMap<Vec<i64>, HashSet<i64>>
}

impl Day4 {
    pub fn from_lines(content: &str) -> Day4 {
        let mut equations: Vec<Equation> = Vec::new();
        let memoized_possibilities: HashMap<Vec<i64>, HashSet<i64>> = HashMap::new();
        for line in content.split("\n") {
            let trimmed = line.trim();
            if trimmed == "" {
                continue;
            }
            let eq = Equation::from_line(trimmed);
            equations.push(eq);
        }

        Day4 {
            equations,
            memoized_possibilities
        }
    }



    pub fn possibilities_simple(&mut self, terms: Vec<i64>) -> HashSet<i64> {
        let mut worklist: VecDeque<i64> = VecDeque::from_iter(terms);
        let mut toreturn: HashSet<i64> = HashSet::new();
        toreturn.insert(worklist.pop_front().unwrap());

        while worklist.len() > 0 {
            let mut new_toreturn: HashSet<i64> = HashSet::new();
            let popped = worklist.pop_front().unwrap();

            for elem in toreturn.iter() {
                new_toreturn.insert(elem+popped);
                new_toreturn.insert(elem*popped);
            }

            toreturn = new_toreturn;
        }

        toreturn
    }



    pub fn possibilities_2(&self, accumulator: i64, remaining: Vec<i64>) -> HashSet<i64> {
        let mut toreturn: HashSet<i64>  = HashSet::new();
        toreturn
    }

    pub fn possibilities_simple2(&mut self, terms: Vec<i64>) -> HashSet<i64> {
        let mut worklist: VecDeque<i64> = VecDeque::from_iter(terms);
        let mut toreturn: HashSet<i64> = HashSet::new();
        toreturn.insert(worklist.pop_front().unwrap());

        while worklist.len() > 0 {
            let mut new_toreturn: HashSet<i64> = HashSet::new();
            let popped = worklist.pop_front().unwrap();

            for elem in toreturn.iter() {
                new_toreturn.insert(elem+popped);
                new_toreturn.insert(elem*popped);
            }

            toreturn = new_toreturn;
        }

        toreturn
    }

    pub fn possibilities(&mut self, terms: Vec<i64>) -> HashSet<i64> {
        if self.memoized_possibilities.contains_key(&terms) {
            return self.memoized_possibilities.get(&terms).unwrap().clone()
        }

        let mut toreturn: HashSet<i64> = HashSet::new();
        if terms.len() == 1 {
            toreturn.insert(terms[0]);
            self.memoized_possibilities.insert(terms, toreturn.clone());
            return toreturn
        }

        for n in 1..terms.len() {
            let (left, right) = terms.split_at(n);
            let left_possibilities = self.possibilities(left.to_vec());
            let right_possibilities = self.possibilities(right.to_vec());
            for i in left_possibilities.iter() {
                for j in right_possibilities.iter() {
                    toreturn.insert(i*j);
                    toreturn.insert(i+j);
                }
            }
        }
        self.memoized_possibilities.insert(terms, toreturn.clone());
        // println!("Possibilities from '{:?}' are '{:?}'", terms, toreturn);

        return toreturn
    }

    pub fn part1(&mut self) -> i64 {
        let mut res = 0;
        let eqs = self.equations.clone();
        for eq in eqs.iter() {
            println!("will part1 eq {:?}", eq);
            if self.possibilities_simple(eq.terms.clone()).contains(&eq.value) {
                res += eq.value;
            }
        }

        res
    }

    pub fn part2(&mut self) -> i64 {
        let mut res = 0;
        let eqs = self.equations.clone();
        for eq in eqs.iter() {

            let possibilities = possible_values(&eq.terms);
            if possibilities.contains(&eq.value) {
                res += eq.value;
                println!("Eq {:?} OK!", eq);
            }
        }

        res
    }


    pub fn part2_x(&mut self) -> i64 {
        let mut res = 0;
        let eqs = self.equations.clone();
        for eq in eqs.iter() {

            let mut eq_found_ok = false;
            let terms_as_string: Vec<String> = eq.terms.iter().map(|l| format!("{}", l)).collect();
            let expressions = permute_expr(terms_as_string);
            

            for expr in expressions.iter() {
                if !eq_found_ok && evaluated(expr) == eq.value {
                    println!("OK! {} -> {}", expr, eq.value);
                    eq_found_ok = true
                }
            }

            println!("Eq with val={} and terms '{:?}' got expressions '{:?}'. eq_found_ok={}", eq.value, eq.terms, expressions, eq_found_ok);
            if eq_found_ok {
                res += eq.value
            }
        }

        res
    }

    pub fn permute_expressions(&mut self, permute_expressions: Vec<i64>) {

    }


    pub fn part2_nah(&mut self) -> i64 {
        let mut res = 0;
        let eqs = self.equations.clone();
        for eq in eqs.iter() {
            println!("will part1 eq {:?}", eq);
            if self.possibilities(eq.terms.clone()).contains(&eq.value) {
                res += eq.value;
            }
        }

        res
    }

}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut d7 = Day4::from_lines(input);    
    let toreturn = d7.part1();
    // d7.show();
    // println!("{:?}", d7.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d7 = Day4::from_lines(input);    
    let toreturn = d7.part2();
    // d7.show();
    // println!("{:?}", d7.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d7::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_permute() {
        let mut input: Vec<String> = Vec::new();
        input.push("5".to_string());
        let mut actual = permute_expr(input.clone());
        let mut expected: HashSet<String> = HashSet::new();
        expected.insert("5".to_string());
        assert_eq!(expected, actual);

        input.push("7".to_string());
        expected.clear();
        expected.insert("5+7".to_string());
        expected.insert("5*7".to_string());
        expected.insert("57".to_string());
        actual = permute_expr(input.clone());
        assert_eq!(expected, actual);


        input.push("1".to_string());
        expected.clear();
        expected.insert("5+7+1".to_string());
        expected.insert("5*7+1".to_string());
        expected.insert("57+1".to_string());
        expected.insert("5+7*1".to_string());
        expected.insert("5*7*1".to_string());
        expected.insert("57*1".to_string());
        expected.insert("5+71".to_string());
        expected.insert("5*71".to_string());
        expected.insert("571".to_string());
        actual = permute_expr(input.clone());
        assert_eq!(expected, actual);

    }
    #[test]
    fn test_evaluated() {
        assert_eq!(5, evaluated(&"5".to_string()));
        assert_eq!(10, evaluated(&"5+5+0".to_string()));
        assert_eq!(5, evaluated(&"0+5".to_string()));
        assert_eq!(5, evaluated(&"5+0".to_string()));
        assert_eq!(10, evaluated(&"5+1+1+1+1+1".to_string()));
        assert_eq!(9, evaluated(&"5+1+1+1*1+1".to_string()));
    }


    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d7::tests --nocapture
        if false {
            let pbuf = input::get_input("2024_d7_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 3749); // p1 sample
        }
        if false {
            let pbuf = input::get_input("2024_d7.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert!(actual < 2314937655662);
            assert_eq!(actual, 2314935962622); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d7_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 11387); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d7.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 401477450831495); // p2 skarp
        }
    }
}

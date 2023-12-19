use regex::Regex;
use rand::Rng;

use std::{iter, collections::{VecDeque, HashMap, HashSet}, time::{Instant, Duration}, ops::Add};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Factory {
    workflows: Vec<Workflow>,
    goto_table: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

#[allow(dead_code)]
impl Factory {
    pub fn from_content(content: &str) -> Factory {

        let sections: Vec<&str> = content.trim().split("\n\n").collect();
        let workflow_lines: Vec<&str> = sections.get(0).unwrap().trim().split("\n").collect();
        let parts_lines: Vec<&str> = sections.get(1).unwrap().trim().split("\n").collect();    

        let workflows: Vec<Workflow> = workflow_lines.iter().map( |l| {
            Workflow::from_line(l)
        }).collect();

        let parts: Vec<Part> = parts_lines.iter().map( |l| {
            Part::from_line(l)
        }).collect();

        let mut goto_table: HashMap<String, Workflow> = HashMap::new();
        for wf in workflows.iter() {
            goto_table.insert(wf.name.clone(), wf.clone());
        }
        
        Factory { 
            workflows,
            goto_table,
            parts
        }
    }


    pub fn qc_part(&self, part:&Part) -> Outcome {
        let mut next_workflow = self.goto_table.get("in").unwrap();
        loop {
            let result = next_workflow.apply_to(part);
            match result {
                Outcome::Accept => { return result },
                Outcome::Reject => { return result },
                Outcome::None => { panic!("flkdhfdhl") },
                Outcome::Goto(wf_name) => {
                    next_workflow = self.goto_table.get(wf_name.as_str()).unwrap();
                },
            }
        };
    }

    pub fn part1(&mut self) -> i64 {
        let mut toreturn = 0;
        for part in self.parts.iter() {
            let result = self.qc_part(part);
            match result {
                Outcome::Accept => { toreturn += part.sum_ratings(); },
                Outcome::Reject => {  },
                _ => { panic!("fdhfd" )},
            }
        }
        
        toreturn
    }

    pub fn part2_estimate(&mut self) -> i64 {
        let possible_combos:f64 = 3999.0*3999.0*3999.0*3999.0;
        let mut accepted:f64 = 0.0;
        let mut rejected:f64 = 0.0;

        let mark = Instant::now();
        let deadline = mark.add(Duration::from_secs(1 * 500));

        while Instant::now() < deadline {
            println!("Estimating...");
            for _ in 0..50000 * 500 {
                let part = Part{
                    x:  rand::thread_rng().gen_range(1..=4000),
                    m:  rand::thread_rng().gen_range(1..=4000),
                    a:  rand::thread_rng().gen_range(1..=4000),
                    s:  rand::thread_rng().gen_range(1..=4000),
                };
                match self.qc_part(&part) {
                    Outcome::Accept => { accepted += 1.0 },
                    Outcome::Reject => { rejected += 1.0 },
                    _ => { panic!("fdh")},
                }
            }
        }

        let ratio:f64  = accepted  / (accepted+rejected);
        let toreturn:i64 = (possible_combos * ratio) as i64;
        println!("part2 estimate is {}. accepted={}, rejected={}, ratio={}", toreturn, accepted, rejected, ratio);
        toreturn
    }

    pub fn part2(&mut self) -> i64 {
        let mut span_dividors: HashSet<String> = HashSet::new();
        for wf in self.workflows.iter() {
            span_dividors.extend(wf.outcome_span_dividors());
        }
        println!("Got span dividors {:?}", span_dividors);
        let eq_classes = EqClasses::from_span_dividors(&span_dividors, &self);
        return eq_classes
        /*
        // println!("Got eq_classes {:?}", eq_classes);


        let mut toreturn = 0;

        for eq_class in eq_classes.iter() {
            let representation = eq_class.sample();
            if self.qc_part(&representation) == Outcome::Accept {
                let count = eq_class.count();
                / *
                println!("x={}..{},m={}..{},a={}..{},s={}..{} -> Represents {} accepted.", 
                eq_class.x_low,    eq_class.x_high,    eq_class.m_low,    eq_class.m_high,    eq_class.a_low,    eq_class.a_high,    eq_class.s_low,    eq_class.s_high,
                count);
                * /
            
                toreturn += count;
            }

        }




        toreturn
         */
    }

}



#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[allow(dead_code)]
impl Part {
    pub fn from_line(line: &str) -> Part {
        let re = Regex::new(r"^.x=(?P<x>[0-9]+),m=(?P<m>[0-9]+),a=(?P<a>[0-9]+),s=(?P<s>[0-9]+).$").unwrap();
        let groups = match re.captures(line) {
            Some(stuff) => stuff,
            None => { 
                println!("Could not regexp match part line : '{}'", line);
                panic!("Bad regexp for part line")
            },
        };

        // {x=2698,m=2050,a=10,s=1956}
        Part { 
            x: groups["x"].parse::<i64>().unwrap(),
            m: groups["m"].parse::<i64>().unwrap(),
            a: groups["a"].parse::<i64>().unwrap(),
            s: groups["s"].parse::<i64>().unwrap(),
        }
    }

    pub fn sum_ratings(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}


#[allow(dead_code)]
impl Workflow {
    pub fn from_line(line: &str) -> Workflow {
        let re = Regex::new(r"^(?P<name>[a-z]+)\{(?P<rules>.*)\}$").unwrap();
        let groups = match re.captures(line.trim()) {
            Some(stuff) => stuff,
            None => { 
                println!("Could not regexp rule from content : '{}'", line);
                panic!("Bad regexp for part line")
            },
        };

        let name = groups["name"].to_string();
        let rules_frags: Vec<&str> = groups["rules"].split(",").collect();

        let mut rules: Vec<Rule> = Vec::new();
        for frag in rules_frags {
            rules.push(Rule::from_content(frag));
        }
        
        Workflow { 
            name,
            rules
         }
    }

    pub fn apply_to(&self, part: &Part) -> Outcome {
        for rule in self.rules.iter() {
            let outcome = rule.apply_to(part);
            if outcome != Outcome::None {
                return outcome
            }
        }
        panic!("fkdhöklh ")
    }

    pub fn outcome_span_dividors(&self) -> HashSet<String> {
        let mut toreturn: HashSet<String> = HashSet::new();
        for rule in self.rules.iter() {
            toreturn.extend(rule.outcome_span_dividors());
        }
        toreturn
    }

}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
    CmpLt(String, i64, Outcome),
    CmpGt(String, i64, Outcome),
    Unconditional(Outcome),
}


#[allow(dead_code)]
impl Rule {
    pub fn from_content(content: &str) -> Rule {
        if content.contains("<") {
            let re = Regex::new(r"^(?P<varname>[xmas])<(?P<number>[0-9]+):(?P<outcome>[ARa-z]+)$").unwrap();
            let groups = match re.captures(content) {
                Some(stuff) => stuff,
                None => { 
                    println!("Could not regexp LT rule from content : '{}'", content);
                    panic!("Bad regexp for part line")
                },
            };
    
            Rule::CmpLt(
                groups["varname"].to_string(),
                groups["number"].parse::<i64>().unwrap(),
                Outcome::from_content(&groups["outcome"]),
            )
        } else if content.contains(">") {
            let re = Regex::new(r"^(?P<varname>[xmas])>(?P<number>[0-9]+):(?P<outcome>[ARa-z]+)$").unwrap();
            let groups = match re.captures(content) {
                Some(stuff) => stuff,
                None => { 
                    println!("Could not regexp rule from content : '{}'", content);
                    panic!("Bad regexp for part line")
                },
            };
    
            Rule::CmpGt(
                groups["varname"].to_string(),
                groups["number"].parse::<i64>().unwrap(),
                Outcome::from_content(&groups["outcome"]),
            )
        } else {
            Rule::Unconditional(Outcome::from_content(content))
        }
    }

    pub fn apply_to(&self, part: &Part) -> Outcome {
        match self {
            Rule::Unconditional(outcome) => { outcome.clone() },
            Rule::CmpLt(varname, value , outcome ) => {
                match varname.as_str() {
                    "x" => {
                        if part.x < *value { outcome.clone() } else { Outcome::None }
                    },
                    "m" => {
                        if part.m < *value { outcome.clone() } else { Outcome::None }
                    },
                    "a" => {
                        if part.a < *value { outcome.clone() } else { Outcome::None }
                    },
                    "s" => {
                        if part.s < *value { outcome.clone() } else { Outcome::None }
                    },
                    _ => { panic!("fldhjk") }
                }
            },
            Rule::CmpGt(varname, value , outcome ) => {
                match varname.as_str() {
                    "x" => {
                        if part.x > *value { outcome.clone() } else { Outcome::None }
                    },
                    "m" => {
                        if part.m > *value { outcome.clone() } else { Outcome::None }
                    },
                    "a" => {
                        if part.a > *value { outcome.clone() } else { Outcome::None }
                    },
                    "s" => {
                        if part.s > *value { outcome.clone() } else { Outcome::None }
                    },
                    _ => { panic!("fldhjk") }
                }
            },
        }
    }


    fn outcome_span_dividors(&self) -> HashSet<String> {
        // Return set of 
        let mut toreturn: HashSet<String> = HashSet::new();
        if let Rule::CmpGt(var_name, value ,_ ) = self {
            toreturn.insert(format!("{}>={}", var_name, value+1));
        } else if let Rule::CmpLt(var_name, value ,_ ) = self {
            toreturn.insert(format!("{}>={}", var_name, value));
        };

        toreturn
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome  {
    Accept,
    Reject,
    Goto (String), 
    None,
}

#[allow(dead_code)]
impl Outcome {
    pub fn from_content(content: &str) -> Outcome {
        match content {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            _ => Outcome::Goto(content.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct EqClasses {
    x_low: i64,
    x_high: i64,
    m_low: i64,
    m_high: i64,
    a_low: i64,
    a_high: i64,
    s_low: i64,
    s_high: i64,    
}

impl EqClasses {

    #[allow(dead_code)]
    // pub fn from_span_dividors(dividors: &HashSet<String>) -> Vec<EqClasses> {
    pub fn from_span_dividors(dividors: &HashSet<String>, factory: &Factory) -> i64 {
        // In: Set of rule strings of form 'x<=400,x<=600,a<=222' and builds
        // combinatory list of all such rules.
        //let mut toreturn : Vec<EqClasses> = Vec::new();
        let mut toreturn = 0;
        let mut x_starts: HashSet<i64> = HashSet::new();
        let mut m_starts: HashSet<i64> = HashSet::new();
        let mut a_starts: HashSet<i64> = HashSet::new();
        let mut s_starts: HashSet<i64> = HashSet::new();

        x_starts.insert(1); x_starts.insert(4001);
        m_starts.insert(1); m_starts.insert(4001);
        a_starts.insert(1); a_starts.insert(4001);
        s_starts.insert(1); s_starts.insert(4001);

        let re = Regex::new(r"^(?P<var_name>[xmas])>=(?P<value>[0-9]+)$").unwrap();


        for dividor in dividors.iter() {
            let groups = match re.captures(dividor) {
                Some(stuff) => stuff,
                None => { 
                    println!("Could not regexp match part line : '{}'", dividor);
                    panic!("Bad regexp for part line")
                },
            };

            let value = groups["value"].parse::<i64>().unwrap();
            let var_name  = &groups["var_name"];
            match var_name {
                "x" => { x_starts.insert(value) },
                "m" => { m_starts.insert(value) },
                "a" => { a_starts.insert(value) },
                "s" => { s_starts.insert(value) },
                _ => { panic!("fldhjk") }
            };
        };

        let mut sorted_x: Vec<&i64> = Vec::from_iter(x_starts.iter());
        sorted_x.sort();

        let mut sorted_m: Vec<&i64> = Vec::from_iter(m_starts.iter());
        sorted_m.sort();

        let mut sorted_a: Vec<&i64> = Vec::from_iter(a_starts.iter());
        sorted_a.sort();

        let mut sorted_s: Vec<&i64> = Vec::from_iter(s_starts.iter());
        sorted_s.sort();

        println!("x_starts is {:?}", x_starts);
        println!("sorted_x is {:?}", sorted_x);

        // With great loop nesting comes a runtime of "finished in 12143.26s".
        for xn in 0..sorted_x.len()-1 {
            let x_low = **sorted_x.get(xn).unwrap();
            let x_high = **sorted_x.get(xn+1).unwrap()-1;
            println!("Doing xn class {}...{}", x_low, x_high);
            for mn in 0..sorted_m.len()-1 {
                let m_low = **sorted_m.get(mn).unwrap();
                let m_high = **sorted_m.get(mn+1).unwrap()-1;
    
                for an in 0..sorted_a.len()-1 {
                    let a_low = **sorted_a.get(an).unwrap();
                    let a_high = **sorted_a.get(an+1).unwrap()-1;
        
                    for sn in 0..sorted_s.len()-1 {
                        let s_low = **sorted_s.get(sn).unwrap();
                        let s_high = **sorted_s.get(sn+1).unwrap()-1;
            
                        let eq_class = EqClasses {
                            x_low,
                            x_high,
                            m_low,
                            m_high,
                            a_low,
                            a_high,
                            s_low,
                            s_high,
                        };
                        let representation = eq_class.sample();
                        if factory.qc_part(&representation) == Outcome::Accept {
                            let count = eq_class.count();
                            toreturn += count;
                        }

                            /* 
                            toreturn.push(EqClasses {
                                x_low,
                                x_high,
                                m_low,
                                m_high,
                                a_low,
                                a_high,
                                s_low,
                                s_high,
                            });*/
                    }
                }
            }
        }

        toreturn     

    }

    #[allow(dead_code)]
    pub fn sample(&self) -> Part {
        // Sample part representing this span.
        Part { x: self.x_low, m: self.m_low, a: self.a_low, s: self.s_low }   
    }

    pub fn count(&self) -> i64 {
        (self.x_high-self.x_low+1) *
        (self.m_high-self.m_low+1) *
        (self.a_high-self.a_low+1) *
        (self.s_high-self.s_low+1)
    }
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d19::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_parse() {
        let part = Part::from_line("{x=2461,m=1339,a=466,s=291}");
        assert_eq!(part.x, 2461);
        assert_eq!(part.m, 1339);
        assert_eq!(part.a, 466);
        assert_eq!(part.s, 291);

        let rule = Rule::from_content("A");
        assert_eq!(rule, Rule::Unconditional(Outcome::Accept));
        assert_eq!(rule.apply_to(&part), Outcome::Accept);

        assert_eq!(Rule::from_content("x<2000:A").apply_to(&part), Outcome::None);
        assert_eq!(Rule::from_content("x>2000:A").apply_to(&part), Outcome::Accept);


        let workflow = Workflow::from_line("px{a<2006:qkq,m>2090:A,rfg}");
        assert_eq!(workflow.name, "px");
        assert_eq!(workflow.apply_to(&part), Outcome::Goto("qkq".to_string()));

    }

    #[test]
    fn test_part2_estimate() {
        if false {
            let pbuf = input::get_input("2023_d19_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Factory::from_content(&content);
            let actual = pool.part2_estimate();
            // Plus minus 10%.
            assert!(167409079868000 - 16740907986800 < actual && actual < 167409079868000 + 16740907986800);
        }
        {
            let pbuf = input::get_input("2023_d19.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Factory::from_content(&content);
            let actual = pool.part2_estimate();
            assert!(127404547784541 - 12740454778454 < actual && actual < 127404547784541 + 12740454778454);
        }        
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d19::tests --nocapture

        {
            let pbuf = input::get_input("2023_d19_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Factory::from_content(&content);
            let actual = pool.part1();
            assert_eq!(actual, 19114); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d19.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Factory::from_content(&content);
            let actual = pool.part1();
            assert_eq!(actual, 397134); // p1 skarp
        }

        {
            let pbuf = input::get_input("2023_d19_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Factory::from_content(&content);
            let actual = pool.part2();
            assert_eq!(actual, 167409079868000); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d19.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut pool = Factory::from_content(&content);
            let actual = pool.part2();
            assert_eq!(actual, 127517902575337); // p2 skarp
        }
    }
}


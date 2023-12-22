use regex::Regex;
use rand::Rng;

use std::{iter, collections::{VecDeque, HashMap, HashSet}, time::{Instant, Duration}, ops::Add, f32::consts::LN_10};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Circuitry {
    module_table: HashMap<String, Module>,
}

#[allow(dead_code)]
impl Circuitry {
    pub fn from_content(content: &str) -> Circuitry {
        let mut module_table: HashMap<String, Module> = HashMap::new();
        let mut modules_referenced: HashSet<String> = HashSet::new();

        for line in content.trim().split("\n") {
            let module = Module::from_line(line.trim());
            for out_name in module.outputs.iter() {
                modules_referenced.insert(out_name.clone());
            }
            module_table.insert(module.name.clone(), module);
        }

        for out_mname in modules_referenced {
            if !module_table.contains_key(&out_mname) {
                println!(" == WARNING == : Creating implicit module {} as broadcast", out_mname);
                let dummy: Module = Module { 
                    name: out_mname.to_string(), 
                    flipflop_state: false, 
                    mtype: ModuleType::Broadcast, 
                    inputs: HashMap::new(), 
                    outputs: Vec::new(), 
                };
                module_table.insert(out_mname.to_string(), dummy);
            }
        }

        let mut wires_to_connect: Vec<(String, String)> = Vec::new();

        for (in_mname, in_mod) in module_table.iter() {
            for out_mname in in_mod.outputs.iter() {
                wires_to_connect.push((in_mname.clone(), out_mname.clone()));
            }
        }

        for (in_mname, out_mname) in wires_to_connect.iter() {
            let out_mod = module_table.get_mut(out_mname).unwrap();
            out_mod.inputs.insert(in_mname.clone(), Pulse::Low);
        }
        
        let toreturn = Circuitry { 
            module_table,
        };

        // println!("parsed content {} into circuitry {:#?}", content, toreturn);

        toreturn
    }

    pub fn draw_as_plant(self) -> String {
        let mut frags :Vec<String> = Vec::new();
        frags.push("@startuml".to_string());

        frags.push("
rectangle \"\\n\\n    broadcaster    \\n\\n\" as broadcaster
rectangle \"\\n\\n    ng    \\n\\n\" as ng
rectangle \"\\n\\n    qb    \\n\\n\" as qb
rectangle \"\\n\\n    mp    \\n\\n\" as mp
rectangle \"\\n\\n    rx    \\n\\n\" as rx
rectangle \"\\n\\n    qt    \\n\\n\" as qt".to_string());
        
        for (_,module) in self.module_table.iter() {
            for dst in module.outputs.iter() {
                if module.name == "broadcaster" {
                    frags.push(format!("({}) --> ({})", module.name, dst));
                } else  if module.name == "qt" {
                    frags.push(format!("({}) --> ({})", module.name, dst));
                } else  if module.name == "ng" {
                    frags.push(format!("({}) --> ({})", module.name, dst));
                } else  if module.name == "qb" {
                    frags.push(format!("({}) --> ({})", module.name, dst));
                } else  if module.name == "mp" {
                    frags.push(format!("({}) --> ({})", module.name, dst));
                } else {
                    frags.push(format!("({}) --> ({})", module.name, dst));
                }
            }
        }

        frags.push("@enduml".to_string());
        frags.join("\n")
    }

    pub fn push_button(&mut self, debug_log: bool) -> (i64, i64, Vec<String>) {
        let mut pulse_worklist: VecDeque<(String, Pulse, String)> = VecDeque::new();
        let mut debug_log_out: Vec<String> = Vec::new();
        let mut low_pulse_count = 0;
        let mut high_pulse_count = 0;

        pulse_worklist.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

        while pulse_worklist.len() > 0 {
            let (origin, pulse, destination) = pulse_worklist.pop_front().unwrap();
            let dst_module = self.module_table.get_mut(&destination).unwrap();

            let pulse_str: &str;
            if pulse == Pulse::Low { 
                low_pulse_count += 1;
                pulse_str = "low";
            } else { 
                high_pulse_count += 1;
                pulse_str = "high";
            };

            if debug_log {
                debug_log_out.push(format!("{} -{}-> {}", origin, pulse_str, destination));
            }

            let new_pulses = dst_module.receive_pulse(pulse, origin);
            for (new_dst, new_pulse) in new_pulses {
                pulse_worklist.push_back((destination.clone(), new_pulse, new_dst));
            }
        }
        (low_pulse_count, high_pulse_count, debug_log_out)
    }

    pub fn part1(&mut self) -> i64 {
        let mut low_count_total:i64 = 0;
        let mut high_count_total:i64 = 0;

        for _ in 0..1000 {
            let (low_count_once, high_count_once,_) = self.push_button(false);
            low_count_total += low_count_once;
            high_count_total += high_count_once;
        }
        return low_count_total * high_count_total;
    }


    pub fn part2(&mut self) -> i64 {
        let mut pushes: i64 = 0;
        // let mut encountered_states: HashSet<String> = HashSet::new();

        loop {
            let mut pulse_worklist: VecDeque<(String, Pulse, String)> = VecDeque::new();
            /*
            let state: String = format!("{:?}", self);
            if encountered_states.contains(&state) {
                println!("After {} pushes, we are at state: {:#?}", pushes, self);
                panic!("We are looping.")
            }
            encountered_states.insert(state);
             */

            pulse_worklist.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
            pushes += 1;

            while pulse_worklist.len() > 0 {
                let (origin, pulse, destination) = pulse_worklist.pop_front().unwrap();
                let dst_module = self.module_table.get_mut(&destination).unwrap();

                let new_pulses = dst_module.receive_pulse(pulse, origin);
                for (new_dst, new_pulse) in new_pulses {

                    //if pushes % 100000 == 5 {
                    if new_dst == "dr" && new_pulse == Pulse::High {
                        println!("After {} pushes, pulse includes: {} --{:?}--> {}", pushes, destination, new_pulse, new_dst);
                        // println!("Verbose state is {:?}", self);
                    }

                    if new_dst == "rx" && new_pulse == Pulse::Low {
                        return pushes
                    }
                    pulse_worklist.push_back((destination.clone(), new_pulse, new_dst));
                }
                if pushes % 100000 == 5 {
                    // println!("Verbose state is {:?}", self);
                }

            }
        }
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    FlipFlop,
    Broadcast,
    Conjunction,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    name: String,
    flipflop_state: bool,
    mtype: ModuleType,
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>, // Needs to be vec, not hashset since we process in order.
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Pulse {
    Low,
    High,
}

#[allow(dead_code)]
impl Module {
    pub fn from_line(content: &str) -> Module {
        let name: String;
        let flipflop_state = false;
        let mtype: ModuleType;
        let inputs: HashMap<String, Pulse> = HashMap::new();
        let mut outputs: Vec<String> = Vec::new() ;


        let arrow_frags: Vec<&str> = content.split(" -> ").collect();
        if arrow_frags.len() != 2 {
            println!("Unparseable module from line '{}'", content);
            panic!("flhdfdljh")
        }

        let name_part =  arrow_frags.get(0).unwrap();
        let output_part = arrow_frags.get(1).unwrap();

        if name_part.starts_with("%") {
            mtype = ModuleType::FlipFlop;
            name = name_part[1..].to_string();
        } else if name_part.starts_with("&") {
            mtype = ModuleType::Conjunction;
            name = name_part[1..].to_string();
        } else {
            mtype = ModuleType::Broadcast;
            name = name_part[..].to_string();
        }

        for output in output_part.split(", ") {
            outputs.push(output.to_string());
        };

        let toreturn = Module { 
            name,
            flipflop_state,
            mtype,
            inputs,
            outputs,
        };

        // println!("parsed {} into module {:?}", output_part, toreturn);

        toreturn
    }

    pub fn receive_pulse(&mut self, pulse: Pulse, origin_name: String) -> Vec<(String, Pulse)> {

        match self.mtype {
            ModuleType::Broadcast => {
                
                let mut toreturn: Vec<(String, Pulse)> = Vec::new();
                for out_fname in self.outputs.iter() {
                    toreturn.push((out_fname.clone(), pulse.clone()));
                }
                return toreturn;
            },
            ModuleType::Conjunction => {
                /*
                Conjunction modules (prefix &) remember the type of the most recent pulse received 
                from each of their connected input modules; they initially default to remembering a 
                low pulse for each input. 
                
                // When a pulse is received, the conjunction module first updates its memory for that input. 
                
                 */
                self.inputs.insert(origin_name, pulse);


                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                //   --->>> if any of input history is low, send high pulse.
                //          else send low.
                let mut to_send = Pulse::Low;
                for (_, input_history) in self.inputs.iter() {
                    if *input_history == Pulse::Low {
                        to_send = Pulse::High
                    }
                };

                let mut toreturn: Vec<(String, Pulse)> = Vec::new();
                for out_fname in self.outputs.iter() {
                    toreturn.push((out_fname.clone(), to_send.clone()));
                }
                return toreturn;
            },
            ModuleType::FlipFlop => {
                if pulse == Pulse::High {
                    return Vec::new(); // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                }
                self.flipflop_state = !self.flipflop_state;
                let out_pulse = if self.flipflop_state {
                    Pulse::High
                } else {
                    Pulse::Low
                };
                
                let mut toreturn: Vec<(String, Pulse)> = Vec::new();
                for out_fname in self.outputs.iter() {
                    toreturn.push((out_fname.clone(), out_pulse.clone()));
                }
                return toreturn;
            },
        }
    }

}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d20::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_parse() {
        let module = Module::from_line("broadcaster -> a, b, c");

        assert_eq!(module.mtype, ModuleType::Broadcast);
        assert_eq!(module.outputs.len(), 3);
        assert_eq!(module.name, "broadcaster");
    }

    #[test]
    fn draw_graph() {
        let pbuf = input::get_input("2023_d20.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let mut circuit = Circuitry::from_content(&content);
        let plant = circuit.draw_as_plant();
        println!(" ==v== Got drawing ==v==\n{}\n ==^======^==", plant);
    }

    

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d20::tests --nocapture

        {
            let pbuf = input::get_input("2023_d20_sample1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut circuit = Circuitry::from_content(&content);
            let actual = circuit.part1();
            assert_eq!(actual, 32000000); // p1 sample 1
        }
        {
            let pbuf = input::get_input("2023_d20_sample2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut circuit = Circuitry::from_content(&content);
            let actual = circuit.part1();
            assert_eq!(actual, 11687500); // p1 sample 2
        }
        {
            let pbuf = input::get_input("2023_d20.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut circuit = Circuitry::from_content(&content);
            let actual = circuit.part1();
            assert_eq!(actual, 919383692); // p1 skarp
        }
        println!("Doing p2 skarp");
        {
            let pbuf = input::get_input("2023_d20.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let mut circuit = Circuitry::from_content(&content);
            let actual = circuit.part2();
            // 2335500005 is too low.

/*
pi@pi$ factor 3917
3917: 3917
pi@pi$ factor 3919
3919: 3919
pi@pi$ factor 4007
4007: 4007
pi@pi$ factor 4027
4027: 4027
pi@pi$ python3
Python 3.10.12 (main, Nov 20 2023, 15:14:05) [GCC 11.4.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 3917*3919*4007*4027
247702167614647
>>> 
 */
            assert_eq!("Does not work. To figure out, identify the 4 fan-in blocks to the 'dr' box from plantuml graph, then identify the periodicity by viewing debug output, then multiply together as above.", "true");
            assert_eq!(actual, 247702167614647); // p2 skarp
        }
    }
}


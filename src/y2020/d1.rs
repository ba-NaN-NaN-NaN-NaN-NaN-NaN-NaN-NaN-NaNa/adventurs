use std::str::FromStr;

use std::collections::HashSet;
use crate::structs::{I64Pair, I64Tri, Sort};

#[allow(dead_code)]
pub fn populate_i64_set(mut int_set: HashSet<i64>, lines: &str) -> HashSet<i64> {
    // Insert a bunch of integers into a set.
    // The integers can be provided as a text block.
    let owned = String::from(lines);
    let parts = owned.split("\n");
    for part in parts {
        // print!("Got a part {}", part);
        let s = part.trim();
        if s.len() > 0 {
            let my_int = i64::from_str(s).unwrap();
            int_set.insert(my_int);
        }
    }
    int_set
}

#[allow(dead_code)]
pub fn i64_set_builder(lines: &str) -> HashSet<i64> {
    return populate_i64_set(HashSet::new(), lines)
}

#[allow(dead_code)]
pub fn find_summable_pairs(wanted_sum: i64, int_set: &HashSet<i64>) -> Option<I64Pair> {
    // If hashset contains a + b such that a+b=2020, return .
    for i in int_set {
        let complement = wanted_sum - i;
        if int_set.contains(&complement) {
            let mut pair = I64Pair{x:*i,y:complement};
            pair.sort();
            return Some(pair);
        }
    }
    None
}

#[allow(dead_code)]
pub fn find_summable_tris(int_set: &HashSet<i64>) -> Option<I64Tri> {
    // If hashset contains a, b and c such that a+b+c=2020, return .
    for i in int_set {
        let complement = 2020 - i;
        match find_summable_pairs(complement, int_set) {
            Some(I64Pair{x,y}) => {
                let mut tri = I64Tri{x:*i,y:x,z:y};
                tri.sort();
                return Some(tri)
            },
            None => {},
        }
    }
    None
}

#[allow(dead_code)]
pub fn freq_summer(origin: i64, deltas: &str) -> i64 {
    let owned = String::from(deltas);
    let parts = owned.split(",");
    let mut acc = origin;
    for part in parts {
        // print!("Got a part {}\n", part);
        let s = part.trim();
        let my_int = i64::from_str(s).unwrap();
        acc += my_int
    }
    acc
}

use std::str::FromStr;

use std::collections::HashSet;

#[allow(dead_code)]
pub fn populate_i64_set(mut int_set: HashSet<i64>, lines: &str) -> HashSet<i64> {
    // Insert a bunch of integers into a set.
    // The integers can be provided as a text block.
    let owned = String::from(lines);
    let parts = owned.split("\n");
    for part in parts {
        print!("Got a part {}", part);
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
pub fn freq_summer(origin: i32, deltas: &str) -> i32 {
    let owned = String::from(deltas);
    let parts = owned.split(",");
    let mut acc = origin;
    for part in parts {
        print!("Got a part {}", part);
        let s = part.trim();
        let my_int = i32::from_str(s).unwrap();
        acc += my_int
    }
    // print!("Parts is {}", parts);
    // origin + parts.count() as i32
    acc
}

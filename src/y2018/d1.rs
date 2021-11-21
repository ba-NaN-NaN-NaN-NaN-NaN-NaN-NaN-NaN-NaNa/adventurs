use std::str::FromStr;

#[allow(dead_code)]
pub fn freq_summer(origin: i32, deltas: &str) -> i32 {
    let owned = String::from(deltas);
    let parts = owned.split(",");
    let mut acc = origin;
    for part in parts {
        //print!("Got a part {}", part);
        let s = part.trim();
        let my_int = i32::from_str(s).unwrap();
        acc += my_int
    }
    // print!("Parts is {}", parts);
    // origin + parts.count() as i32
    acc
}

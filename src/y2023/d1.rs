use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let lines = input.split("\n");
    let mut acc = 0; // origin;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        let mut digits = Vec::new();
        // println!("Got a part {}", line);
        let chars = line.split("");

        for char in chars {
            let my_int = char.parse::<i64>();
            match my_int {
                Ok(val) => {digits.push(val)}
                _ => {}
            }
        }

        let to_add = digits.get(0).unwrap() * 10 + digits.pop().unwrap();
        // println!("Adding {}", to_add);
        acc += to_add
    }
    // print!("Parts is {}", parts);
    // origin + parts.count() as i32
    acc
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    // let owned = String::from(deltas);
    let lines = input.split("\n");
    let mut acc = 0; // origin;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        let line2 = despell(line.to_string());
        let mut digits = Vec::new();
        // println!("Got a part {}", line2);
        let chars = line2.split("");

        for char in chars {
            let my_int = char.parse::<i64>();
            match my_int {
                Ok(val) => {digits.push(val)}
                _ => {}
            }
        }

        let to_add = digits.get(0).unwrap() * 10 + digits.pop().unwrap();
        // println!("Adding {}", to_add);
        acc += to_add
    }
    // println!("Parts is {}", parts);
    // origin + parts.count() as i32
    acc
}

#[allow(dead_code)]
pub fn despell(input: String) -> String {
    let toreturn = despell_leading(despell_trailing(input.clone()));
    // println!("despell({}) -> {}", input, toreturn);
    toreturn
}

#[allow(dead_code)]
pub fn despell_leading(input: String) -> String {
    let hmap = HashMap::from([
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
    
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),]);

    for (search, replace) in hmap.iter() {
        if input.starts_with(search) {
            let i = search.len();
            let toreturn = format!("{}{}", replace, input.get(i..).unwrap());
            return toreturn
        }
    }
    let to_iter = String::from(input.get(1..).unwrap());
    //  println!("Iterating leading {}", to_iter);
    despell_leading(to_iter)
}


#[allow(dead_code)]
pub fn despell_trailing(input: String) -> String {
    let hmap = HashMap::from([
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
    
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),]);

    for (search, replace) in hmap.iter() {
        if input.ends_with(search) {
            let i = input.len()-search.len();
            let toreturn = format!("{}{}", input.get(..i).unwrap(), replace);
            return toreturn
        }
    }
    let i = input.len();
    let to_iter = String::from(input.get(..i-1).unwrap());
    // println!("Iterating trailing {}", to_iter);
    despell_trailing(to_iter)
}


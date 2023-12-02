use std::collections::HashSet;

#[allow(dead_code)]
pub fn part1(input: &str) -> i64{
    let lines:Vec<&str> = input.split("\n").collect();
    let mut toreturn = 0;
    for line in lines  {
        let to_parse = line.trim();
        if to_parse.len() == 0 {
            continue;
        }
        // println!("Will parse {}.", to_parse);
        let change = to_parse.parse::<i64>().unwrap();
        toreturn += change;
    }
    return toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64{
    let lines:Vec<&str> = input.split("\n").collect();
    let mut encountered: HashSet<i64> = HashSet::<i64>::new();
    let mut current_freq = 0;
    
    for _ in 0..i64::MAX {
        for line in lines.clone()  {
            let to_parse = line.trim();
            if to_parse.len() == 0 {
                continue;
            }
            // println!("Will parse {}.", to_parse);
            let change = to_parse.parse::<i64>().unwrap();
            current_freq += change;
            if encountered.contains(&current_freq) {
                return current_freq
            }
            encountered.insert(current_freq);
        }
    }
    //   return current_freq
    return i64::MAX
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2018::d1::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_d1_full() {
        {
            assert_eq!(part1("+1 \n -2 \n +3 \n +1 \n "), 3);

            assert_eq!(part1("+1\n +1\n -2"),   0);
            assert_eq!(part1("-1\n -2\n -3"), -6);

            let pbuf = input::get_input("2018_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            assert_eq!(part1(content.as_str()), 538);
        }
        {
            assert_eq!(part2("+1 \n -2 \n +3 \n +1 \n "), 2);
            let pbuf = input::get_input("2018_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            assert_eq!(part2(content.as_str()), 77271);
        }
    }
}

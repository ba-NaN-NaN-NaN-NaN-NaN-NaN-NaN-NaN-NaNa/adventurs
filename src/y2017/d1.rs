#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let input = input.trim();

    let mut digits = Vec::<i64>::new();
    for char in input.to_string().chars() {
        let as_str = char.to_string();
        digits.push(as_str.parse::<i64>().unwrap());
    }

    let mut toreturn = 0;
    let length = digits.len();
    
    for n in 0..length {
        if digits[n] == digits[(n+1)%length] {
            // println!("Got match {}", digits[n]);
            toreturn += digits[n];
        }
    }
    return toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let input = input.trim();

    let mut digits = Vec::<i64>::new();
    for char in input.to_string().chars() {
        let as_str = char.to_string();
        digits.push(as_str.parse::<i64>().unwrap());
    }

    let mut toreturn = 0;
    let length = digits.len();
    
    for n in 0..length {
        if digits[n] == digits[(n+length/2)%length] {
            // println!("Got match {}", digits[n]);
            toreturn += digits[n];
        }
    }
    return toreturn
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2017::d1::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_d1_full() {
        {
            // 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
            assert_eq!(part1("1122"), 3);
            // 1111 produces 4 because each digit (all 1) matches the next.
            assert_eq!(part1("1111"), 4);
            // 1234 produces 0 because no digit matches the next.
            assert_eq!(part1("1234"), 0);
            // 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
            assert_eq!(part1("91212129"), 9);

            let pbuf = input::get_input("2017_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            assert_eq!(part1(content.trim()), 1136);
        }

        {
            assert_eq!(part2("1212"), 6);
            assert_eq!(part2("1221"), 0);
            assert_eq!(part2("123425"), 4);
            assert_eq!(part2("123123"), 12);
            assert_eq!(part2("12131415"), 4);
            let pbuf = input::get_input("2017_d1.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            assert_eq!(part2(content.trim()), 1092);
        }
    }
}

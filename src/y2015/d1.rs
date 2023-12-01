#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut toreturn = 0;
    for ch in input.trim().chars() {
        match ch {
            ')' => { toreturn -= 1},
            '(' => { toreturn += 1},
            _ => { panic!() },
        }
    }

    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut position = 0;
    for (n, ch) in input.trim().chars().enumerate() {
        match ch {
            ')' => { position -= 1},
            '(' => { position += 1},
            _ => { panic!() },
        }
        if position == -1  {
            let toreturn:i64 = n.try_into().unwrap();
            return toreturn + 1
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2015::d1::tests::test_p1p2 --exact --nocapture
        let pbuf = input::get_input("2015_d1.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();

        {
            // Part 1
            assert_eq!(0, part1("(())"));
            assert_eq!(0, part1("()()"));

            assert_eq!(3, part1("((("));
            assert_eq!(3, part1("(()(()("));
            assert_eq!(3, part1("))((((("));

            assert_eq!(-1, part1("())"));
            assert_eq!(-1, part1("))("));

            assert_eq!(-3, part1(")))"));
            assert_eq!(-3, part1(")())())"));

            let actual = part1(&content);
            assert_eq!(actual, 74);
        }

        {
            // Part 2
            assert_eq!(1, part2(")"));
            assert_eq!(5, part2("()())"));
            assert_eq!(1795, part2(&content));

        }
    }
}

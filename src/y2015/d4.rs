#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut digest = md5::Context::new();
    digest.consume(input.as_bytes());
    
    // for n in 1..i64::MAX {
    for n in 1..6000000 {
        let mut updater = digest.clone();
        updater.consume(format!("{}", n));
        let dgs = updater.compute();
        if dgs.0[0] == 0x00 && dgs.0[1] == 0x00 && (dgs.0[2] & 0xf0) == 0x00 {
            println!("Digest for {}{} gave {:?}", input, n, dgs);
            return n
        }
    }

    return -1
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut digest = md5::Context::new();
    digest.consume(input.as_bytes());
    
    // for n in 1..i64::MAX {
    for n in 1..600000000 {
        let mut updater = digest.clone();
        updater.consume(format!("{}", n));
        let dgs = updater.compute();
        if dgs.0[0] == 0x00 && dgs.0[1] == 0x00 && dgs.0[2] == 0x00 {
            println!("Digest for {}{} gave {:?}", input, n, dgs);
            return n
        }
    }

    return -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d4() {
        // cargo test --package adventurs --bin adventurs -- y2015::d4::tests --nocapture
        {
            // Part 1
            assert_eq!(609043, part1("abcdef"));
            assert_eq!(1048970, part1("pqrstuv"));
            assert_eq!(346386, part1("iwrupvqb"));
        }

        {
            assert_eq!(9958218, part2("iwrupvqb"));
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut toreturn = 0;
    let lines = input.split("\n");
    for line in lines {
        let l = line.trim();
        if l.len() == 0 {
            continue;
        }

        let parts: Vec<&str> = l.split("x").collect();
        let mut i0 = parts[0].parse::<i64>().unwrap();
        let mut i1 = parts[1].parse::<i64>().unwrap();
        let mut i2 = parts[2].parse::<i64>().unwrap();
        if i1 < i0  {
            (i0, i1) = (i1, i0);
        }
        if i2 < i1  {
            (i2, i1) = (i1, i2);
        }
        if i1 < i0  {
            (i0, i1) = (i1, i0);
        }        

        let partial = 3*i0*i1 + 2*i1*i2 + 2*i2*i0;
        // println!("Sorted is {}, {}, {} -> {}", i0, i1, i2, partial);
        
        toreturn += partial
    }
    
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut toreturn = 0;
    let lines = input.split("\n");
    for line in lines {
        let l = line.trim();
        if l.len() == 0 {
            continue;
        }

        let parts: Vec<&str> = l.split("x").collect();
        let mut i0 = parts[0].parse::<i64>().unwrap();
        let mut i1 = parts[1].parse::<i64>().unwrap();
        let mut i2 = parts[2].parse::<i64>().unwrap();
        if i1 < i0  {
            (i0, i1) = (i1, i0);
        }
        if i2 < i1  {
            (i2, i1) = (i1, i2);
        }
        if i1 < i0  {
            (i0, i1) = (i1, i0);
        }        

        let partial = i0+i0+i1+i1+i0*i1*i2;
        // println!("Sorted is {}, {}, {} -> {}", i0, i1, i2, partial);
        
        toreturn += partial
    }
    
    toreturn
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d2::tests::test_p1p2 --exact --nocapture
        let pbuf = input::get_input("2015_d2.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();

        {
            // Part 1
            assert_eq!(58, part1("2x3x4"));
            assert_eq!(43, part1("1x1x10"));
            assert_eq!(58+43, part1("2x3x4\n1x1x10"));

            let actual = part1(&content);
            assert_eq!(actual, 1588178); // p1
        }
        {
            assert_eq!(34, part2("2x3x4"));
            assert_eq!(14, part2("1x1x10"));
            assert_eq!(34+14, part2("2x3x4\n1x1x10"));
            let actual = part2(&content);
            assert_eq!(actual, 3783758); 
        }
    }
}

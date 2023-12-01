#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut largest = 0;
    let mut current = 0;
    let lines :std::str::Split<'_, &str>= input.split("\n");
    for line in lines {
        // println!("Got line {}.", line);
        let trimmed = line.trim();
        if trimmed.len() > 0 {
            let my_int = trimmed.parse::<i64>().unwrap();
            current += my_int;
        } else {
            if current > largest {
                largest = current;
                // println!("New record {}", largest);
            }
            current = 0;
        }
    }
    if current > largest {
        largest = current;
    }
    // 11773697 is too large.
    // 725637 is too large.
    // println!("Final largest {}", largest);
    largest
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let lines :std::str::Split<'_, &str>= input.split("\n");
    let mut chunks = Vec::new();
    chunks.push(0);
    let mut current = 0;
    for line in lines {
        // println!("Got line {}.", line);
        let trimmed = line.trim();
        if trimmed.len() > 0 {
            let my_int = trimmed.parse::<i64>().unwrap();
            current += my_int;
        } else {
            chunks.push(current);
            current = 0;
        }
    }
    chunks.sort();
    println!("Final chunks {:#?}", chunks);
    chunks.pop().unwrap() + chunks.pop().unwrap() + chunks.pop().unwrap()
}

#[allow(dead_code)]
pub fn mass_2_fuel(input:i64) -> i64 {
    let needed = (input / 3)-2;
    if needed < 0 {
        return 0
    } else {
        return needed
    }
}

#[allow(dead_code)]
pub fn part1(weights: Vec<i64>) -> i64 {
    let mut toreturn = 0;
    for weight in weights {
        toreturn += mass_2_fuel(weight)
    }
    return toreturn
}

#[allow(dead_code)]
pub fn part2(weights: Vec<i64>) -> i64 {
    let mut toreturn = 0;
    for weight in weights {
        toreturn += iterate_tyrrany(mass_2_fuel(weight))
    }
    return toreturn
}


#[allow(dead_code)]
pub fn iterate_tyrrany(input:i64) -> i64 {
    // the tyranny of the rocket equation
    let mut fuel_of_fuels = Vec::<i64>::new();
    fuel_of_fuels.push(input);

    while fuel_of_fuels[fuel_of_fuels.len()-1] != 0 {
        fuel_of_fuels.push(mass_2_fuel(fuel_of_fuels[fuel_of_fuels.len()-1]))
    }

    let mut sum = 0;
    for part in fuel_of_fuels {
        sum += part;
    }
    return sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_mass_2_fuel() {
        assert_eq!(mass_2_fuel(12), 2); // p1 sample
        assert_eq!(mass_2_fuel(14), 2);
        assert_eq!(mass_2_fuel(1969), 654);
        assert_eq!(mass_2_fuel(100756), 33583);
    }


    #[test]
    fn test_part_1() {
        let pbuf = input::get_input("2019_d1.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let ints = input::rows_to_ints(&content);
        assert_eq!(part1(ints), 3291760); 
    }

    #[test]
    fn test_part_2() {       
        assert_eq!(100756 + 50346, iterate_tyrrany(100756));

        let pbuf = input::get_input("2019_d1.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();
        let ints = input::rows_to_ints(&content);
        let actual = part2(ints);
        assert!(actual > 3291760);
        assert_eq!(actual, 4934767); 
    }
}

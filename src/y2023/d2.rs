use std::collections::VecDeque;
use regex::Regex;

#[allow(dead_code)]
pub struct Game {
    game_nr: i64,
    min_red: i64,
    min_green: i64,
    min_blue: i64,
}

#[allow(dead_code)]
impl Game {

    fn from_line(input: &str) -> Game  {
        let tokens = tokenize(input);
        return Game::eat_token_stream(&mut VecDeque::from(tokens));
    }

    // The approach of parsing via string splitting turned out awful. I switched
    // to the token stream approach, but leaving this old variant in for history.

    /*
    fn from_line(input: &str) -> Game  {
        // Parse 1 input line into a game.
        let decolon: Vec<&str> = input.trim().split(":").collect();
        if decolon.len() != 2 {
            panic!("Could not parse line {}", input)
        }

        let game_parts: Vec<&str> = decolon.get(0).unwrap().split(" ").collect();
        // println!("game_parts is {:?}",game_parts);

        let game_nr_str =  game_parts.get(1).unwrap();
        // println!("game_nr_str is {:?}",game_nr_str);

        let game_nr:i64 = game_nr_str.parse::<i64>().unwrap();

        let mut toreturn = Game { game_nr: game_nr, min_red: 0, min_green: 0, min_blue: 0 };

        // set: The first set is 3 blue cubes and 4 red cubes; 
        let sets: Vec<&str> = decolon.get(1).unwrap().split(";").collect();
        for set in sets  {
            let shown: Vec<&str> = set.split(",").collect();
            for mut show in shown {
                show = show.trim();
                // println!("Got shown one set: {}", show);
                let parts: Vec<&str> = show.split(" ").collect();
                let candidate = parts.get(0).unwrap().parse::<i64>().unwrap();
                let colorname = parts.get(1).unwrap().to_owned();
                match colorname {
                    "red" => {
                        if candidate > toreturn.min_red { toreturn.min_red = candidate }
                    },
                    "green" => {
                        if candidate > toreturn.min_green { toreturn.min_green = candidate }
                    },
                    "blue" => {
                        if candidate > toreturn.min_blue { toreturn.min_blue = candidate }
                    },
                    _ => {panic!("fkjdgh")},
                }
            }
        }

        toreturn
    }
    */

    fn eat_token_stream(stream: &mut VecDeque<String>) -> Game {
        // Eats one game from the token stream.
        if stream[0] != "Game" {
            panic!("eat_token_stream() asked to eat stream not located at start of game, got {}", stream[0]);
        }

        _ = stream.pop_front();
        let number = stream.pop_front().unwrap();

        let mut toreturn = Game { game_nr: number.parse::<i64>().unwrap(), min_red: 0, min_green: 0, min_blue: 0 };

        while stream.len() > 0 && stream[0] != "Game" {
            let count_str = stream.pop_front().unwrap();
            let count = count_str.parse::<i64>().unwrap();
            let color = stream.pop_front().unwrap();
            // println!("Got {} of color {}.", count, color);

            match color.as_str() {
                "red" => {
                    toreturn.min_red = toreturn.min_red.max(count);
                },
                "green" => {
                    toreturn.min_green = toreturn.min_green.max(count);
                },
                "blue" => {
                    toreturn.min_blue = toreturn.min_blue.max(count);
                },
                _ => {panic!("fkjdgh")},
            }
        }

        return toreturn
    }

    fn power(&self) -> i64 {
        return self.min_red * self.min_green * self.min_blue 
    }

    fn is_possible_p1(&self) -> bool {
        // possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes
        return self.min_red <= 12 && self.min_green <= 13 && self.min_blue <= 14
    }


}

/*
#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let lines: Vec<&str>  = input.split("\n").collect();
    let mut games = Vec::new();
    for mut line in lines {
        line = line.trim();
        if line.len() == 0 { continue; }
        let game = Game::from_line_2(line);
        if game.is_possible_p1() {
            games.push(game)
        }
    }

    let mut toreturn = 0;
    for game in games {
        toreturn += game.game_nr
    }
    return toreturn
}
*/

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut tokens = tokenize(input);
    let mut games = Vec::new();
    while tokens.len() > 0 {
        let game = Game::eat_token_stream(&mut tokens);
        if game.is_possible_p1() {
            games.push(game)
        }
    }

    let mut toreturn = 0;
    for game in games {
        toreturn += game.game_nr
    }
    return toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut tokens = tokenize(input);
    let mut games = Vec::new();
    while tokens.len() > 0 {
        let game = Game::eat_token_stream(&mut tokens);
        games.push(game);
    }

    let mut toreturn = 0;

    for game in games {
        toreturn += game.power();
    }
    return toreturn
}

#[allow(dead_code)]
pub fn tokenize(input: &str) -> VecDeque<String> {
    // Break input into relevant tokens, ignoring formatting chars.
    let mut toreturn = Vec::new();
    let mut worklist = input;
    
    let re_token = Regex::new(r"^([a-zA-Z0-9]+)").unwrap();
    let re_noise = Regex::new(r"^([^a-zA-Z0-9]+)").unwrap();

    while worklist.len() > 0 {
        match re_token.find(worklist) {
            None => {
                let to_discard = re_noise.find(worklist).unwrap().as_str();
                worklist = &worklist[to_discard.len()..];
            }
            Some(tok) => {
                let eat = tok.as_str();
                worklist = &worklist[eat.len()..];
                toreturn.push(eat.to_string());
                // println!("Eating {}", eat);
            }
        }
    }
    return VecDeque::from(toreturn)
    
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d2::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_tokenizer() {
        let input = "Game 2:            1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        
        Game 3: 8 green, 6 blue, 20 red; 5 blue,   4 red, 13           green; 5 green, 1 red
        ";
        let tokens = tokenize(input);
        assert!(tokens.len() > 0);
        assert!(tokens.get(0).unwrap() == "Game");
        assert!(tokens.get(5).unwrap() == "green");
        assert!(tokens.get(6).unwrap() == "3");

    }

    #[test]
    fn test_minimum_rgbs() {
        let g3 = Game::from_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(3, g3.game_nr);
        assert_eq!(20, g3.min_red);
        assert_eq!(13, g3.min_green);
        assert_eq!(6, g3.min_blue);
    }

    #[test]
    fn test_power() {
        let g3 = Game::from_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(1560, g3.power());
    }

    
    
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d2::tests::test_p1p2 --exact --nocapture

        test_tokenizer();
        test_minimum_rgbs();
        test_power();
        {
            let pbuf = input::get_input("2023_d2_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 8); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 2551); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d2_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 2286); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d2.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 62811); // p2 skarp
        }
    }
}

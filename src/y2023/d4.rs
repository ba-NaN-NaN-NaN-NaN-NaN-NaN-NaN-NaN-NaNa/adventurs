use std::collections::{VecDeque, HashMap};
use regex::Regex;

#[allow(dead_code)]
pub struct Card {
    // One line in 2023 d4
    card_nr: i64,
    winning_nrs: Vec<i64>,
    have_nrs: Vec<i64>
}

#[allow(dead_code)]
pub struct Lottery {
    // One line in 2023 d4
    cards: HashMap<i64, Card>,
    to_handin: Deck,
    handin_log: Deck,
}


#[allow(dead_code)]
type Deck = HashMap<i64, i64>;



#[allow(dead_code)]
impl Card {
    fn from_line(input: &str) -> Card  {
        let re_token = Regex::new(r"^([a-zA-Z0-9 ]+):([a-zA-Z0-9 ]+)\|([a-zA-Z0-9 ]+)$").unwrap();
        let res = re_token.captures(input).unwrap();

        let part0 = tokenize(&res[1].trim());
        let part1 = tokenize(&res[2].trim());
        let part2 = tokenize(&res[3].trim());

        return Card { 
            card_nr: part0[1].parse::<i64>().unwrap(),
            winning_nrs: part1.iter().map(|line| {
                // println!("Will now parse as i64: '{}'", line);
                line.trim().parse::<i64>().unwrap()
            }).collect(),
            have_nrs: part2.iter().map(|line| line.trim().parse::<i64>().unwrap()).collect(),
        }
    }

    fn matching_number_count(&self) -> i64 {
        let mut count:i64 = 0;
        for have_nr in self.have_nrs.iter() {
            if self.winning_nrs.contains(&have_nr) {
                count = count + 1
            }
        }
        count
    }

    fn points(&self) -> i64 {
        let count = self.matching_number_count();
        if count == 0 {
            return 0
        }
        let toreturn: i64 = 2_i64.pow((count-1).try_into().unwrap());
        toreturn
    }

    // fn hand_in_one(&self) -> Vec<i64> {
    fn hand_in_one(&self) -> Deck {
        // Hand in one card of this type.
        // If card 5 is worth 2 points, it will return [6,7];
        
        let mut toreturn : Deck = Deck::new();
        let count = self.matching_number_count();
        
        for n in 0..count {
            toreturn.insert(self.card_nr + 1 + n, 1);
        }    

        // println!("Card {} explodes into {:?}", self.card_nr, toreturn);
        toreturn
    }
}

impl Lottery {

    #[allow(dead_code)]
    pub fn from_content(input: &str) -> Lottery {
        let mut cards:HashMap<i64, Card> = HashMap::new();
        let mut initial_deck = Deck::new();

        for line in input.split("\n") {
            if line.trim().len() == 0 {
                 continue;
            }
 
            let card = Card::from_line(line.trim());
            
            initial_deck.insert(card.card_nr, 1);
            cards.insert(card.card_nr, card);
        }

        Lottery { 
            cards: cards, 
            to_handin: initial_deck,
            handin_log: Deck::new(),
        }
    }

    pub fn iterate_handin(&mut self) {
        let card_nrs = self.cards.iter().map( | (card_id, card) | card_id);
        // println!("Iterating handin. Will hand in {:?}", self.to_handin.clone());
        // println!("Iterating handin. Previously handed in {:?}", self.handin_log.clone());

        let old_handin = self.to_handin.clone();
        let mut new_handin = Deck::new();

        for (handin_card_nr, handin_count) in old_handin.iter() {
            let handin_card = self.cards.get(handin_card_nr).clone().unwrap();

            let prizes_one = handin_card.hand_in_one();
            for (prize_nr, prize_count) in prizes_one.iter() {
                match new_handin.get(&prize_nr) {
                    Some(already_present) => {
                        new_handin.insert(*prize_nr, already_present + prize_count * handin_count);
                    },
                    None => {
                        new_handin.insert(*prize_nr, prize_count * handin_count);
                    },
                }
            }

            match self.handin_log.get(&handin_card_nr) {
                Some(already_present) => {
                    self.handin_log.insert(*handin_card_nr, already_present + handin_count);
                },
                None => {
                    self.handin_log.insert(*handin_card_nr, *handin_count);
                },
            }

        }

        self.to_handin.clear();
        self.to_handin.extend(new_handin);
        /*

        for card_nr in card_nrs {
            let count:i64 = match self.to_handin.get(card_nr) {
                Some(n) => *n,
                None => 0_i64,
            };

            if count == 0 {
                continue;
            }

            println!("Will hand in {} of card nr {}", count, card_nr);
            match self.handin_log.get(&card_nr) {
                Some(already_present) => {
                    println!("Adding to handin_log: Card {} = {} + {}", card_nr, already_present, count);
                    self.handin_log.insert(*card_nr, already_present + count)
                },
                None => {
                    println!("Adding to handin_log: Card {} = {}", card_nr, count);
                    self.handin_log.insert(*card_nr,  count)
                },
            };

            let card = self.cards.get(&card_nr).clone().unwrap();
            let handin_result = card.hand_in_one();
            println!("Handing in 1 of {} gave us {:?}", card_nr, handin_result);

            for (prize_card, multiple) in handin_result {
                match self.to_handin.get(&prize_card) {
                    Some(already_present) => {
                        self.to_handin.insert(prize_card, already_present + count * multiple)
                    },
                    None => {
                        self.to_handin.insert(prize_card,  count * multiple)
                    },
                };
            }
        }
         */
    }

#[allow(dead_code)]
    pub fn part1(&mut self, input: &str) -> i64 {
        let mut toreturn = 0;
        for line in input.split("\n") {
            if line.trim().len() == 0 {
                continue;
            }

            let card = Card::from_line(line.trim());
            toreturn += card.points();
        }
        return toreturn
    }

    /*
    #[allow(dead_code)]
    pub fn part2(&mut self, input: &str) -> i64 {
        let mut cards:HashMap<i64, Card> = HashMap::new();

        let mut handed_in: HashMap<i64, i64> = HashMap::new();
        let mut unexploded: Vec<i64> = Vec::new();

        for line in input.split("\n") {
            if line.trim().len() == 0 {
                continue;
            }

            let card = Card::from_line(line.trim());
            unexploded.push(card.card_nr);
            handed_in.insert(card.card_nr, 0);
            cards.insert(card.card_nr, card);
        }

        while unexploded.len() > 0 {
            let nr_to_explode = unexploded.pop().unwrap();
            // println!("will now explode nr {}", nr_to_explode);
            let card = cards.get(&nr_to_explode).unwrap();
            *handed_in.get_mut(&card.card_nr).unwrap() += 1;

            let mut to_append = card.hand_in_one().clone();
            unexploded.append(&mut to_append);
        }

        println!("Handed in after part2 is {:#?}", handed_in);

        let mut toreturn = 0;
        for (_, value) in handed_in.into_iter() {
            toreturn += value
        }


        toreturn
    }
     */

    #[allow(dead_code)]
    pub fn part2(&mut self) -> i64 {
        
        // let mut 
        while self.to_handin.len() > 0 {
        // for _ in 0..4 {
           // self.to_handin = self.iterate_handin(self.to_handin.clone());
           self.iterate_handin();
        }
        return self.count_handed_in();
    }

    #[allow(dead_code)]
    pub fn count_handed_in(&self) -> i64 {
        let mut toreturn = 0;
        for (_, count) in self.handin_log.iter() {
            toreturn += count;
        }
        return toreturn
    }
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
    // $ cargo test --package adventurs --bin adventurs -- y2023::d4::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_from_line() {
        let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let card4 = Card::from_line(input);

        assert_eq!(card4.card_nr ,4);

        assert_eq!(card4.winning_nrs.len(), 5);
        assert_eq!(card4.have_nrs.len() , 8);

        let card1 = Card::from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card1.card_nr ,1);

        assert_eq!(card4.points(),1);
        assert_eq!(card1.points(),8);

    }
    
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d4::tests --nocapture


        {
            let pbuf = input::get_input("2023_d4_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = Lottery::from_content(&content).part1(&content);
            assert_eq!(actual, 13); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d4.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = Lottery::from_content(&content).part1(&content);
            assert_eq!(actual, 18619); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d4_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = Lottery::from_content(&content).part2();
            assert_eq!(actual, 30); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d4.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = Lottery::from_content(&content).part2();
            assert_eq!(actual, 8063216); // p2 skarp
        }
    }
}

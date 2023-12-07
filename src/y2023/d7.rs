use std::collections::{VecDeque, HashSet, HashMap};
use std::cmp::Ordering;
use regex::Regex;


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


#[allow(dead_code)]
pub fn part1(content: &str) -> i64 {
    let unranked_cards = into_hands(&content, false);
    let ranked_cards = sort_by_rank(unranked_cards.clone());
    let mut toreturn = 0;

    for n in 0..ranked_cards.len() {
        let card = ranked_cards.get(n).clone().unwrap();
        let mut card_nr: i64 = n.try_into().unwrap();
        card_nr+=1;
        toreturn += card_nr*card.bid;
    }
    return toreturn
}

#[allow(dead_code)]
fn part2(content: &str) -> i64 {
    let unranked_cards = into_hands(&content, true);
    let ranked_cards = sort_by_rank(unranked_cards.clone());
    let mut toreturn = 0;

    for n in 0..ranked_cards.len() {
        let card = ranked_cards.get(n).clone().unwrap();
        let mut card_nr: i64 = n.try_into().unwrap();
        card_nr+=1;
        toreturn += card_nr*card.bid;
    }
    return toreturn
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Hand {
    cards: String,
    bid: i64,

    strength: i64,
    strength_with_jokers: i64,
    hand_type: HandType,
    hand_type_with_jokers: HandType,

    consider_jokers: bool,
}


#[allow(dead_code)]
pub fn strength_no_jokers(cards: &str) -> i64 {
    let hand_type = calc_type(cards);
    
    let mut toreturn = hand_type.hand_score();

    if cards.len() != 5 {
        panic!("lfjd")
    }

    for card in cards.chars() {
        match card {
            'A' => {toreturn = toreturn * 20 + 14},  
            'K'=> {toreturn = toreturn * 20 + 13},
            'Q'=> {toreturn = toreturn * 20 + 12}, 
            'J'=> {toreturn = toreturn * 20 + 11}, 
            'T'=> {toreturn = toreturn * 20 + 10}, 
            '9'=> {toreturn = toreturn * 20 + 9},
            '8'=> {toreturn = toreturn * 20 + 8},
            '7'=> {toreturn = toreturn * 20 + 7},
            '6'=> {toreturn = toreturn * 20 + 6},
            '5'=> {toreturn = toreturn * 20 + 5},
            '4'=> {toreturn = toreturn * 20 + 4},
            '3'=> {toreturn = toreturn * 20 + 3},
            '2'=> {toreturn = toreturn * 20 + 2},
            _ => { 
                panic!("gfdjuh")
            }
        }
    }
    toreturn
}



#[allow(dead_code)]
pub fn strength_with_jokers(cards: &str, hand_type: HandType) -> i64 {
    let mut toreturn = hand_type.hand_score();

    if cards.len() != 5 {
        panic!("lfjd")
    }

    for card in cards.chars() {
        match card {
            'A' => {toreturn = toreturn * 20 + 14},  
            'K'=> {toreturn = toreturn * 20 + 13},
            'Q'=> {toreturn = toreturn * 20 + 12}, 
            'T'=> {toreturn = toreturn * 20 + 10}, 
            '9'=> {toreturn = toreturn * 20 + 9},
            '8'=> {toreturn = toreturn * 20 + 8},
            '7'=> {toreturn = toreturn * 20 + 7},
            '6'=> {toreturn = toreturn * 20 + 6},
            '5'=> {toreturn = toreturn * 20 + 5},
            '4'=> {toreturn = toreturn * 20 + 4},
            '3'=> {toreturn = toreturn * 20 + 3},
            '2'=> {toreturn = toreturn * 20 + 2},
            'J'=> {toreturn = toreturn * 20 + 1}, 
            _ => { 
                panic!("gfdjuh")
            }
        }
    }
    toreturn
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.consider_jokers {
            self.strength_with_jokers.cmp(&other.strength_with_jokers)    
        } else {
        self.strength.cmp(&other.strength)
    }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.consider_jokers {
            Some(self.strength_with_jokers.cmp(&other.strength_with_jokers))
        } else {
        Some(self.strength.cmp(&other.strength))
    }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.consider_jokers {
            self.strength_with_jokers == other.strength_with_jokers
        } else {
        self.strength == other.strength
        }
    }
}

impl Eq for Hand { }


#[allow(dead_code)]
pub fn into_hands(content: &str, consider_jokers: bool) -> Vec<Hand> {
    let mut toreturn = Vec::new();
    let lines:Vec<&str> = content.split("\n").collect();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }

        let parts:Vec<&str> = trimmed.split(" ").collect();
        if parts.len() != 2 {
            println!("Bad into_hands value {}", trimmed);
        }
        let cards = parts[0];
        let bid = parts[1].parse::<i64>().unwrap();

        let replaceable_hand = best_hand_types_possible(cards);

        toreturn.push(Hand { 
            cards: cards.to_string(),
            bid: bid,
            strength: strength_no_jokers(cards),
            hand_type: calc_type(cards),
            hand_type_with_jokers: replaceable_hand.clone(),
            strength_with_jokers: strength_with_jokers(cards, replaceable_hand),
            consider_jokers:consider_jokers,
        });
    }

    toreturn
}

/*

    Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456
    */

#[derive(PartialEq, Debug, Clone)]
pub enum HandType {
    FiveOfKind, // 
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair, // 
    HighCard, // 
}

impl HandType {    
    #[allow(dead_code)]
    pub fn hand_score(&self) -> i64 {
        match &self {
            HandType::FiveOfKind => { 7 },
            HandType::FourOfKind => { 6 },
            HandType::FullHouse => { 5 },
            HandType::ThreeOfKind => { 4 },
            HandType::TwoPair => { 3 },
            HandType::OnePair => { 2 },
            HandType::HighCard => { 1 },
        }
    }
}


#[allow(dead_code)]
pub fn calc_type(cards: &str) -> HandType {
    let mut unique_labels:HashSet<char> = HashSet::new();
    let mut sorted_labels: Vec<char> = cards.trim().chars().collect();
    sorted_labels.sort();

    for label in cards.trim().chars() {
        unique_labels.insert(label);
    }

    let mut card_counts: HashMap<char, i64> = HashMap::new();
    for label in unique_labels {
        let mut count = 0;
        for char in cards.trim().chars() {
            if char == label {
                count = count+1
            }
        }
        card_counts.insert(label, count);
    }

    let mut counts: Vec<i64> = Vec::new();
    for (_, count) in &card_counts {
        counts.push(*count);
    }

    counts.sort();
    let counts_sliced = counts.as_slice();
    // println!("For cards {} we have counts {:?}, or in sorted vec: {:?}", cards, &card_counts,  counts_sliced);
    if counts_sliced == [5] {    
        return HandType::FiveOfKind
    }

    if counts_sliced == [1, 4] {    
        return HandType::FourOfKind
    }

    if counts_sliced == [2, 3] {    
        return HandType::FullHouse
    }

    if counts_sliced == [1, 1, 3] {    
        return HandType::ThreeOfKind
    }

    if counts_sliced == [1, 2, 2] {    
        return HandType::TwoPair
    }
    if counts_sliced == [1, 1, 1, 2] {    
        return HandType::OnePair
    }
    if counts_sliced == [1, 1, 1, 1, 1] {    
        return HandType::HighCard
    }


    println!("Unhandled type {}", cards);
    panic!("Unhandled case")
}

#[allow(dead_code)]
pub fn best_hand_types_possible(cards: &str) -> HandType {
    // Return best hand types possible

    let mut toreturn = calc_type(cards);
    let mut toreturn_score = toreturn.hand_score();


    let mut worklist:HashSet<String> = HashSet::new();
    worklist.insert(cards.to_string());
    while worklist.len() > 0 {
        let to_process = worklist.clone();
        worklist.clear();
        for to_explode in to_process {
            if !to_explode.contains("J") {
                let candidate = calc_type(to_explode.as_str());
                if candidate == HandType::FiveOfKind {
                    // We can stop here, no need to search for better.
                    return HandType::FiveOfKind
                }

                let candidate_score = candidate.hand_score();
                if candidate_score > toreturn_score {
                    toreturn = candidate;
                    toreturn_score = candidate_score;
                }

            } else {
                for replace in ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"] {
                    let replacement = to_explode.replacen("J", &replace, 1);
                    worklist.insert(replacement);
                }
            }
        }
    }

    toreturn
}

#[allow(dead_code)]
pub fn sort_by_rank(mut cards:  Vec<Hand> ) -> Vec<Hand>  {
    cards.sort();
    cards
}


#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d7::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_sort_rank() {
        
        let pbuf = input::get_input("2023_d7_sample.txt").unwrap();
        let content = input::readstring(&pbuf).unwrap();

        let unranked_cards = into_hands(&content, false);
        let ranked_cards = sort_by_rank(unranked_cards.clone());
        assert_eq!(5, unranked_cards.len());  
        assert_eq!(5, ranked_cards.len());  

        assert_eq!("32T3K".to_string(), ranked_cards.get(0).unwrap().cards);  
        assert_eq!("KTJJT".to_string(), ranked_cards.get(1).unwrap().cards);  

        

    }

    #[test]
    fn test_hand_kind() {
 
        assert_eq!(HandType::FiveOfKind, calc_type("AAAAA"));
        assert_eq!(HandType::FourOfKind, calc_type("AA8AA"));
        assert_eq!(HandType::FullHouse, calc_type("23332"));
        assert_eq!(HandType::ThreeOfKind, calc_type("TTT98"));
        assert_eq!(HandType::TwoPair, calc_type("23432"));
        assert_eq!(HandType::OnePair, calc_type("A23A4"));
        assert_eq!(HandType::HighCard, calc_type("23456"));
        
        

    }
    /*
    Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456
 */

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d7::tests --nocapture


        {
            let pbuf = input::get_input("2023_d7_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 6440); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d7.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 247815719); // p1 skarp
        }


        {
            let pbuf = input::get_input("2023_d7_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 5905); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d7.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 248747492); // p2 skarp
        }
    }
}

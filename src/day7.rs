use std::collections::{HashSet, HashMap};

use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/7
/**/
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day7.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten();

    let mut hands = vec![];

    for line in lines {
        let (cards, bid) = line.split_once(' ').unwrap();
        println!("{cards}, {bid}");
        let hand = Hand::new(cards, bid.parse()?);
        hands.push(hand);
    }
    hands.sort_by(Hand::compare);
    
    let total = hands.into_iter().enumerate()
        .map(|(rank, hand)| (rank+1)*hand.bid )
        .sum();

    Ok(total)
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    str: String,
    typ: HandType,
    bid: usize,
}
impl Hand {
    fn new(str: &str, bid: usize) -> Self {
        Self {str: str.to_string(), typ: Self::determine_hand(str), bid}
    }
    fn determine_hand(str: &str) -> HandType {
        use  HandType::*;
        let mut map = HashMap::new();
        for char in str.chars() {
            *map.entry(char).or_insert(0) += 1;
        }
        let num_unique_cards = map.iter().collect::<HashSet<_>>().len();
        let most_duplicates = *map.values().max().unwrap();
    
        match (num_unique_cards, most_duplicates) {
            (1,_) => FiveOfAKind,   //AAAAA
            (2,4) => FourOfAKind,   //AAAAB
            (2,3) => FullHouse,     //AAABB
            (2,_) => unreachable!(),
            (3,3) => ThreeOfAKind,  //AAABC
            (3,2) => TwoPair,       //AABBC
            (3,_) => unreachable!(),
            (4,_) => OnePair,       //AABCD
            (5,_) => HighCard,      //ABCDE
            _ => unreachable!("Whoops: {num_unique_cards}, {most_duplicates}, {str}")
        }
    }
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        if self.typ != other.typ {
            (self.typ).cmp(&other.typ)
        }
        else {
            const STRING_ORDER: &str = "AKQJT98765432";
            for (char1, char2) in (self.str.chars()).zip(other.str.chars()) {
                let pos1 = STRING_ORDER.find(char1).unwrap();
                let pos2 = STRING_ORDER.find(char2).unwrap();
                if pos1 != pos2 {
                    return pos1.cmp(&pos2).reverse()
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day7.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten();

    let mut hands = vec![];

    for line in lines {
        let (cards, bid) = line.split_once(' ').unwrap();
        println!("{cards}, {bid}");
        let hand = Hand::new_joker_hand(cards, bid.parse()?);
        hands.push(hand);
    }
    hands.sort_by(Hand::compare_joker);
    
    let total = hands.into_iter().enumerate()
        .map(|(rank, hand)| (rank+1)*hand.bid )
        .sum();

    Ok(total)
}

impl Hand {
    fn new_joker_hand(str: &str, bid: usize) -> Self {
        Self {str: str.to_string(), bid, typ: Self::determine_joker_hand(str)}
    }
    fn determine_joker_hand(str: &str) -> HandType {
        use HandType::*;
        let mut map = HashMap::new();
        let mut num_jokers = 0;
        for char in str.chars() {
            if char == 'J' {
                num_jokers += 1; 
                continue;
            }
            *map.entry(char).or_insert(0) += 1;
        }
    
        // Non-joker cards
        let num_unique_nonjoker_cards = map.iter().collect::<HashSet<_>>().len();
        let most_duplicates_nonjoker = *map.values().max().unwrap_or(&0);
    
        match (num_unique_nonjoker_cards, most_duplicates_nonjoker, num_jokers) {
        // 0 or 1 type of non-joker card
            (0..=1,_,_) => FiveOfAKind, //AAAAA, AAAAJ, AAAJJ, AAJJJ, AJJJJ, JJJJJ

        // 2 types of non-joker card
        //   AAAAB     AAABJ     AABJJ     ABJJJ
            (2,4,0) | (2,3,1) | (2,2,2) | (2,1,3) => FourOfAKind,
        //   AAABB     AABBJ
            (2,3,0) | (2,2,1) => FullHouse,

        // 3 types of non-joker card
        //   AAABC     AABCJ     ABCJJ
            (3,3,0) | (3,2,1) | (3,1,2) => ThreeOfAKind,
        //   AABBC
            (3,2,0) => TwoPair,         

        // 4 types of non-joker card
        //   AABCD     ABCDJ
            (4,2,0) | (4,1,1) => OnePair,

        // 5 types of non-joker card
        //   ABCDE
            (5,_,_) => HighCard,        
            _ => unreachable!("Whoops: {num_unique_nonjoker_cards}, {most_duplicates_nonjoker}, {str}")
        }
    }
    fn compare_joker(&self, other: &Self) -> std::cmp::Ordering {
        if self.typ != other.typ {
            (self.typ).cmp(&other.typ)
        }
        else {
            const JOKER_STRING_ORDER: &str = "AKQT98765432J";
            for (char1, char2) in (self.str.chars()).zip(other.str.chars()) {
                let pos1 = JOKER_STRING_ORDER.find(char1).unwrap();
                let pos2 = JOKER_STRING_ORDER.find(char2).unwrap();
                if pos1 != pos2 {
                    return pos1.cmp(&pos2).reverse()
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}
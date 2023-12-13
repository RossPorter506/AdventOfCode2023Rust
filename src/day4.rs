use crate::prelude::*;
use std::collections::HashSet as Set;
/**/
// https://adventofcode.com/2023/day/4
/**/

/// Given a list of scratch cards containing winning numbers and numbers, calculate the score of each card (given by 2^matching if matching =/= 0, 0 otherwise).
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day4.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten();
    let mut total = 0;

    for line in lines {
        if line.is_empty() {continue;}
        let card = parse_line(&line)?;
        let score = card.calculate_score();
        println!("Card {} is worth {score}", card.id);
        total += score;
    }
    
    Ok(total)
}

fn parse_line(str: &str) -> Result<Card> {
    let (prefix, numbers) = str.split_once(": ")
        .ok_or(anyhow!("Could not split {str} on \": \""))?;
    let (_, id_str) = prefix.split_once(' ')
        .ok_or(anyhow!("Could not extract ID from {prefix}"))?;
    let id: usize = id_str.trim().parse()?;
    let (winning_nums, our_nums) = numbers.split_once(" | ")
        .ok_or(anyhow!("Could not split {numbers} on \": \""))?;

    let winning_num_set: Result<Set<usize>> = winning_nums.split_whitespace()
        .map(|x| x.parse::<usize>()
        .map_err(|_| anyhow!("Failed to parse winning number {x}")))
        .collect();
    let our_num_set: Result<Set<usize>> = our_nums.split_whitespace()
        .map(|x| x.parse::<usize>()
        .map_err(|_| anyhow!("Failed to parse winning number {x}")))
        .collect();

    Ok( Card::new(id, winning_num_set?, our_num_set?) )
}

#[derive(Clone)]
struct Card {
    id: usize,
    winning_nums: Set<usize>,
    nums: Set<usize>,
    matching_nums: Set<usize>
}
impl Card {
    fn new(id: usize, winning_nums: Set<usize>, nums: Set<usize>) -> Self {
        let matching_nums = winning_nums.intersection(&nums).cloned().collect();
        Card { id, winning_nums, nums, matching_nums }
    }
    fn calculate_score(&self) -> usize {
        match self.matching_nums.len() {
            0 => 0,
            x => 2_usize.pow(x as u32 - 1)
        }
    }
}

/// Given a list of scratch cards, where winning cards add extra cards based on how many numbers match. Calculate how many cards are won in total.
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day4.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten();
    // Construct a card list to be used later.
    let mut card_list: Vec<Card> = vec![];
    
    for line in lines {
        if line.is_empty() {continue;}
        let card = parse_line(&line)?;
        card_list.push(card);
    }

    // Iterate through the card list. For each card count how many we have of this card (num_of_card), and how many cards this card wins (num_won).
    // Add num_of_card cards to the next num_won cards.
    let mut cards: Vec<usize> = vec![1;card_list.len()];
    for i in 0..cards.len() {
        let num_of_card = cards[i];
        let num_won = card_list[i].matching_nums.len();
        for card in cards.iter_mut().skip(i+1).take(num_won) {
            *card += num_of_card;
        }
    }
    
    // Return the total number of cards
    Ok(cards.into_iter().sum())
}
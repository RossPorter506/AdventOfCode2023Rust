use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/2
/**/


/// Read in a list of games separated by newlines. Each game consists of a number of hands, where each hand optionally lists a number of red green and blue cubes.
/// 
/// Determine which games can be played with only MAX_HAND cubes and sum their IDs.
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut total = 0;

    for line in lines.flatten() {
        if line.is_empty() {continue;}
        let game = parse_line(&line)?;
        if game.is_possible_with(&NUM_CUBES) {
            total += game.id;
        }
    }

    Ok(total)
}

const NUM_CUBES: Hand = Hand {red: 12, green: 13, blue: 14};

#[derive(Default, Debug)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}
impl Hand {
    /// Whether this hand could be produced using the given number of cubes
    fn is_possible_with(&self, num_cubes: &Hand) -> bool {
        self.red <= num_cubes.red && 
        self.green <= num_cubes.green && 
        self.blue <= num_cubes.blue
    }
}
struct Game {
    id: usize,
    hands: Vec<Hand>,
}
impl Game {
    /// Whether this game could be played with the given number of cubes
    fn is_possible_with(&self, cubes: &Hand) -> bool {
        // Game is possible if all hands are possible
        self.hands.iter().all(|hand| hand.is_possible_with(cubes))
    }
}

/// Parse a line like 'Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green' into a struct.
fn parse_line(str: &str) -> Result<Game> {
    let (game_prefix, game) = str.split_once(": ")
        .ok_or(anyhow!("Failed to split string {str} on ':'"))?;
    let (_, id_str) = game_prefix.split_once(' ')
        .ok_or(anyhow!("Failed to split string {game_prefix} on ' '"))?;
    let id: usize = id_str.parse()?;
    let mut hands: Vec<Hand> = vec![];

    let hands_as_strs = game.split("; ");
    // hands_as_strs = ["6 red, 4 green, 7 blue", "3 red, 1 green, 5 blue", ...]
    for hand_str in hands_as_strs {
        // hand_str = "6 red, 4 green, 7 blue"
        let colour_counts = hand_str.split(", ");
        // colour_counts = ["6 red", "4 green", "7 blue"]
        let mut hand_struct: Hand = Hand::default();
        for colour_count_as_str in colour_counts {
            // colour_count_as_str = "6 red"
            let (count_str, colour_str) = colour_count_as_str.split_once(' ')
                .ok_or(anyhow!("Failed to split string {colour_count_as_str} on ' '"))?;
            // (count_str, colour_str) = ("6", "red")
            let count = count_str.parse()?;
            match colour_str {
                "red" => hand_struct.red = count,
                "green" => hand_struct.green = count,
                "blue" => hand_struct.blue = count,
                _ => unreachable!(),
            }
        }
        hands.push(hand_struct);
    }

    Ok(Game {id, hands})
}

/// Read in a list of games separated by newlines. Each game consists of a number of hands, where each hand optionally lists a number of red green and blue cubes.
/// 
/// Calculate the minimum hand required to play a game. Multiply the number of cubes required to get the 'power', then return the sum of powers over all games.
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut total = 0;
    
    for line in lines.flatten() {
        if line.is_empty() {continue;}
        let game = parse_line(&line)?;
        total += game.minimum_hand().power();
    }

    Ok(total)
}

impl Game {
    /// Determine the minimum number of cubes required to make this game possible
    fn minimum_hand(&self) -> Hand {
        let mut min_hand = Hand::default();
        for hand in &self.hands {
            min_hand.red = min_hand.red.max(hand.red);
            min_hand.green = min_hand.green.max(hand.green);
            min_hand.blue = min_hand.blue.max(hand.blue);
        }
        min_hand
    }
}

impl Hand {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}
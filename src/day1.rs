use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/1
/**/

/// Read in a list of values separated by newlines. Concatenate the first and last digit from each row to make a number. Sum all of these numbers.
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut total = 0;

    for line in lines.flatten() {
        if line.is_empty() {continue;}
        let first_numeral = get_first_digit(&line)?;
        let last_numeral = get_last_digit(&line)?;
        let number: u8 = (first_numeral.to_string() + &last_numeral.to_string()).parse()?;
        println!("extracted {number} from {line}");
        total += number as usize;
    }

    Ok(total)
}

fn get_first_digit(str: &str) -> Result<char> {
    for char in str.chars() {
        if char.is_ascii_digit() {
            return Ok(char);
        }
    }
    bail!("No digit in string!")
}

fn get_last_digit(str: &str) -> Result<char> {
    let str = str.chars().rev().collect::<String>();
    get_first_digit(&str)
}

/// Read in a list of values separated by newlines. Concatenate the first and last numeral (i.e. a digit or a word representing a digit like 'seven') from each row to make a number. Sum all of these numbers.
// Almost identical, but we digitify the line prior to digit extraction.
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut total = 0;

    for line in lines.flatten() {
        if line.is_empty() {continue;}
        let line = only_digits_and_numerals(&line)?;
        let first_numeral = get_first_digit(&line)?;
        let last_numeral = get_last_digit(&line)?;
        let number: u8 = (first_numeral.to_string() + &last_numeral.to_string()).parse()?;
        total += number as usize;
    }

    Ok(total)
}

const NUMERAL_WORDS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

struct Numeral {
    start_index: usize,
    replace_with: char,
}

/// Remove any non-numeral and non-digit characters. Replace all instances of numerals with digits. 
/// 
/// Turns a string like 'five8abcdonezero123' into '5810123'
fn only_digits_and_numerals(str: &str) -> Result<String> {
    let mut numeral_instances : Vec<Numeral> = vec![];
    // Find all instances of numerals
    for (digit, numeral) in NUMERAL_WORDS.iter().enumerate() {
        let matches = str.match_indices(numeral);
        for (pos, _) in matches {
            let numeral = Numeral{
                start_index: pos, 
                replace_with: DIGIT_CHARS[digit],
            };
            numeral_instances.push(numeral);
        }
    }
    // Find all instances of digits
    for (index, char) in str.chars().enumerate() {
        if char.is_ascii_digit() {
            let numeral = Numeral{
                start_index: index, 
                replace_with: char,
            };
            numeral_instances.push(numeral);
        }
    }
    // Construct a new string based on the numerals and digits found
    numeral_instances.sort_by(|a,b| a.start_index.cmp(&b.start_index));
    let out = numeral_instances.into_iter().map(|i| i.replace_with).collect::<String>();
    Ok(out)
}
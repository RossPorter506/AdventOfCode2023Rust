use itertools::Itertools;
use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/12
/**/
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day12.txt")?;
    let lines = BufReader::new(file).lines().flatten().collect::<Vec<String>>();

    let total: usize = lines.iter().map(|l| {
        let (damaged, sections) = parse_line(l);
        let num_repaired = calculate_all_possible_repaired_strings(damaged, &sections);

        //let valid_strings: Vec<_> = possible_repairs.iter().filter(|s| is_valid_configuration(s, &sections, true)).collect();

        println!("'{damaged}' has {num_repaired} possible repaired versions");
        num_repaired
    }).sum();
    Ok(total)
}

fn parse_line(str: &str) -> (&str, Vec<usize>){
    let (part1, part2) = str.split_once(' ').unwrap();
    let part2 = part2.split(',')
        .map(|s| s.parse().unwrap());

    (part1, part2.collect())
}

fn calculate_all_possible_repaired_strings(str: &str, contiguous_lengths: &[usize]) -> usize {
    let num_unknown = str.chars().filter(|&c| c == '?').count();
    
    let mut input = vec![str.to_string()];
    for _ in 0..num_unknown {
        let mut intermediates = vec![];
        for string in input {
            for char in ["#", "."] {
                let str_temp = string.replacen('?', char, 1);
                // Because input grows exponentially, check if this string is even possible before it goes back into the pool.
                if is_valid_configuration(&str_temp, contiguous_lengths, false) {
                    intermediates.push(str_temp);
                }
            }
        }
        input = intermediates;
    }
    input.len()
}

/// Checks whether a particular string fulfills requirements.
/// 
/// Supports checking incomplete strings: If `should_be_complete` is false then this function returns true on seeing the first '?' (i.e. this string is not invalid *so far*). 
/// When `should_be_complete` is true then any '?'s will fail the string.
fn is_valid_configuration(str: &str, contiguous_lengths: &[usize], should_be_complete: bool) -> bool {
    let mut seen_index = 0;
    let mut contiguous_lengths = contiguous_lengths.iter().peekable();

    while let Some(&length) = contiguous_lengths.next() {
        // Skip parts of the string we've already seen.
        let substr = str.chars().skip(seen_index).collect::<String>();
        let mut substr_iter = substr.chars().peekable();

        // Skip any leading '.'s
        while let Some('.') = substr_iter.peek() {
            substr_iter.next();
            seen_index += 1;
        }

        // How many of the current contiguous block we've seen
        let mut current_count = 0;
        
        // Check there are at least `length` contiguous '#'s
        while current_count < length {
            match substr_iter.next() {
                Some('.') => return false,
                Some('#') => current_count += 1,
                Some('?') => return !should_be_complete,
                Some(_) => unreachable!(),
                None => return false, // ran out of string and length still not satisfied
            };
        }
        
        seen_index += length+1;
        // Check it isn't longer than length by peeking at next element.
        match substr_iter.peek() {
            Some('.') => continue,
            Some('#') => return false,
            Some('?') => return !should_be_complete,
            Some(_) => unreachable!(),
            None => if contiguous_lengths.peek().is_none() {return true}, // we are at the end of both the string and the contiguous sections
        }
    }
    // If there are any more '#'s after we have satisfied all the chunks it fails
    if str.chars().skip(seen_index).contains(&'#') {
        return false;
    }
    // likewise if we're in complete mode and see a '?'
    if str.chars().skip(seen_index).contains(&'?') && should_be_complete {
        return false;
    }
    // Otherwise they're just trailing '.'s, we don't mind
    true
}

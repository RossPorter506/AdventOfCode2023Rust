use crate::prelude::*;
use std::collections::VecDeque;
/**/
// https://adventofcode.com/2023/day/3
/**/

const LINE_WIDTH: usize = 140;

/// Given rows of chars, find all numbers adjacent to (non-'.') symbols. Return the sum of all of these.
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day3.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();
    let mut total = 0;
    // Make a queue of length 3. Push two lines.
    let mut rows: VecDeque<[char;LINE_WIDTH]> = VecDeque::new();
    let dummy_row = ['.';LINE_WIDTH];
    //Add a dummy row so we can check the top row the same as the middle rows.
    rows.push_back(dummy_row);
    rows.push_back(parse_line(&lines.next().ok_or(anyhow!("Could not get line from file"))?)?);

    for line in lines {
        if line.is_empty() {continue;}
        //Push a new line at the bottom
        rows.push_back(parse_line(&line)?);
        //Check middle line for entries.
        let part_numbers: Vec<usize> = check_for_part_numbers(&rows)?;
        //Delete the top line.
        rows.pop_front();
        total += part_numbers.into_iter().sum::<usize>();
    }
    // Add a dummy row so we can check the last row
    rows.push_back(dummy_row);
    let part_numbers: Vec<usize> = check_for_part_numbers(&rows)?;
    total += part_numbers.into_iter().sum::<usize>();
    
    Ok(total)
}

fn parse_line(str: &str) -> Result<[char;LINE_WIDTH]> {
    let vec = str.chars().collect::<Vec<char>>();
    let arr: [char;LINE_WIDTH] = vec.as_slice().try_into()?;
    Ok(arr)
}

fn check_for_part_numbers(rows: &VecDeque<[char;LINE_WIDTH]>) -> Result<Vec<usize>> {
    let mut out: Vec<usize> = vec![];
    let mut currently_in_num: bool;
    let mut is_part_num = false; // Have we found a symbol adjacent to this number?
    let mut current_num: Vec<char> = vec![]; // Number as we've seen it thus far

    for (pos, char) in rows[1].iter().enumerate() {
        currently_in_num = char.is_ascii_digit();

        if currently_in_num {
            current_num.push(*char);
            if !is_part_num {
                is_part_num = check_neighbours_for_symbol(rows, pos);
            }
        }
        else if !current_num.is_empty() {
            // If we were keeping track of a number prior to this add it to the list if it's an ID.
            if is_part_num {
                let num_as_usize = current_num.iter().collect::<String>().parse()?;
                out.push(num_as_usize);
            }
            current_num.clear();
            is_part_num = false;
        }
    }
    if !current_num.is_empty() && is_part_num { // deal with any number on the very end that didn't get pushed.
        let num_as_usize = current_num.iter().collect::<String>().parse()?;
        out.push(num_as_usize);
    }
    Ok(out)
}

fn check_neighbours_for_symbol(rows: &VecDeque<[char;LINE_WIDTH]>, pos: usize) -> bool {
    for x in -1..=1_isize {
        for y in -1..=1_isize {
            let check_x = pos.saturating_add_signed(x).min(LINE_WIDTH-1);
            let check_y = 1usize.saturating_add_signed(y).min(2);
            if is_symbol(rows[check_y][check_x]) {
                return true;
            }
        }
    }
    false
}

fn is_symbol(test: char) -> bool {
    (test != '.') && (!test.is_ascii_digit())
}

/// Given rows of chars, find all symbols adjacent to exactly two numbers. Multiply the two adjacent numbers, and return the sum of all of these.
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day3.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();
    let mut total = 0;
    // Make a queue of length 3. Push two lines.
    let mut rows: VecDeque<[char;LINE_WIDTH]> = VecDeque::new();
    let dummy_row = ['.';LINE_WIDTH];
    //Add a dummy row so we can check the top row the same as the middle rows.
    rows.push_back(dummy_row);
    rows.push_back(parse_line(&lines.next().ok_or(anyhow!("Could not get line from file"))?)?);

    for line in lines {
        if line.is_empty() {continue;}
        //Push a new line at the bottom
        rows.push_back(parse_line(&line)?);
        //Check middle line for entries.
        let gear_ratios: Vec<usize> = check_for_gears(&rows)?;
        //Delete the top line.
        rows.pop_front();
        total += gear_ratios.into_iter().sum::<usize>();
    }
    // Add a dummy row so we can check the last row
    rows.push_back(dummy_row);
    let gear_ratios: Vec<usize> = check_for_gears(&rows)?;
    total += gear_ratios.into_iter().sum::<usize>();

    Ok(total)
}

fn check_for_gears(rows: &VecDeque<[char;LINE_WIDTH]>) -> Result<Vec<usize>> {
    let mut out: Vec<usize> = vec![];
    for (pos, char) in rows[1].iter().enumerate() {
        if is_symbol(*char) {
            let adjacent_nums = check_neighbours_for_nums(rows, pos)?;
            if adjacent_nums.len() == 2 {
                let ratio = adjacent_nums.into_iter().reduce(|acc, x| acc * x).ok_or(anyhow!("Could not reduce"))?;
                out.push(ratio);
            }
        }
    }
    
    Ok(out)
}

fn check_neighbours_for_nums(rows: &VecDeque<[char;LINE_WIDTH]>, pos: usize) -> Result<Vec<usize>> {
    let mut out: Vec<usize> = vec![];

    // Check position directly left
    let right_of_pos = pos.saturating_add_signed(1).min(LINE_WIDTH-1);
    let right_char = rows[1][right_of_pos];
    if right_char.is_ascii_digit() {
        out.push(find_num(&rows[1], right_of_pos)?);
    }

    // Check position directly right
    let left_of_pos = pos.saturating_add_signed(-1);
    let left_char = rows[1][left_of_pos];
    if left_char.is_ascii_digit() {
        out.push(find_num(&rows[1], left_of_pos)?);
    }

    // Check the three spaces below
    out.extend(add_vertical_neighbours(&rows[2], pos)?);

    // Check the three spaces above
    out.extend(add_vertical_neighbours(&rows[0], pos)?);
    Ok(out)
}

/// Check whether any numbers are adjacent to `pos` within `row`
fn add_vertical_neighbours(row: &[char;LINE_WIDTH], pos: usize) -> Result<Vec<usize>>{
    let mut out: Vec<usize> = vec![];

    //Calculate left and right positions, careful of edges
    let right_of_pos = pos.saturating_add(1).min(LINE_WIDTH-1);
    let left_of_pos = pos.saturating_add_signed(-1);

    // Get neighbouring spaces as array
    let neighbours: [char;3] = row[left_of_pos..=right_of_pos].try_into()?;

    // Whether each neighbour is a digit or not
    let neighbour_is_digit: [bool;3] = neighbours.iter().map(|x| x.is_ascii_digit()).collect::<Vec<bool>>().try_into().map_err(|_| anyhow!("Could not collect bool arr"))?;
    
    match neighbour_is_digit {
        // one num, search from left
        [true, true, _] | [true, false, false] => {out.push(find_num(row, left_of_pos)?)}, 
        // one num, start already found - middle
        [false, true, _] => {out.push(find_num(row, pos)?)}, 
        // one num, start already found - right
        [false, false, true] => {out.push(find_num(row, right_of_pos)?)}, 
        // two nums, search left and right
        [true, false, true] => {
            out.push(find_num(row, left_of_pos)?);
            out.push(find_num(row, right_of_pos)?);
        }, 
        // no nums
        [false, false, false] => (), 
    };
    Ok(out)
}

/// Given a position that is somewhere within a number, return the whole number.
fn find_num(row: &[char;LINE_WIDTH], mut pos: usize) -> Result<usize> {
    let mut out: Vec<char> = vec![];
    //Find the start of the number, mindful of edges
    while pos > 0 && row[pos.saturating_sub(1)].is_ascii_digit() {
        pos = pos.saturating_sub(1);
    }

    // Record the number, mindful of edges
    for &char in row[pos..].iter() {
        if !char.is_ascii_digit() { break; }
        out.push(char);
    }
    // Parse vec to usize
    Ok(out.into_iter().collect::<String>().parse()?)
}
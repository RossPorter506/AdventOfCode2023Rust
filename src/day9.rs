use std::collections::VecDeque;

use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/9
/**/
pub fn calculate_part1() -> Result<isize>{
    let file = File::open("input/day9.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten();
    let mut total = 0;

    for line in lines {
        let mut diffs: Vec<Vec<isize>> = vec![];
        let seq = parse_line(&line)?;
        diffs.push(seq);
        
        // While top layer still has non-zero values, add a new layer calculated from the differences between the previous row's entries.
        while diffs.iter().next_back().unwrap().iter().any(|x| *x != 0) {
            let new_layer = diffs.iter().next_back().unwrap().as_slice().windows(2).map(|x| x[1]-x[0]).collect();
            diffs.push(new_layer);
        }

        let depth = diffs.len();

        // Add a zero to the row of zeroes on the last layer. Then add a new value to each row.
        diffs.iter_mut().next_back().unwrap().push(0);
        for i in (1..depth).rev() {
            let difference = *diffs[i].iter().next_back().unwrap();
            let prev_in_row = *diffs[i-1].iter().next_back().unwrap();

            diffs[i-1].push(prev_in_row + difference);
        }
        total += diffs[0].iter().next_back().unwrap();
    }
    
    Ok(total)
}

fn parse_line(str: &str) -> Result<Vec<isize>> {
    let out: Result<Vec<isize>, _> = str.split_whitespace().map(|s| s.parse()).collect();
    Ok(out?)
}
// Basically the same as part 1 but using VecDeque to append to the front.
pub fn calculate_part2() -> Result<isize>{
    let file = File::open("input/day9.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten();
    let mut total = 0;

    for line in lines {
        let mut diffs: Vec<VecDeque<isize>> = vec![];
        let seq = parse_line(&line)?;
        diffs.push(seq.into());
        
        
        // While top layer still has non-zero values, add a new layer calculated from the differences between the previous row's entries.
        while diffs.iter().next_back().unwrap().iter().any(|x| *x != 0) {
            let new_layer = diffs.iter_mut().next_back().unwrap().make_contiguous().windows(2).map(|x| x[1]-x[0]).collect();
            diffs.push(new_layer);
        }
        diffs.iter().for_each(|layer| println!("{layer:?}"));
        let depth = diffs.len();

        // Add a zero to the row of zeroes on the last layer. Then add a new value to each row.
        diffs.iter_mut().next_back().unwrap().push_back(0);
        for i in (1..depth).rev() {
            let difference = *diffs[i].iter().next().unwrap();
            let prev_in_row = *diffs[i-1].iter().next().unwrap();

            diffs[i-1].push_front(prev_in_row - difference);
        }
        diffs.iter().for_each(|layer| println!("{layer:?}"));
        total += diffs[0].iter().next().unwrap();
    }
    
    Ok(total)
}
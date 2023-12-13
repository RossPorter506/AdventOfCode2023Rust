use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/13
/**/

pub fn calculate_part1() -> Result<usize>{
    let input: Vec<Vec<char>> = include_str!("../input/day13.txt")
        .lines()
        .map(|s| 
            s.chars().collect())
        .collect();

    let landscapes = parse_landscapes(input);

    let mut total = 0;
    'outer: for landscape in landscapes {
        let width = landscape[0].len();
        let length = landscape.len();
        
        for mirror_row in 1..length {
            if is_mirror_row(&landscape, mirror_row) {
                total += 100*mirror_row;
                continue 'outer;
            }
        }

        let landscape = transpose(landscape);
        for mirror_col in 1..width {
            if is_mirror_row(&landscape, mirror_col) {
                total += mirror_col;
                continue 'outer;
            }
        }
    }

    Ok(total)
}

// Parse input lines into discrete landscape 
fn parse_landscapes(input: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>>{
    let mut landscapes = vec![];
    let mut landscape = vec![];
    for line in input {
        if line.is_empty() {
            landscapes.push(landscape);
            landscape = vec![];
        }
        else {
            landscape.push(line);
        }
    }
    landscapes
}

// Matrix transpose
fn transpose(vec: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = vec.len();
    let cols = vec[0].len();

    (0..cols).map(|col| {
        (0..rows)
            .map(|row| vec[row][col])
            .collect()}
    ).collect()
}

fn is_mirror_row(matrix: &Vec<Vec<char>>, test_row: usize) -> bool {
    let test_length = test_row.min(matrix.len() - test_row);
    for offset in 1..=test_length {
        if matrix[test_row-offset] != matrix[test_row+offset-1] {
            return false;
        }
    }
    true
}

// Almost identical to part 1, except we look for a row with one difference, not zero.
// Really? That's it?
pub fn calculate_part2() -> Result<usize>{
    let input: Vec<Vec<char>> = include_str!("../input/day13.txt")
        .lines()
        .map(|s| 
            s.chars().collect())
        .collect();

    let landscapes = parse_landscapes(input);

    let mut total = 0;
    'outer: for landscape in landscapes {
        let width = landscape[0].len();
        let length = landscape.len();
        
        for mirror_row in 1..length {
            if calculate_mirror_closeness(&landscape, mirror_row) == 1 {
                total += 100*mirror_row;
                continue 'outer;
            }
        }
        let landscape = transpose(landscape);
        for mirror_col in 1..width {
            if calculate_mirror_closeness(&landscape, mirror_col) == 1 {
                total += mirror_col;
                continue 'outer;
            }
        }

    }

    Ok(total)
}

// Rather than returning false when we find a mismatch just count the errors
fn calculate_mirror_closeness(matrix: &Vec<Vec<char>>, test_row: usize) -> usize {
    let test_length = test_row.min(matrix.len() - test_row);
    let mut count = 0;
    for offset in 1..=test_length {
        for j in 0..matrix[0].len() {
            if matrix[test_row-offset][j] != matrix[test_row+offset-1][j] {
                count += 1;
            }
        }
    }
    count
}


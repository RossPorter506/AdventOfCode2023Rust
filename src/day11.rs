use crate::prelude::*;
use itertools::Itertools;
/**/
// https://adventofcode.com/2023/day/11
/**/
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day11.txt")?;
    let input_lines = BufReader::new(file).lines().flatten();

    let mut universe: Vec<String> = vec![];

    // Expand empty rows
    for line in input_lines {
        let row_is_empty = line.chars().all(|c| c == '.');
        if row_is_empty {
            universe.push(line.clone());
        }
        universe.push(line);
    }

    // Expand empty columns
    for col in (0..universe[0].len()).rev() {
        let col_is_empty = universe.iter().all(|row| row.chars().nth(col) == Some('.'));
        if col_is_empty {
            universe.iter_mut().for_each( |row| row.insert(col, '.'));
        }
    }

    let galaxies = find_galaxy_locations(&universe);

    // Measure distance from all pairs of galaxies. 
    let total = galaxies.iter()
        .tuple_combinations()
        .map(manhattan_distance)
        .sum();

    Ok(total)
}

fn find_galaxy_locations(universe: &[String]) -> Vec<(usize, usize)> {
    let mut out = vec![];
    
    for (row, contents) in universe.iter().enumerate() {
        let galaxy_positions: Vec<(usize, usize)> = contents.chars()
            .enumerate()
            .filter_map(|(col, c)| 
                if c == '#' {Some((row, col))} else {None}
            ).collect();

        out.extend(galaxy_positions);
    }
    out
}

fn manhattan_distance(points: (&(usize, usize), &(usize, usize))) -> usize {
    let p1 = points.0;
    let p2 = points.1;
    ((p1.0 as isize - p2.0 as isize).abs() + (p1.1 as isize - p2.1 as isize).abs()) as usize
}

pub fn calculate_part2() -> Result<usize>{
    let universe: Vec<String> = include_str!("../input/day11.txt")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    
    // Find locations pre-expansion
    let mut galaxies = find_galaxy_locations(&universe);
    
    // Find indices of empty rows, cols
    let mut empty_row_indices = vec![];
    for (row, contents) in universe.iter().enumerate() {
        let row_is_empty = contents.chars().all(|c| c == '.');
        if row_is_empty {
            empty_row_indices.push(row);
        }
    }

    let mut empty_col_indices = vec![];
    for col in 0..universe[0].len() {
        let col_is_empty = universe.iter().all(|row| row.chars().nth(col) == Some('.'));
        if col_is_empty {
            empty_col_indices.push(col);
        }
    }

    // Adjust galaxy locations for expansion
    const EXPANSION_FACTOR: usize = 1_000_000;
    for galaxy in galaxies.iter_mut() {
        let num_empty_rows_before_galaxy: usize = empty_row_indices.iter().filter(|&&i| i < galaxy.0).count();
        let num_empty_cols_before_galaxy: usize = empty_col_indices.iter().filter(|&&i| i < galaxy.1).count();
        galaxy.0 += num_empty_rows_before_galaxy * (EXPANSION_FACTOR-1);
        galaxy.1 += num_empty_cols_before_galaxy * (EXPANSION_FACTOR-1);
    }

    // Measure distance from all pairs of galaxies
    let total = galaxies.iter()
        .tuple_combinations()
        .map(manhattan_distance)
        .sum();

    Ok(total)
}
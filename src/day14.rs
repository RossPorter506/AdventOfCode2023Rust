use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/14
/**/

pub fn calculate_part1() -> Result<usize>{
    let mut platform: Vec<Vec<char>> = include_str!("../input/day14.txt")
        .lines()
        .map(|s| 
            s.chars().collect())
        .collect();

    platform = tilt_platform_north(platform);

    Ok(calculate_load(platform))
}

fn tilt_platform_north(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>>{
    for row in 0..platform.len() {
        for col in 0..platform[row].len() {
            if platform[row][col] == 'O' {
                tilt_rock_north(&mut platform, (row, col));
            }
        }
    }
    platform
}

fn tilt_rock_north(platform: &mut [Vec<char>], rock_pos: (usize, usize)) {
    let rock_row = rock_pos.0;
    let rock_col = rock_pos.1;
    let mut found_empty = None;
    for i in (0..rock_row).rev() {
        match platform[i][rock_col] {
            '#' | 'O' => break,
            '.' => found_empty = Some(i),
            _ => unreachable!(),
        }
    }
    if let Some(i) = found_empty {
        platform[i][rock_col] = 'O';
        platform[rock_row][rock_col] = '.';
    }
}

fn calculate_load(platform: Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for (idx, row) in platform.iter().enumerate() {
        for &elem in row {
            if elem == 'O' {
                total += platform.len() - idx;
            }
        }
    }
    total
}

pub fn calculate_part2() -> Result<usize>{
    let mut platform: Vec<Vec<char>> = include_str!("../input/day14.txt")
        .lines()
        .map(|s| 
            s.chars().collect())
        .collect();

    let mut prev_configurations = vec![platform.clone()];

    const N_CYCLES: usize = 1_000_000_000;
    for _ in 0..N_CYCLES {
        for _ in 0..=3 {
            platform = tilt_platform_north(platform);
            platform = rotate_clockwise(platform);
        }
        // Have we seen this configuration before?
        if let Some(loop_start_pos) = prev_configurations.iter().rposition(|plat| plat == &platform) {
            // If so, loop detected. 
            let cycle_len = prev_configurations.len() - loop_start_pos;
            println!("Previous configuration detected at pos {loop_start_pos}/{}. Loop of length {cycle_len}", prev_configurations.len());
            // Calculate what state the platform will be in at sten N_CYCLES
            let final_conf = ((N_CYCLES-prev_configurations.len()) % cycle_len) + loop_start_pos;
            platform = prev_configurations[final_conf].clone();
            break;
        }
        prev_configurations.push(platform.clone());
    }
    
    Ok(calculate_load(platform))
}

fn rotate_clockwise(platform: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let rows = platform.len();
    let cols = platform[0].len();

    let mut vec = vec![];
    for col_idx in 0..cols {
        let mut row = vec![];
        for row_idx in 0..rows {
            row.push(platform[rows-1 - row_idx][col_idx]);
        }
        vec.push(row);
    }

    vec
}
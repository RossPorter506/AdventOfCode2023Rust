#![allow(dead_code)]
#![feature(iter_map_windows)]

mod day1; 
mod day2;
mod day3;
mod day4; 
mod day5; 
mod day6; 
mod day7;
mod day8;
mod day9;
mod day10; 
mod day11; use day11 as day;
mod prelude; use prelude::*;

/**/
// https://adventofcode.com/2023
/**/

fn main() -> Result<()>{
    let result = day::calculate_part2()?;
    println!("{result}");
    Ok(())
}

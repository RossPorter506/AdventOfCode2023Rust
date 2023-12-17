#![allow(dead_code)]

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
mod day11; 
mod day12; 
mod day13;
mod day14;
mod day15; use day15 as day;
mod prelude; use prelude::*;

/**/
// https://adventofcode.com/2023
/**/

fn main() -> Result<()>{
    let result = day::calculate_part2()?;
    println!("{result}");
    Ok(())
}

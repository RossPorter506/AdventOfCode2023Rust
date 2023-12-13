#![allow(dead_code)]

mod day1; 
mod day2;
mod day3;
mod day4; 
mod day5; 
mod day6; use day6 as day;
mod prelude; use prelude::*;

/**/
// https://adventofcode.com/2023
/**/

fn main() -> Result<()>{
    let result = day::calculate_part2()?;
    println!("{result}");
    Ok(())
}

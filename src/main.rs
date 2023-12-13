#![allow(dead_code)]


mod day1; 
mod day2;
mod day3;
mod day4; use day4 as day;
mod prelude; use prelude::*;

/**/
// https://adventofcode.com/2023
/**/

fn main() -> Result<()>{
    let result = day::calculate_part2()?;
    println!("{result}");
    Ok(())
}

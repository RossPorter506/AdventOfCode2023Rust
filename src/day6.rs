use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/6
/**/
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day6.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();

    let times = lines.next().unwrap().trim_start_matches("Time:").split_whitespace().map(|x| x.parse::<usize>()).collect::<Result<Vec<usize>,_>>()?;
    let dists = lines.next().unwrap().trim_start_matches("Distance:").split_whitespace().map(|x| x.parse::<usize>()).collect::<Result<Vec<usize>,_>>()?;

    let mut ans = 1;
    for (time, dist) in times.into_iter().zip(dists) {
        let num_winning: usize = calculate_num_winning_pairs(time, dist);
        dbg!(&num_winning);
        ans *= num_winning;
    }
    Ok(ans)
}

fn calculate_num_winning_pairs(time:usize, dist: usize) -> usize {
    let mut count: usize = 0;
    let (mut x, mut y) = (time.div_ceil(2), time/2);

    while x*y > dist {
        count += 2;
        x += 1; 
        y -=1;
    }

    if time%2==0 {
        count = count.saturating_sub(1);
    }
    
    count
}

pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day6.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();

    let time = lines.next().unwrap().trim_start_matches("Time:")
        .to_string().chars()
        .flat_map(|c| if c.is_ascii_whitespace() {None} else {Some(c)})
        .collect::<String>()
        .parse::<usize>()?;
    let dist = lines.next().unwrap().trim_start_matches("Distance:")
        .to_string().chars()
        .flat_map(|c| if c.is_ascii_whitespace() {None} else {Some(c)})
        .collect::<String>()
        .parse::<usize>()?;
    
    let num_winning: usize = calculate_num_winning_pairs(time, dist);
    dbg!(&num_winning);
    let ans = num_winning;
    
    Ok(ans)
}
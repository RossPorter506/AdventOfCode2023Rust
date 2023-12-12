use std::collections::HashMap;

use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/8
/**/
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day8.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();

    let instructions = lines.next().unwrap().chars()
        .map(|c| 
            match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            })
        .collect::<Vec<Direction>>();

    let map_lines: Vec<String> = lines.skip(1).collect();
    let mut map_map: HashMap<String, [String;2]> = HashMap::new();
    for line in map_lines {
        let (left, right) = line.split_once(" = ").and_then(|(_, dirs)| dirs.split_once(", ")).unwrap();
        let (mut left, mut right) = (left.chars(), right.chars());
        left.next();
        right.next_back();
        let (left, right) = (left.as_str(), right.as_str());
        map_map.insert(line[0..3].to_string(), [left.to_string(), right.to_string()]);
    }

    const INITIAL_NODE: &str = "AAA";
    const TARGET_NODE: &str = "ZZZ";
    
    let mut next_node = INITIAL_NODE.to_string(); 
    let mut steps = 0;
    
    loop {
        for next_instruction in instructions.iter() {
            if next_node == TARGET_NODE {
                return Ok(steps);
            }
            steps += 1;
            let next_dirs = map_map.get(&next_node).unwrap();
            next_node = next_dirs[*next_instruction as usize].to_string();
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left=0,
    Right=1,
}
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day8.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();

    let instructions = lines.next().unwrap().chars()
        .map(|c| 
            match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            })
        .collect::<Vec<Direction>>();

    let map_lines: Vec<String> = lines.skip(1).collect();
    let mut map_map: HashMap<String, [String;2]> = HashMap::new();
    for line in map_lines {
        let (left, right) = line.split_once(" = ").and_then(|(_, dirs)| dirs.split_once(", ")).unwrap();
        let (mut left, mut right) = (left.chars(), right.chars());
        left.next();
        right.next_back();
        let (left, right) = (left.as_str(), right.as_str());
        map_map.insert(line[0..3].to_string(), [left.to_string(), right.to_string()]);
    }

    const END_REQ: fn(&String) -> bool = |str: &String| str.ends_with('Z');
    let mut steps = 0;
    let mut current_nodes: Vec<String> = map_map.keys().filter(|l| l.ends_with('A')).map(|l| l[0..3].to_string()).collect();

    loop {
        for next_instruction in instructions.iter() {
            let next_nodes: Vec<String> = current_nodes.iter()
                .map(|current_node| {
                    let next_dirs = map_map.get(current_node).unwrap();
                    let next_node = &next_dirs[*next_instruction as usize];
                    next_node.to_string()
                    })
                .collect();
            
            if current_nodes.iter().all(END_REQ){
                return Ok(steps);
            }
            steps += 1;
            current_nodes = next_nodes;
        }
    }
}
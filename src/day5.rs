use crate::prelude::*;
use std::{collections::HashMap as Map, ops::Range};
/**/
// https://adventofcode.com/2023/day/5
/**/

pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day5.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();

    let seed_to_soil_map =              Map::<Range<usize>, Range<usize>>::new();
    let soil_to_fertilizer_map =        Map::<Range<usize>, Range<usize>>::new();
    let fertilizer_to_water_map =       Map::<Range<usize>, Range<usize>>::new();
    let water_to_light_map =            Map::<Range<usize>, Range<usize>>::new();
    let light_to_temperature_map =      Map::<Range<usize>, Range<usize>>::new();
    let temperature_to_humidity_map =   Map::<Range<usize>, Range<usize>>::new();
    let humidity_to_location_map =      Map::<Range<usize>, Range<usize>>::new();
    
    let mut maps = [
        seed_to_soil_map, soil_to_fertilizer_map, 
        fertilizer_to_water_map, water_to_light_map, 
        light_to_temperature_map, temperature_to_humidity_map, 
        humidity_to_location_map];
    let mut current_map = 0;

    let seeds: Vec<usize> = lines.next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>,_>>()?;
    let lines = lines.skip(2);

    // Construct the map
    for line in lines {
        if line.is_empty() {continue;}
        match parse_line(&line)? {
            LineContents::String(_) => current_map += 1,
            LineContents::Numbers(nums) => {
                let destination_start = nums[0];
                let source_start = nums[1];
                let len = nums[2];
                maps[current_map].insert(source_start..source_start+len, destination_start..destination_start+len);
            }
        };
    }

    let mut locations:Vec<usize> = vec![];
    
    for seed in seeds {
        let mut source = seed;
        'outer: for map in &maps {
            for source_range in map.keys() {
                if source_range.contains(&source) {
                    let dest_range = &map[source_range];
                    let position = source - source_range.start;
                    source = dest_range.start + position;
                    
                    continue 'outer;
                }
            }
            //println!("Not found. Mapping {source} to {source}");
        }
        locations.push(source);
        println!();
    }

    let ans = locations.into_iter().min().unwrap();
    Ok(ans)
}

enum LineContents<'a> {
    String(&'a str),
    Numbers(Vec<usize>),
}

fn parse_line(str: &str) -> Result<LineContents> {
    if str.contains(':') {
        Ok(LineContents::String(str))
    }
    else {
        let nums: Result<Vec<usize>, _> = str.split_whitespace()
            .map(|x| x.parse::<usize>())
            .collect();
        Ok(LineContents::Numbers(nums?))
    }
}

pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day5.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();

    let seed_to_soil_map =              Map::<Range<usize>, Range<usize>>::new();
    let soil_to_fertilizer_map =        Map::<Range<usize>, Range<usize>>::new();
    let fertilizer_to_water_map =       Map::<Range<usize>, Range<usize>>::new();
    let water_to_light_map =            Map::<Range<usize>, Range<usize>>::new();
    let light_to_temperature_map =      Map::<Range<usize>, Range<usize>>::new();
    let temperature_to_humidity_map =   Map::<Range<usize>, Range<usize>>::new();
    let humidity_to_location_map =      Map::<Range<usize>, Range<usize>>::new();
    
    let mut maps = [
        seed_to_soil_map, soil_to_fertilizer_map, 
        fertilizer_to_water_map, water_to_light_map, 
        light_to_temperature_map, temperature_to_humidity_map, 
        humidity_to_location_map];
    let mut current_map = 0;

    let seeds: Vec<usize> = lines.next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>,_>>()?;
    
    // Of course in part 2 the seeds are actually ranges too.
    let mut seed_ranges: Vec<Range<usize>> = vec![];
    for seed_pair in seeds.as_slice().chunks(2) {
        assert!(seed_pair.len() == 2);
        let start = seed_pair[0];
        let len = seed_pair[1];
        seed_ranges.push(start..start+len);
    }
    let lines = lines.skip(2);

    // Construct the maps
    for line in lines {
        if line.is_empty() {continue;}
        match parse_line(&line)? {
            LineContents::String(_) => {current_map += 1;},
            LineContents::Numbers(nums) => {
                let destination_start = nums[0];
                let source_start = nums[1];
                let range_length = nums[2];
                //println!("Inserting mapping from {}..{} to {}..{}", source_start, source_start+range_length, destination_start, destination_start+range_length);
                maps[current_map].insert(source_start..source_start+range_length, destination_start..destination_start+range_length);
            }
        };
    }

    let mut locations:Vec<Range<usize>> = vec![];
    
    for seed_range in seed_ranges {
        //println!("Beginning with seed range {seed_range:?}");
        let mut sources = vec![seed_range.clone()];
        let mut dests;
        // For each map, map all values in sources before continuing.
        for map in &maps {
            // As we map values they will move from sources to dests.
            dests = vec![];
            // For each range in sources...
            'middle: while let Some(input_range) = sources.pop() {
                // Compare them to the ranges we previously got from reading the input.
                for compare_range in map.keys() {
                    // The ranges will probably not line up perfectly. Split them into sections that are above, below, or contained within.
                    // Anything contained within one of the map ranges can be mapped as such. Any leftovers above or below will get put back into sources to be matched against again.
                    let (below, overlapping, above) 
                        = find_intersection(input_range.clone(), compare_range);
                    
                    // Map whatever part overlaps with this range
                    if let Some(range) = overlapping {
                        let dest_range = &map[compare_range];
                        let start_distance = range.start - compare_range.start;
                        let len = range.len();
                        let mapped_range = (dest_range.start+start_distance)..(dest_range.start+start_distance+len);
                        //println!("We are mapping {range:?} because it is within {possible_source_range:?}, which gets mapped to {dest_range:?}. Hence we map {range:?} to {mapped_range:?}");
                        dests.push(mapped_range);
                        
                        // Put any non-overlapping parts of the range back in the sources list to be matched against later
                        if let Some(range) = below {
                            sources.push(range)
                        }
                        if let Some(range) = above {
                            sources.push(range)
                        }

                        // We are done with this range, continue with the next range in the list
                        continue 'middle;
                    }
                }
                // We looped over all the ranges in the map but did not find a match for this range. These are mapped with the identity fn, i.e. no change.
                dests.push(input_range);
            }
            // We have finished mapping everything to the next stage. To get ready for the next stage we refill sources by setting it equal to dests.
            sources = dests;
        }
        locations.extend(sources);
    }
    let ans = locations.into_iter().map(|range| range.start).reduce(std::cmp::min).unwrap();
    Ok(ans)
}

/// Given a range r1, split it into one or more ranges such that the first return value is the range below r2, the middle is the intersection of r1 and r2, and the last return value is the range above r2
fn find_intersection(r1: Range<usize>, r2: &Range<usize>) -> (Option<Range<usize>>, Option<Range<usize>>, Option<Range<usize>>) {
    // We symbolise r1 using round brackets ( ) and r2 as square brackets [ ]

    // If r2 contains r1 entirely then the intersection is just r1: [ ( ) ]
    if r2.contains(&r1.start) && r2.contains(&(r1.end-1).max(r1.start)) {
        return (None, Some(r1), None);
    }
    // If we overlap on the top half: ( [ ) ]
    else if r2.contains(&(r1.end-1).max(r1.start)) {
        return (Some(r1.start..r2.start), Some(r2.start..r1.end), None);
    }
    //e.g. r1 = 5..15, r2 = 0..10
    // If we overlap on the bottom half: [ ( ] )
    else if r2.contains(&r1.start) {
        return (None, Some(r1.start..r2.end), Some(r2.end..r1.end));
    }
    // r1 = 0..20, r2 = 10..15
    // If r1 contains r2 then split into three chunks. ( [   ] )
    else if r1.contains(&r2.start) && r1.contains(&(r2.end-1).max(r1.start)) {
        return (Some(r1.start..r2.start), Some(r2.clone()), Some(r2.end..r1.end));
    }
    // (  ) [  ]
    else if r1.end <= r2.start {
        return (Some(r1), None, None);
    }
    // [  ] (  )
    else if r1.start >= r2.end {
        return (None, None, Some(r1));
    }
    unreachable!()
}
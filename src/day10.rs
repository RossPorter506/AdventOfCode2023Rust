use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/10
/**/
pub fn calculate_part1() -> Result<usize>{
    let input = include_str!("../input/day10.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = input[0].len();
    let length = input.len();

    let mut dist_from_start: Vec<Vec<Option<usize>>> = vec![ vec![None; width]; length ];

    let start_row = input.iter().position(|row| row.contains(&'S')).unwrap();
    let start_col = input[start_row].iter().position(|c| c==&'S').unwrap();
    let start = (start_row, start_col);

    let finder_builder = FinderBuilder::new(&input);

    dist_from_start[start_row][start_col] = Some(0);
    let mut points_to_search: Vec<(usize, usize)> = finder_builder.build(start).find_neighbours();

    // Calculate distance from start to all reachable nodes
    loop{
        let mut new_points: Vec<(usize, usize)> = vec![];
        for &point in points_to_search.iter() {
            let mut finder = finder_builder.build(point);

            // Find neighbours
            let neighbours = finder.find_neighbours();

            // Calculate distance from start
            let min_neighbour_distance = neighbours.iter().flat_map(|&(row, col)| dist_from_start[row][col]).min().unwrap();
            dist_from_start[point.0][point.1] = Some( min_neighbour_distance + 1 );

            // Explore unseen neighbours
            let unseen_neighbours: Vec<_> = neighbours.into_iter().filter(|&(row, col)| dist_from_start[row][col].is_none()).collect();
            new_points.extend(unseen_neighbours);
        }
        if new_points.is_empty() {break}
        points_to_search = new_points;
    }

    let max = dist_from_start.into_iter().flat_map(|row| row.into_iter().max()).max().unwrap().unwrap();

    Ok(max)
}

/// A builder struct that can generate instances of Finder, used to determine the locations of valid neighbours.
struct FinderBuilder<'a> {
    input: &'a Vec<Vec<char>>,
    width: usize,
    length: usize,
}
impl<'a> FinderBuilder<'a> {
    fn new(input: &'a Vec<Vec<char>>) -> Self {
        let width = input[0].len();
        let length = input.len();
        Self{input, width, length}
    }
    /// Create an instance of a Finder that finds the valid neighbours of `point`
    fn build(&self, point: (usize,usize)) -> Finder {
        Finder {input: self.input, width: self.width, length: self.length, point}
    }
}

/// Helper struct that determines which neighbours are valid connections.
struct Finder<'a> {
    input: &'a Vec<Vec<char>>,
    width: usize,
    length: usize,
    point: (usize, usize),
}
impl Finder<'_> {
    fn find_neighbours(&mut self) -> Vec<(usize, usize)> {
        let mut vec = vec![];
        // north
        if self.point.0 > 0 && self.is_valid_neighbour((-1, 0)){ 
            vec.push((self.point.0-1, self.point.1));
        }
        // south
        if self.point.1 > 0 && self.is_valid_neighbour((0, -1)){ 
            vec.push((self.point.0, self.point.1-1))
        }
        // east
        if self.point.0 < self.length-1 && self.is_valid_neighbour((1, 0)){ 
            vec.push((self.point.0+1, self.point.1))
        }
        // west
        if self.point.1 < self.width-1 && self.is_valid_neighbour((0, 1)){ 
            vec.push((self.point.0, self.point.1+1))
        }
        vec
    }

    fn is_valid_neighbour(&self, relative_dir: (isize, isize)) -> bool {
        // A neighbour is valid if we point into them and they point into us.
        const SYMBOLS_POINTING_UP:      &[char] = &['|', 'L', 'J', 'S'];
        const SYMBOLS_POINTING_DOWN:    &[char] = &['|', '7', 'F', 'S'];
        const SYMBOLS_POINTING_LEFT:    &[char] = &['-', 'L', 'F', 'S'];
        const SYMBOLS_POINTING_RIGHT:   &[char] = &['-', 'J', '7', 'S'];
        
        let us = &self.input[self.point.0][self.point.1];

        // Safe, as find_neighbours has already checked bounds.
        let them = &self.input[self.point.0.wrapping_add_signed(relative_dir.0)][self.point.1.wrapping_add_signed(relative_dir.1)];

        match relative_dir {
            (1, 0) => SYMBOLS_POINTING_DOWN.contains(us)  && SYMBOLS_POINTING_UP.contains(them),
            (-1,0) => SYMBOLS_POINTING_UP.contains(us) && SYMBOLS_POINTING_DOWN.contains(them),
            (0, 1) => SYMBOLS_POINTING_LEFT.contains(us) && SYMBOLS_POINTING_RIGHT.contains(them),
            (0,-1) => SYMBOLS_POINTING_RIGHT.contains(us) && SYMBOLS_POINTING_LEFT.contains(them),
            _ => unreachable!(),
        }
    }
}

pub fn calculate_part2() -> Result<usize>{
    let mut input = include_str!("../input/day10.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = input[0].len();
    let length = input.len();

    let mut dist_from_start: Vec<Vec<Option<usize>>> = vec![ vec![None; width]; length ];

    let start_row = input.iter().position(|row| row.contains(&'S')).unwrap();
    let start_col = input[start_row].iter().position(|c| c==&'S').unwrap();
    let start = (start_row, start_col);

    let finder_builder = FinderBuilder::new(&input);

    dist_from_start[start_row][start_col] = Some(0);
    let mut points_to_search: Vec<(usize, usize)> = finder_builder.build(start).find_neighbours();

    // Calculate distance from start to all reachable nodes
    loop{
        let mut new_points: Vec<(usize, usize)> = vec![];
        for &point in points_to_search.iter() {
            let mut finder = finder_builder.build(point);

            // Find neighbours
            let neighbours = finder.find_neighbours();

            // Calculate distance from start
            let min_neighbour_distance = neighbours.iter().flat_map(|&(row, col)| dist_from_start[row][col]).min().unwrap();
            dist_from_start[point.0][point.1] = Some( min_neighbour_distance + 1 );

            // Explore unseen neighbours
            let unseen_neighbours: Vec<_> = neighbours.into_iter().filter(|&(row, col)| dist_from_start[row][col].is_none()).collect();
            new_points.extend(unseen_neighbours);
        }
        if new_points.is_empty() {break}
        points_to_search = new_points;
    }

    // Replace S with it's equivalent symbol
    let points = finder_builder.build(start).find_neighbours();
    let relative_neighbour_locations = (
        (points[0].0 as isize - start.0 as isize, points[0].1 as isize - start.1 as isize),
        (points[1].0 as isize - start.0 as isize, points[1].1 as isize - start.1 as isize)
    );
    let new_char = match relative_neighbour_locations {
        (( 1, 0), (-1, 0)) | ((-1, 0), ( 1, 0)) => '|',
        (( 0, 1), ( 0,-1)) | (( 0,-1), ( 0, 1)) => '-',
        ((-1, 0), ( 0,-1)) | (( 0,-1), (-1, 0)) => 'J',
        ((-1, 0), ( 0, 1)) | (( 0, 1), (-1, 0)) => 'L',
        (( 1, 0), ( 0,-1)) | (( 0,-1), ( 1, 0)) => '7',
        (( 1, 0), ( 0, 1)) | (( 0, 1), ( 1, 0)) => 'F',
        _ => unreachable!(),
    };
    
    input[start_row][start_col] = new_char;

    // We effectively use the concept of a winding number to keep track of whether we're inside or outside.
    // A tile counts as 'inside' when it is within an odd number of main loop pipes. Since we're iterating horizontally we only care about vertical pipes. Corners are tricky.
    let mut enclosed_tiles = 0;
    for (row_idx, row) in input.into_iter().enumerate() {
        let mut is_inside = false;

        // The effect of a corner on the winding number can only be calculated when another corner appears. Store any uncalculated corner here.
        let mut unmatched_corner: Option<char> = None;

        for (col, char) in row.into_iter().enumerate() {
            // If we are looking at something that is part of the main loop it can't count as enclosed, but it does affect whether the winding number (i.e. inside/outside) changes
            if dist_from_start[row_idx][col].is_some() {
                match (unmatched_corner, char) {
                    (_ , '|') => is_inside = !is_inside, // verticals always increase winding number
                    (Some('L'), '7') | (Some('F'), 'J') => {is_inside = !is_inside; unmatched_corner = None}, // corner pairs that have a net 'verticality' also increase the winding number
                    (Some('F'), '7') | (Some('L'), 'J') => unmatched_corner = None, // corner pairs that go back in the direction they started from don't increase the winding number.
                    (None, x) if "LF".contains(x) => unmatched_corner = Some(x), // if we encounter a corner with no corner already stored, store it and move on. (Will always be right-facing corners as it's a closed loop)
                    (_, '-') => (), // horizontals have no effect.
                    _ => unreachable!(),
                };
            }
            // Otherwise if it's not part of the main loop we count the tile as enclosed or not
            else {
                enclosed_tiles += is_inside as usize; 
            }
        }
    }

    Ok(enclosed_tiles)
}

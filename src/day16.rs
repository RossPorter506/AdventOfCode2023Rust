use crate::prelude::*;

/**/
// https://adventofcode.com/2023/day/16
/**/

pub fn calculate_part1() -> Result<usize>{
    let contraption: Vec<Vec<char>> = include_str!("../input/day16.txt")
        .lines()
        .map(|s| 
            s.chars().collect())
        .collect();

    Ok( test_path(Path::default(), &contraption) )
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
struct Path {
    position: (usize, usize),
    direction: Direction,
}
impl Path {
    /// Where this path will go next
    fn next(&self, contraption: &Vec<Vec<char>>) -> Option<Vec<Path>> {
        let max_len = contraption.len();
        let max_width = contraption[0].len();
        let maximums = (max_len, max_width);
        let current_char = contraption[self.position.0][self.position.1];

        let mut next_paths: Vec<Path> = vec![];
        match current_char {
            '|' if self.direction.is_horizontal() => {
                next_paths.extend( Path{position: self.position, direction: Direction::Upwards}.next_simple_path(maximums));
                next_paths.extend( Path{position: self.position, direction: Direction::Downwards}.next_simple_path(maximums));
            },
            '-' if self.direction.is_vertical() => {
                next_paths.extend( Path{position: self.position, direction: Direction::Leftwards}.next_simple_path(maximums));
                next_paths.extend( Path{position: self.position, direction: Direction::Rightwards}.next_simple_path(maximums));
            },
            mirror if r"/\".contains(mirror) => {
                let direction = self.direction.reflected_by(mirror);
                let maybe_path = Path{position: self.position, direction}.next_simple_path(maximums);
                next_paths.extend( maybe_path );
            }
            '.' | '|' | '-' => next_paths.extend( self.next_simple_path(maximums) ),
            _ => unreachable!(),
        };
        Some(next_paths)
    }
    /// Where this path will be if it is allowed to move in the direction it is pointing. Returns None if it goes out of bounds.
    fn next_simple_path(&self, max: (usize, usize)) -> Option<Path> {
        let position = match self.direction {
            Direction::Upwards    if self.position.0 > 0        => (self.position.0-1, self.position.1),
            Direction::Downwards  if self.position.0 < max.0-1  => (self.position.0+1, self.position.1),
            Direction::Leftwards  if self.position.1 > 0        => (self.position.0, self.position.1-1),
            Direction::Rightwards if self.position.1 < max.1-1  => (self.position.0, self.position.1+1),
            _ => return None,
        };
        Some( Path{position, direction: self.direction} )
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
enum Direction {
    Upwards,
    Downwards,
    Leftwards,
    #[default]
    Rightwards,
}
impl Direction {
    fn is_vertical(&self) -> bool {
        use Direction::*;
        match &self {
            Upwards | Downwards => true,
            Leftwards | Rightwards => false,
        }
    }
    fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }
    fn reflected_by(&self, mirror: char) -> Direction {
        use Direction::*;
        match (mirror, self) {
            ('/', Upwards)    => Rightwards,
            ('\\', Downwards) => Rightwards,
            ('/', Downwards)  => Leftwards,
            ('\\', Upwards)   => Leftwards,
            ('/', Leftwards)  => Downwards,
            ('\\', Rightwards) => Downwards,
            ('/', Rightwards) => Upwards,
            ('\\', Leftwards) => Upwards,
            _ => panic!("Non-mirror character {mirror} provided"),
        }
    }
}

pub fn calculate_part2() -> Result<usize>{
    let contraption: Vec<Vec<char>> = include_str!("../input/day16.txt")
        .lines()
        .map(|s| 
            s.chars().collect())
        .collect();
    
    let max_row = contraption[0].len();
    let max_col = contraption.len();
    let first_column_starts = (0..max_row).map(|i| Path{position: (i,0),            direction: Direction::Rightwards} );
    let last_column_starts  = (0..max_row).map(|i| Path{position: (i,max_col-1),    direction: Direction::Leftwards} );
    let first_row_starts    = (0..max_col).map(|i| Path{position: (0,i),            direction: Direction::Downwards} );
    let last_row_starts     = (0..max_col).map(|i| Path{position: (max_row-1,i),    direction: Direction::Upwards} );

    let perimeter: Vec<_> = first_column_starts
        .chain(first_row_starts)
        .chain(last_column_starts)
        .chain(last_row_starts)
        .collect();
    
    let total = perimeter.iter()
        .map(|&start_path| test_path(start_path, &contraption))
        .max().unwrap();
    Ok( total )
}

/// Returns the number of energized tiles created by a path.
fn test_path(path: Path, contraption: &Vec<Vec<char>>) -> usize {
    let mut energized_overlay = contraption.clone();

    let mut prev_paths = vec![];
    let mut light_paths = vec![path];
    energized_overlay[path.position.0][path.position.1] = '#';

    while let Some(path) = light_paths.pop() {
        // Follow the current path, adhering to contraption effects.
        let next_paths = path.next(contraption);

        // mark these new paths as energized. Add any unseen paths to be explored next
        for &path in next_paths.iter().flatten() {
            energized_overlay[path.position.0][path.position.1] = '#';
            if !prev_paths.contains(&path) {
                light_paths.push(path);
                prev_paths.push(path);
            } 
        }
    }
    energized_overlay.into_iter().map(|row| row.into_iter().filter(|&chr| chr == '#').count()).sum()
}
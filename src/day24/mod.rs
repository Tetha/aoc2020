use std::collections::{HashSet, VecDeque};

use fasthash::city::crc::Hash128;

use crate::AdventError;


#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug)]
struct InputDecoder {
    chars: VecDeque<char>,
}

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let tiles = find_black_tiles(input);
    println!("There are {} black tiles", tiles.len());
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut tiles = find_black_tiles(input);
//    println!("Day 0: {}", tiles.len());

    for day in 1..=100 {
        tiles = do_simulation_step(tiles);
//        println!("Day {}: {}", day, tiles.len());
        // println!("{}", itertools::join(tiles.iter().map(|&(q, r)| format!("{}/{}", r, q)), ", "));
    }
    println!("There are {} black tiles", tiles.len());
    Ok(())
}
pub fn test() -> Result<(), AdventError> {
    let input = include_str!("example_input");
    let mut tiles = find_black_tiles(input);
    println!("Day 0: {}", tiles.len());

    for day in 1..=100 {
        tiles = do_simulation_step(tiles);
        println!("Day {}: {}", day, tiles.len());
        // println!("{}", itertools::join(tiles.iter().map(|&(q, r)| format!("{}/{}", r, q)), ", "));
    }
    Ok(())
}
fn find_black_tiles(input: &str) -> HashSet<(i32, i32), Hash128> {
    let mut black_tiles = HashSet::with_hasher(Hash128);
    for l in input.lines() {
        let coord = to_coordinate(l);
        if !black_tiles.insert(coord) {
            black_tiles.remove(&coord);
        }
    }
    black_tiles
}

fn to_coordinate(input: &str) -> (i32, i32) {
    let mut q = 0;
    let mut r = 0;

    for direction in InputDecoder::new(input) {
        let (dq, dr) = to_axial_offset(direction);
        q += dq;
        r += dr;
    }
    (q, r)
}

fn do_simulation_step(black_cells: HashSet<(i32, i32), Hash128>) -> HashSet<(i32, i32), Hash128> {
    let possibly_active_cells = black_cells.iter()
                                                          .flat_map(|&(r, q)| enumerate_neighbour_offsets(r, q))
                                                          .collect::<HashSet<(i32, i32)>>();
    
    let mut result: HashSet<(i32, i32), Hash128> = HashSet::with_hasher(Hash128);

    for (r, q) in possibly_active_cells {
        let black_neighbours = enumerate_neighbour_offsets(r, q)
                               .iter()
                               .filter(|&c| black_cells.contains(c))
                               .count();
        //println!("{}/{} has {} neighbours", r, q, black_neighbours);
        if black_cells.contains(&(r, q)) {
            // current is black
            let cell_dies = black_neighbours == 0 || black_neighbours > 2;
            if !cell_dies {
                result.insert((r, q));
            }
        } else {
            // current is white
            if black_neighbours == 2 {
                result.insert((r, q));
            }
        }
    }
    result
}
fn enumerate_neighbour_offsets(r: i32, q: i32) -> Vec<(i32, i32)> {
    vec![
        Direction::East,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
        Direction::NorthEast,
    ].iter()
     .map(|&d| to_axial_offset(d))
     .map(|(dr, dq)| (r + dr, q + dq))
     .collect::<Vec<(i32, i32)>>()
}

fn to_axial_offset(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::East => (1, 0),
        Direction::SouthEast => (0, 1),
        Direction::SouthWest => (-1, 1),
        Direction::West => (-1, 0),
        Direction::NorthWest => (0, -1),
        Direction::NorthEast => (1, -1),
    }
}

impl InputDecoder {
    fn new(input: &str) -> InputDecoder {
        InputDecoder{
            chars: VecDeque::from(input.chars().collect::<Vec<char>>()),
        }
    }
}
impl Iterator for InputDecoder {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.pop_front() {
            Some('w') => Some(Direction::West),
            Some('e') => Some(Direction::East),
            Some('s') => match self.chars.pop_front() {
                Some('w') => Some(Direction::SouthWest),
                Some('e') => Some(Direction::SouthEast),
                Some(_) => panic!("unexpected char after s"),
                None => panic!("incomplete s"),
            },
            Some('n') => match self.chars.pop_front() {
                Some('w') => Some(Direction::NorthWest),
                Some('e') => Some(Direction::NorthEast),
                Some(_) => panic!("unexpected char after n"),
                None => panic!("incomplete n"),
            },
            Some(_) => panic!("unrecognized char"),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example_input");
        assert_eq!(10, find_black_tiles(input).len());
    }

    #[test]
    fn test_first_example_line() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw";
        assert_eq!(
            InputDecoder::new(input).collect::<Vec<Direction>>(),
            vec![
                Direction::SouthEast,
                Direction::SouthEast,
                Direction::NorthWest,
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::West,
                Direction::SouthEast,
                Direction::East,
                Direction::SouthWest,
                Direction::West,
                Direction::SouthWest,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::West,
                Direction::SouthEast,
                Direction::West,
                Direction::SouthWest,
            ],
        );
    }
}
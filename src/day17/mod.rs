use std::collections::HashSet;

use defaultmap::DefaultHashMap;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut state = parse_input(input);
    for i in 0..6 {
        state = state.step();
    }
    println!("There are {} active cubes", state.count_cubes());
    Ok(())
}

fn parse_input(s: &str) -> DimensionState {
    let mut result = DimensionState::new();
    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                result.activate_cube(x as i32, y as i32, 0, 0);
            }
        }
    }
    result
}

fn calculate_neighbours(x: i32, y: i32, z: i32, w: i32) -> HashSet<(i32, i32, i32, i32)> {
    let mut result = HashSet::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                        result.insert((x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }
    }
    return result;
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ConwayCube {
    Active, Inactive,
}

#[derive(Debug)]
struct DimensionState {
    cells: DefaultHashMap<(i32, i32, i32, i32), ConwayCube>,
}

impl DimensionState {
    fn new() -> DimensionState {
        DimensionState{cells: DefaultHashMap::new(ConwayCube::Inactive)}
    }

    fn activate_cube(&mut self, x: i32, y: i32, z: i32, w: i32) {
        self.cells.insert((x, y, z, w), ConwayCube::Active);
    }

    fn count_cubes(&self) -> usize {
        return self.cells.len();
    }

    fn step(&self) -> DimensionState {
        let mut possibly_active_cubes: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for &(x, y, z, w) in self.cells.keys() {
            possibly_active_cubes.insert((x, y, z, w));
            for neighbour in calculate_neighbours(x, y, z, w) {
                possibly_active_cubes.insert(neighbour);
            }
        }

        let mut next_cells: DefaultHashMap<(i32, i32, i32, i32), ConwayCube> = DefaultHashMap::new(ConwayCube::Inactive);
        for (x, y, z, w) in possibly_active_cubes {
            match &self.cells[(x, y, z, w)] {
                ConwayCube::Active => {
                    let active_neighbours = calculate_neighbours(x, y, z, w)
                        .iter()
                        .map(|n| self.cells[n] )
                        .filter(|&s| s == ConwayCube::Active)
                        .count();
                   
                    //println!("  active cube {:>3} {:>3} {:>3} has {} active neighbours", x, y, z, active_neighbours);
                    if active_neighbours == 2 || active_neighbours == 3 {
                        next_cells.insert((x, y, z, w), ConwayCube::Active);
                    }
                }
                ConwayCube::Inactive => {
                    let active_neighbours = calculate_neighbours(x, y, z, w)
                        .iter()
                        .map(|n| self.cells[n] )
                        .filter(|&s| s == ConwayCube::Active)
                        .count();
                    
                    //println!("inactive cube {:>3} {:>3} {:>3} has {} active neighbours", x, y, z, active_neighbours);
                    if active_neighbours == 3 {
                        next_cells.insert((x, y, z, w), ConwayCube::Active);
                    }
                }
            }
        }
        return DimensionState{cells: next_cells};
    }

    fn dump_to_stdout(&self) {
        let min_x = self.cells.keys().map(|k| k.0 ).min().unwrap();
        let max_x = self.cells.keys().map(|k| k.0 ).max().unwrap();

        let min_y = self.cells.keys().map(|k| k.1 ).min().unwrap();
        let max_y = self.cells.keys().map(|k| k.1 ).max().unwrap();

        let min_z = self.cells.keys().map(|k| k.2 ).min().unwrap();
        let max_z = self.cells.keys().map(|k| k.2 ).max().unwrap();

        let min_w = self.cells.keys().map(|k| k.3 ).min().unwrap();
        let max_w = self.cells.keys().map(|k| k.3 ).max().unwrap();

        for w in min_w..=max_w {
            println!("z={}", w);
            for z in min_z..=max_z {
                println!("z={}", z);
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        match self.cells[(x, y, z,  w)] {
                            ConwayCube::Inactive => print!("."),
                            ConwayCube::Active => print!("#"),
                        }
                    }
                    println!("")
                }
                println!("")
            }
            println!("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one(){
        let input = ".#.
..#
###";

        let mut subject = parse_input(input);
        /*
        let mut subject = DimensionState::new();
        subject.activate_cube(1, 0, 0);
        subject.activate_cube(2, 1, 0);
        subject.activate_cube(0, 2, 0);
        subject.activate_cube(1, 2, 0);
        subject.activate_cube(2, 2, 0);
        */

        subject = subject.step();
        subject = subject.step();
        subject = subject.step();
        subject = subject.step();
        subject = subject.step();
        subject = subject.step();

        assert_eq!(112, subject.count_cubes());
    }
}
use std::collections::HashMap;

use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let input = vec![2,1,10,11,0,6];
    let mut memory_game = MemoryGame::new(input);
    let output = memory_game.nth(2019);
    if let Some(value) = output {
        println!("");
        println!("{}", value);
    }
    Ok(())
}

pub fn part2_test() -> Result<(), AdventError> {
    let input = vec![0,3,6];
    //let input = vec![2,1,10,11,0,6];
    let mut memory_game = MemoryGame::new(input);
    let output = memory_game.nth(30000000 - 1);
    if let Some(value) = output {
        println!("");
        println!("{}", value);
    }
    Ok(())
}
pub fn part2() -> Result<(), AdventError> {
    let input = vec![2,1,10,11,0,6];
    let mut memory_game = MemoryGame::new(input);
    let output = memory_game.nth(30000000 - 1);
    if let Some(value) = output {
        println!("");
        println!("{}", value);
    }
    Ok(())
}
struct MemoryGame {
    iterations: u64,
    next_starting_number: usize,
    in_starting_numbers: bool,
    pending_distance_num: Option<u64>,
    numbers: Vec<u64>,
    distances: HashMap<u64, u64>,
}

impl MemoryGame {
    fn new(starting_numbers: Vec<u64>) -> MemoryGame {
        MemoryGame{
            iterations: 0,
            next_starting_number: 0,
            in_starting_numbers: true,
            pending_distance_num: None,
            numbers: starting_numbers,
            distances: HashMap::new()
        }
    }
}
impl Iterator for MemoryGame {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations % 10000 == 0 {
            print!(".");
        }
        if self.iterations % 1000000 == 0 {
            println!("");
        }
        self.iterations += 1;
        if self.in_starting_numbers && self.next_starting_number < self.numbers.len() {
            let result = self.numbers[self.next_starting_number];
            self.next_starting_number += 1;
            if let Some(i) = self.pending_distance_num {
                self.distances.insert(i, self.iterations);
            }
            self.pending_distance_num = Some(result);
            return Some(result);
        } else {
            self.in_starting_numbers = false;
        }
        let most_recent_number = self.pending_distance_num;
        match most_recent_number {
            None => return None,
            Some(i) => {
                let prev_occurence = self.distances.get(&i);
                match prev_occurence {
                    None => {
                        //self.numbers.push(0);
                        if let Some(i) = self.pending_distance_num {
                            self.distances.insert(i, self.iterations);
                        }
                        self.pending_distance_num = Some(0);
                        return Some(0)
                    }
                    Some(&last_iteration) => {
                        if let Some(i) = self.pending_distance_num {
                            self.distances.insert(i, self.iterations);
                        }
                        self.pending_distance_num = Some(self.iterations - last_iteration);
                        //self.numbers.push(distance);
                        return Some(self.iterations - last_iteration)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let mut subject = MemoryGame::new(vec![0, 3, 6]);
        assert_eq!(Some(0), subject.next());
        assert_eq!(Some(3), subject.next());
        assert_eq!(Some(6), subject.next());
        assert_eq!(Some(0), subject.next());
        assert_eq!(Some(3), subject.next());
        assert_eq!(Some(3), subject.next());
        assert_eq!(Some(1), subject.next());
        assert_eq!(Some(0), subject.next());
        assert_eq!(Some(4), subject.next());
        assert_eq!(Some(0), subject.next());
    }

    //#[test]
    //fn test_example_two() {
    //    let mut subject = MemoryGame::new(vec![0,3,6]);
    //    let output = subject.nth(30000000 - 1);
    //    assert_eq!(Some(175594), output);
    //}
}
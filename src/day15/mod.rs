use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let input = vec![2,1,10,11,0,6];
    let mut memory_game = MemoryGame::new(input);
    let output = memory_game.nth(2019);
    if let Some(value) = output {
        println!("{}", value);
    }
    Ok(())
}
struct MemoryGame {
    next_starting_number: usize,
    in_starting_numbers: bool,
    numbers: Vec<u64>,
}

impl MemoryGame {
    fn new(starting_numbers: Vec<u64>) -> MemoryGame {
        MemoryGame{
            next_starting_number: 0,
            in_starting_numbers: true,
            numbers: starting_numbers
        }
    }
}
impl Iterator for MemoryGame {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.in_starting_numbers && self.next_starting_number < self.numbers.len() {
            let result = self.numbers[self.next_starting_number];
            self.next_starting_number += 1;
            return Some(result);
        } else {
            self.in_starting_numbers = false;
        }
        let most_recent_number = self.numbers.last();
        match most_recent_number {
            None => return None,
            Some(&i) => {
                let prev_occurence = self.numbers.iter().rev().enumerate().skip(1).find(|(_, &num)| num == i);
                match prev_occurence {
                    None => {
                        self.numbers.push(0);
                        return Some(0)
                    }
                    Some((idx, _)) => {
                        self.numbers.push(idx as u64);
                        return Some(idx as u64)
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

    #[test]
    fn test_example_two() {
        let mut subject = MemoryGame::new(vec![0,3,6]);
        let output = subject.nth(30000000 - 1);
        assert_eq!(Some(175594), output);
    }
}
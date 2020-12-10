use std::num::ParseIntError;

use itertools::Itertools;

use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let numbers:  Vec<u64> = input.lines().map(|s| s.parse::<u64>() ).collect::<Result<Vec<u64>, ParseIntError>>()?;

    for i in 26..numbers.len() {
        if !check_number(&numbers, i) {
            println!("{}", numbers[i]);
        }
    }
    Ok(())
}

// 26796446

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let numbers:  Vec<u64> = input.lines().map(|s| s.parse::<u64>() ).collect::<Result<Vec<u64>, ParseIntError>>()?;
    let target = 26796446;

    for start in 0..numbers.len() {
        for end in start..numbers.len() {
            let slice = &numbers[start..end];
            if slice.iter().copied().sum::<u64>() == target {
                let least = slice.iter().min();
                let most = slice.iter().max();
                println!("{} + {} = {}", least.unwrap(), most.unwrap(), least.unwrap()+most.unwrap());
            }
        }
    }
    Ok(())
}

fn check_number(numbers: &Vec<u64>, i: usize) -> bool {
    let previous_numbers = &numbers[i-25..i];
    //println!("{}", previous_numbers.len());
    itertools::any(previous_numbers.iter().combinations(2), |v| v[0]+v[1] == numbers[i])
}
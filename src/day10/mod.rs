use std::num::ParseIntError;

use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let numbers = input.lines().map(|l| l.parse::<u32>()).collect::<Result<Vec<u32>, ParseIntError>>()?;
    let (one_diff, three_diff)= find_chargers(&numbers);
    println!("{} * {} = {}", one_diff, three_diff, one_diff * three_diff);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let numbers = input.lines().map(|l| l.parse::<u32>()).collect::<Result<Vec<u32>, ParseIntError>>()?;
    let possibilities = find_possibilities(&numbers);
    println!("{} possibilities", possibilities);
    Ok(())
}

fn find_possibilities(joltages: &Vec<u32>) -> u64 {
    let mut joltages = joltages.clone();
    joltages.push(0);
    joltages.push(*joltages.iter().max().unwrap_or(&0) + 3);
    joltages.sort();

    let mut possibilities_from_here: Vec<u64> = Vec::new();
    for i in 0..joltages.len() {
        possibilities_from_here.push(0);
    }

    possibilities_from_here[joltages.len()-1] = 1; // 1 option at our device
    for i in (0..possibilities_from_here.len()-1).rev() {
        let mut options = 0;
        for offset in 1..=3 {
            if (i+offset < possibilities_from_here.len()) && (joltages[i+offset] - joltages[i] <= 3) {
                options += possibilities_from_here[i+offset];
            }
        }
        possibilities_from_here[i] = options;
    }
    possibilities_from_here[0]
}

fn find_chargers(joltages: &Vec<u32>) -> (u32, u32) {
    let mut sorted_joltages = joltages.clone();

    sorted_joltages.push(0);
    sorted_joltages.push(*joltages.iter().max().unwrap_or(&0) + 3);
    sorted_joltages.sort();
    println!("{:?}", sorted_joltages);

    let mut one_diff = 0;
    let mut three_diff = 0;

    for (first, second) in sorted_joltages.iter().zip(sorted_joltages.iter().skip(1)) {
        if (second - first) == 1 {
            one_diff += 1;
        }

        if (second - first) == 3 {
            three_diff += 1;
        }
    }

    return (one_diff, three_diff);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let input = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        ];

        let (one_diff, three_diff) = find_chargers(&input);
        assert_eq!(7, one_diff);
        assert_eq!(5, three_diff);
    }
    
    #[test]
    pub fn test_example2() {
        let input = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        ];
        let (one_diff, three_diff) = find_chargers(&input);
        assert_eq!(22, one_diff);
        assert_eq!(10, three_diff);
    }

    #[test]
    fn test_part2_example_one() {
        let input = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        ];

        let possibilities = find_possibilities(&input);
        assert_eq!(8, possibilities);
    }
}
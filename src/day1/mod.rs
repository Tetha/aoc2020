use std::num::ParseIntError;

use crate::AdventError;


pub fn day1_part1_main() -> Result<(), AdventError>{
    let input = include_str!("input");

    let numbers = input.lines().map(|l| l.trim().parse::<i32>()).collect::<Result<Vec<i32>, ParseIntError>>()?;

    for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }
            if num1 + num2 == 2020 {
                println!("Result is n={} * m={} = {}", num1, num2, num1*num2);
            }
        }
    }
    Ok(())
}

pub fn day1_part2_main() -> Result<(), AdventError>{
    let input = include_str!("input");

    let numbers = input.lines().map(|l| l.trim().parse::<i32>()).collect::<Result<Vec<i32>, ParseIntError>>()?;

    for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            for (k, num3) in numbers.iter().enumerate() {
                if i == j || i == k || j == k {
                    continue;
                }
                if num1 + num2 + num3 == 2020 {
                    println!("Result is n={} * m={} * o={} = {}", num1, num2, num3, num1*num2*num3);
                }
            }
        }
    }
    Ok(())
}
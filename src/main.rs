use std::env;
use std::num::ParseIntError;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

#[derive(Debug)]
pub struct AdventError {
    pub cause: String,
}

fn main() -> Result<(), AdventError> {
    let args: Vec<String> = env::args().collect();

    match &args[1] as &str {
        "day1_part1" => day1::day1_part1_main(),
        "day1_part2" => day1::day1_part2_main(),
        "day2_part1" => day2::part1(),
        "day2_part2" => day2::part2(),
        "day3_part1" => day3::part1(),
        "day3_part2" => day3::part2(),
        "day4_part1" => day4::part1(),
        "day5_part1" => day5::part1(),
        "day5_part2" => day5::part2(),
        "day6_part1" => day6::part1(),
        "day6_part2" => day6::part2(),
        "day7_part1" => day7::part1(),
        "day7_part2" => day7::part2(),
        "day8_part1" => day8::part1(),
        "day8_part2" => day8::part2(),
        "day9_part1" => day9::part1(),
        "day9_part2" => day9::part2(),
        "day10_part1" => day10::part1(),
        "day10_part2" => day10::part2(),
        "day11_part1" => day11::part1(),
        "day11_test" => day11::test(),
        _ => {
            println!("Unknown argument {}", args[0]);
            Ok(())
        }
    }
}

impl From<ParseIntError> for AdventError {
    fn from(src: ParseIntError) -> Self {
        AdventError{cause: src.to_string()}
    }
}
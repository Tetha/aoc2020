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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

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
        "day11_part2" => day11::part2(),
        "day12_part1" => day12::part1(),
        "day12_part2" => day12::part2(),
        "day13_part1" => day13::part1(),
        "day13_part2" => day13::part2(),
        "day14_part1" => day14::part1(),
        "day14_part2" => day14::part2(),
        "day14_challenge" => day14::challenge(),
        "day15_part1" => day15::part1(),
        "day15_part2_test" => day15::part2_test(),
        "day15_part2" => day15::part2(),
        "day16_part1" => day16::part1(),
        "day16_part2" => day16::part2(),
        "day17_part1" => day17::part1(),
        "day18_part1" => day18::part1(),
        "day19_test" => day19::test(),
        "day19_part1" => day19::part1(),
        "day20_test" => day20::test(),
        "day20_part1" => day20::part1(),
        "day21_test" => day21::test(),
        "day21_part1" => day21::part1(),
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
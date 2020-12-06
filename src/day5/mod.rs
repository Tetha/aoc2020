use std::collections::HashSet;

use crate::AdventError;


enum BisectHalf {
    UPPER,
    LOWER,
}

trait BisectionPredicate {
    fn select_half(&self) -> BisectHalf;
}

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let seat_id = input.lines()
         .map(|l| parse_boarding_pass(l.trim()).unwrap())
         .map(|(r,s)| get_seat(r, s))
         .map(|(r,s)| r*8 + s)
         .max();
    println!("Biggest seat id: {:?}", seat_id);
    Ok(())
}
pub fn part2() -> Result<(), AdventError> {

    let input = include_str!("input");
    let seat_ids: HashSet<u32> = input.lines()
         .map(|l| parse_boarding_pass(l.trim()).unwrap())
         .map(|(r,s)| get_seat(r, s))
         .map(|(r,s)| r*8 + s)
         .collect();

    let min_seat_id = *seat_ids.iter().min().unwrap();
    let max_seat_id = *seat_ids.iter().max().unwrap();

    for i in min_seat_id..=max_seat_id {
        let possible_seat = i + 1;
        if seat_ids.contains(&(possible_seat - 1)) && seat_ids.contains(&(possible_seat + 1)) && !seat_ids.contains(&possible_seat) {
            println!("I sit at {}", possible_seat);
        }
    }
    Ok(())
}

fn parse_boarding_pass(input: &str) -> Result<(Vec<RowSelector>, Vec<SeatSelector>), AdventError> {
    if input.len() != 10 {
        return Err(AdventError{cause: "expected input to be of len 10".to_string()})
    }
    let row_chars: Vec<char> = input.chars().take(7).collect();
    let seat_chars: Vec<char> = input.chars().skip(7).take(3).collect();

    let rows: Vec<RowSelector> = row_chars.iter().map(|c| match c {
        'F' => Ok(RowSelector::FORWARD),
        'B' => Ok(RowSelector::BACKWARD),
        c => Err(AdventError{cause: format!("Unexpected char {}", c)}),
    }).collect::<Result<Vec<RowSelector>,AdventError>>()?;

    let cols: Vec<SeatSelector> = seat_chars.iter().map(|c| match c {
        'R' => Ok(SeatSelector::RIGHT),
        'L' => Ok(SeatSelector::LEFT),
        c => Err(AdventError{cause: format!("Unexpected char {}", c)}),
    }).collect::<Result<Vec<SeatSelector>,AdventError>>()?;

    Ok((rows, cols))
}

fn get_seat(rows: Vec<RowSelector>, seat: Vec<SeatSelector>) -> (u32, u32) {
    let row = bisect(0, 127, &rows);
    let seat = bisect(0, 7, &seat);
    (row, seat)
}

fn bisect<T>(min: u32, max: u32, bisector: &Vec<T>) -> u32 
    where T: BisectionPredicate {
    let mut lower = min;
    let mut higher = max;

    for bisector in bisector.iter() {
        let middle = (lower + higher) / 2;
        println!("low = {} <= middle = {} <= higher = {}", lower, middle, higher);
        match bisector.select_half() {
            BisectHalf::UPPER => lower = middle + 1,
            BisectHalf::LOWER => higher = middle,
        }
    }
    assert_eq!(lower, higher); // TODO: Error handling?
    lower
}

#[derive(Debug)]
enum RowSelector {
    FORWARD,
    BACKWARD,
}

impl BisectionPredicate for RowSelector {
    fn select_half(&self) -> BisectHalf {
        match self {
            RowSelector::FORWARD => BisectHalf::LOWER,
            RowSelector::BACKWARD => BisectHalf::UPPER,
        }
    }
}

#[derive(Debug)]
enum SeatSelector {
    LEFT,
    RIGHT
}

impl BisectionPredicate for SeatSelector {
    fn select_half(&self) -> BisectHalf {
        match self {
            SeatSelector::LEFT => BisectHalf::LOWER,
            SeatSelector::RIGHT => BisectHalf::UPPER,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_from_example1() {
        // FBFBBFF
        let input = vec![
            RowSelector::FORWARD,
            RowSelector::BACKWARD,
            RowSelector::FORWARD,
            RowSelector::BACKWARD,
            RowSelector::BACKWARD,
            RowSelector::FORWARD,
            RowSelector::FORWARD,
        ];
        let output = bisect(0, 127, &input);
        assert_eq!(output, 44);
    }

    #[test]
    fn test_seat_selection_from_example2() {
        // RLR
        let input = vec![
            SeatSelector::RIGHT,
            SeatSelector::LEFT,
            SeatSelector::RIGHT,
        ];
        let output = bisect(0, 7, &input);
        assert_eq!(output, 5);
    }
}
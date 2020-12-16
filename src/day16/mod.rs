use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let parsed_input = input.parse::<Input>()?;
    println!("The false positive rate is {}", parsed_input.sum_invalid_fields());
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut parsed_input = input.parse::<Input>()?;
    parsed_input.remove_invalid_fields();

    let mut field_indexes: HashMap<String, usize> = HashMap::new();
    let mut remaining_fields: Vec<&PassportField> = Vec::new();
    parsed_input.fields.iter().for_each(|f| remaining_fields.push(f));

    while remaining_fields.len() > 0 {
        for i in 0..parsed_input.my_passport.len() {
            let possible_fields = parsed_input.get_possible_fields(i, &remaining_fields);
            if possible_fields.len() == 1 {
                println!("Hard match at index {}: {:?}", i, possible_fields[0]);
                field_indexes.insert(possible_fields[0].name.to_string(), i);
                let idx = remaining_fields.iter().position(|x| x.name == possible_fields[0].name).unwrap();
                remaining_fields.remove(idx);
                break;
            }
        }
    }

    println!("{:?}", field_indexes);
    let mut result: u64 = 1;
    for (field_name, idx) in field_indexes {
        if field_name.starts_with("departure") {
            result = result * (parsed_input.my_passport[idx] as u64);
        }
    }
    println!("Result is: {}", result);
    Ok(())
}

#[derive(Debug)]
struct Input {
    fields: Vec<PassportField>,
    my_passport: Vec<u32>,
    other_passports: Vec<Vec<u32>>,
}

impl Input {
    fn can_be_valid(&self, value: u32) -> bool {
        self.fields
            .iter()
            .any(|f| f.valid_for_any_field(value))
    }
    fn sum_invalid_fields(&self) -> u32 {
        let mut invalid_passports = 0;
        for passport in &self.other_passports {
            invalid_passports += passport.iter()
                    .filter(|&&v| !self.can_be_valid(v))
                    .sum::<u32>();
        }
        return invalid_passports;
    }

    fn remove_invalid_fields(&mut self) {
        let mut new_passports: Vec<Vec<u32>> = Vec::new();
        for passport in &self.other_passports {
            if passport.iter()
                    .all(|&v| self.can_be_valid(v)) {
                new_passports.push(passport.clone());
            }
        }
        self.other_passports = new_passports;
    }

    fn get_possible_fields<'a>(&self, index: usize, fields: &Vec<&'a PassportField>) -> Vec<&'a PassportField> {
        fields
        .iter()
        .filter(|&f| self.can_field_be_in_position(index, f))
        .copied()
        .collect::<Vec<&PassportField>>()
    }

    fn can_field_be_in_position(&self, index: usize, field: &PassportField) -> bool {
        self.other_passports
            .iter()
            .map(|p| p[index] )
            .all(|v| field.valid_for_any_field(v))
    }
}

#[derive(Debug)]
struct Interval {
    min: u32,
    max: u32,
}

impl Interval {
    fn contains(&self, value: u32) -> bool {
        self.min <= value && value <= self.max
    }
}

#[derive(Debug)]
struct PassportField {
    name: String,
    valid_intervals: Vec<Interval>,
}

impl PassportField {
    fn valid_for_any_field(&self, value: u32) -> bool {
        self.valid_intervals.iter().any(|f| f.contains(value))
    }
}

impl FromStr for Interval {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split("-").collect::<Vec<&str>>();
        if chunks.len() != 2 {
            return Err(AdventError{cause: format!("expected 2 chunks in {}", s)});
        }
        let start = chunks[0].parse::<u32>()?;
        let end = chunks[1].parse::<u32>()?;
        Ok(Interval{min: start, max: end})
    }
}

impl FromStr for PassportField {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field_chunks = s.split(":").collect::<Vec<&str>>();
        if field_chunks.len() != 2 {
            return Err(AdventError{cause: format!("split at : failed in {}", s)});
        }
        let field_name = field_chunks[0];
        
        let mut intervals: Vec<Interval> = Vec::new();
        let interval_part = field_chunks[1].split("or");
        for chunk in interval_part {
            intervals.push(chunk.trim().parse::<Interval>()?);
        }
        Ok(PassportField{name: field_name.to_string(), valid_intervals: intervals})
    }
}

impl FromStr for Input {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut fields: Vec<PassportField> = Vec::new();
        for l in &mut lines {
            if l == "" {
                break
            }
            fields.push(l.parse::<PassportField>()?);
        }

        if lines.next() != Some("your ticket:") {
            return Err(AdventError{cause: "could not find expected <your ticket:>".to_string()});
        }
        let mut my_passport: Vec<u32> = Vec::new();
        for l in &mut lines {
            if l == "" {
                break;
            }
            for unparsed_num in l.split(",") {
                my_passport.push(unparsed_num.parse::<u32>()?);
            }
        }

        if lines.next() != Some("nearby tickets:") {
            return Err(AdventError{cause: "could not find expected <nearby tickets:>".to_string()});
        }
        let mut other_passports: Vec<Vec<u32>> = Vec::new();
        for l in &mut lines {
            if l == "" {
                break;
            }
            other_passports.push(
                l.split(",")
                 .map(|l| l.parse::<u32>())
                 .collect::<Result<Vec<u32>, ParseIntError>>()?
            );
        }
        Ok(Input{
            fields,
            my_passport,
            other_passports,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let parsed_input = input.parse::<Input>().unwrap();
        assert_eq!(71, parsed_input.sum_invalid_fields());
    }
}
use std::str::FromStr;
use regex::Regex;
use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut valid = 0;
    for line in input.lines() {
        let chunks: Vec<&str> = line.splitn(2, ':').collect();
        if chunks.len() != 2 {
            println!("Weird line: {}", line);
        }
        let (policy, password) = (chunks[0], chunks[1]);
        let policy = policy.parse::<PasswordPolicy>()?;
        if policy.is_valid(password) {
            valid += 1;
        }
    }
    println!("{} valid passwords", valid);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut valid = 0;
    for line in input.lines() {
        let chunks: Vec<&str> = line.splitn(2, ':').collect();
        if chunks.len() != 2 {
            println!("Weird line: {}", line);
        }
        let (policy, password) = (chunks[0], chunks[1]);
        let policy = policy.parse::<PasswordPolicy>()?;
        let first_matches = password.chars().nth((policy.min) as usize).map(|c| c == policy.letter).ok_or(AdventError{cause: "madness".to_string()})?;
        let second_matches = password.chars().nth((policy.max) as usize).map(|c| c == policy.letter).ok_or(AdventError{cause: "madness".to_string()})?;
        if (first_matches || second_matches)  && (first_matches != second_matches) {
            println!("{}", line);
            valid += 1;
        }
    }
    println!("{} valid passwords", valid);
    Ok(())
}
#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    letter: char
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        let occurences = password.chars().filter(|c| *c == self.letter).count();
        self.min <= occurences as u32 && occurences as u32 <= self.max
    }
}

impl FromStr for PasswordPolicy {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new("(?P<min>[[:digit:]]+)-(?P<max>[[:digit:]]+) (?P<letter>[[:word:]])").unwrap();
        re.captures(s).and_then(|cap| {
            let min = cap.name("min");
            let max = cap.name("max");
            let letter = cap.name("letter");

            if let (Some(min), Some(max), Some(letter)) = (min, max, letter) {
                if let (Ok(min), Ok(max)) = (min.as_str().parse::<u32>(), max.as_str().parse::<u32>()) {
                    Some(PasswordPolicy{min, max, letter: letter.as_str().chars().next().unwrap()})
                } else {
                    None
                }
            } else {
                None
            }
        }).ok_or(AdventError{cause: format!("cannot parse {}", s)})
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_policy_parse() {
        let input = "2-9 c";
        let policy = input.parse::<PasswordPolicy>().unwrap();
        assert_eq!(policy, PasswordPolicy{min: 2, max: 9, letter: 'c'})
    }

    #[test]
    fn test_policy_valid() {
        let subject = PasswordPolicy{min: 2, max: 4, letter: 'c' };
        assert_eq!(false, subject.is_valid("c"));
        assert_eq!(true, subject.is_valid("cc"));
        assert_eq!(true, subject.is_valid("ccc"));
        assert_eq!(true, subject.is_valid("cccc"));
        assert_eq!(false, subject.is_valid("ccccc"));
        assert_eq!(false, subject.is_valid("cccccc"));
        assert_eq!(true, subject.is_valid("acbc"));
        assert_eq!(false, subject.is_valid("accbccdcc"));
    }
}
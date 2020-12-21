use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    solve(input)
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input copy");
    solve(input)
}

pub fn test() -> Result<(), AdventError> {
    let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    solve(input)
}

fn solve(input: &str) -> Result<(), AdventError> {
    let parsed_input = input.parse::<Input>()?;

    let regex_string = convert_to_regex(&parsed_input.rules, 0);
    let mut full_regex = String::new();
    full_regex.push('^');
    full_regex.push_str(&regex_string);
    full_regex.push('$');

    let regex = Regex::new(&full_regex).unwrap();

    let res = parsed_input.messages.iter()
                .filter(|l| regex.is_match(l))
                .count();

    println!("Result is {}", res);
    Ok(())
}
struct Input {
    rules: Vec<ValidationRule>,
    messages: Vec<String>,
}

impl FromStr for Input {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut raw_rules: HashMap<i32, ValidationRule> = HashMap::new();
        for l in &mut lines {
            if l == "" {
                break;
            }
            let chunks = l.split(":").collect::<Vec<&str>>();
            if chunks.len() != 2 {
                println!("Invalid line {}", l);
                continue;
            }
            let id = chunks[0];
            let rule = chunks[1];

            let parsed_id = id.trim().parse::<i32>()?;
            let parsed_rule = rule.trim().parse::<ValidationRule>()?;
            raw_rules.insert(parsed_id, parsed_rule);
        }

        let mut rules: Vec<ValidationRule> = Vec::new();
        for _ in 0..raw_rules.len() {
            rules.push(ValidationRule::Invalid);
        }
        for i in 0..raw_rules.len() {
            if raw_rules.contains_key(&(i as i32)) {
                rules[i] = raw_rules[&(i as i32)].clone();
            }
        }

        let messages = lines.map(|l| l.to_string()).collect::<Vec<String>>();
        Ok(Input{ rules, messages })
    }
}
fn convert_to_regex(ruleset: &Vec<ValidationRule>, rule_idx: usize) -> String {
    let current_rule = &ruleset[rule_idx];
    match current_rule {
        ValidationRule::Invalid => panic!("rule undefined"),
        ValidationRule::Char(c) => c.to_string(),
        ValidationRule::Sequence { rules } => {
            let mut result = "(".to_string();
            result.push_str(&rules.iter()
                 .map(|sub_rule_idx| convert_to_regex(ruleset, *sub_rule_idx))
                 .collect::<String>());
            result.push_str(")");
            result
        }
        ValidationRule::Alternatives { arms } => {
            let mut result = String::new();
            result.push('(');
            for arm in arms {
                let arm_regex = arm.iter()
                                   .map(|sub_rule_idx| convert_to_regex(ruleset, *sub_rule_idx))
                                   .collect::<String>();
                if result.len() > 1 {
                    result.push('|');
                }
                result.push('(');
                result.push_str(&arm_regex); 
                result.push(')');
            }
            result.push(')');
            result
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
enum ValidationRule {
    Invalid,
    Char(char),
    Sequence{ rules: Vec<usize> },
    Alternatives{ arms: Vec<Vec<usize>> }
}

impl FromStr for ValidationRule {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 3 && s.chars().nth(0) == Some('"') && s.chars().nth(2) == Some('"') {
            return Ok(ValidationRule::Char(s.chars().nth(1).unwrap()));
        }

        if s.contains("|") {
            let mut alternatives: Vec<Vec<usize>> = Vec::new();
            for alternative in s.trim().split("|") {
                let sub_rules: Vec<usize> = alternative.trim().split(" ")
                                                      .map(|s| s.parse::<usize>())
                                                      .collect::<Result<Vec<usize>, ParseIntError>>()?;
                alternatives.push(sub_rules);
            }
            return Ok(ValidationRule::Alternatives{ arms: alternatives });
        }

        let mut rules: Vec<usize> = Vec::new();
        for part in s.trim().split(" ") {
            rules.push(part.parse::<usize>()?);
        }
        return Ok(ValidationRule::Sequence{ rules });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_single_char() {
        let input = "\"a\"";
        let parsed = input.parse::<ValidationRule>().unwrap();
        assert_eq!(ValidationRule::Char('a'), parsed);
    }

    #[test]
    fn test_alternation_parser() {
        let input = "1 3 | 3 1";
        let parsed = input.parse::<ValidationRule>().unwrap();
        assert_eq!(parsed,
            ValidationRule::Alternatives{arms: vec![
                vec![1, 3],
                vec![3, 1],
            ]}
        );
    }

    #[test]
    fn test_sequence() {
        let input = "1 2 3";
        let parsed = input.parse::<ValidationRule>().unwrap();
        assert_eq!(parsed,
            ValidationRule::Sequence{rules: vec![
                1, 2, 3
            ]});
    }

    #[test]
    fn test_example_one_regex() {
        let rules = vec![
            ValidationRule::Sequence{ rules: vec![1, 2]},
            ValidationRule::Char('a'),
            ValidationRule::Alternatives{
                arms: vec![
                    vec![1, 3],
                    vec![3, 1],
                ]
            },
            ValidationRule::Char('b'),
        ];
        let regex = convert_to_regex(&rules, 0);
        println!("{:?}", regex);
    }
}
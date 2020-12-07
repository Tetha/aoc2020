use std::collections::{HashMap, HashSet};

use crate::AdventError;

struct Group{
    size: i32,
    answers: HashMap<char, i32>,
}

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let groups = get_all_groups(&mut input.lines());

    let total_answers: usize = groups.iter().map(|g| g.answers.len()).sum();
    println!("There is a total of {} answers", total_answers);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let groups = get_all_groups(&mut input.lines());

    let total_answers: usize = groups.iter().map(|g| count_questions_answered_by_all(g)).sum();
    println!("There is a total of {} answers", total_answers);
    Ok(())
}

fn count_questions_answered_by_all(g: &Group) -> usize {
    g.answers.iter().filter(|(_, v)| **v == g.size).count()
}
fn get_all_groups<'a, T>(lines: &mut T) -> Vec<Group> 
    where T: Iterator<Item=&'a str> {

    let mut result = Vec::new();
    loop {
        let group = get_answers_for_group(lines);
        if group.answers.len() == 0 {
            return result;
        } else {
            result.push(group);
        }
    }
}
fn get_answers_for_group<'a, T>(lines: &mut T) -> Group
    where T: Iterator<Item=&'a str> {
    
    let mut result = HashMap::new();
    let mut group_size = 0;
    for line in lines {
        if line.trim().len() == 0 {
            break;
        }
        group_size += 1;
        for c in line.trim().chars() {
            let current_count = result.get(&c);
            match current_count{
                Some(previous_count) => result.insert(c, previous_count + 1),
                None => result.insert(c, 1),
            };
        }
    }
    return Group{size: group_size, answers: result};
}
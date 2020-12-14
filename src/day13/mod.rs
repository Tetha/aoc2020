use std::num::ParseIntError;

use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let start_time_line = input.lines().nth(0).unwrap();
    let bus_id_line = input.lines().nth(1).unwrap();

    let start_time = start_time_line.trim().parse::<u32>()?;

    let bus_ids = bus_id_line.trim()
        .split(",")
        .filter(|s| *s != "x")
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    let earliest_bus = find_earliest_bus(start_time, bus_ids);
    match earliest_bus {
        None => println!("Cannot find earliest bus"),
        Some((id, wait_time)) => println!("id = {} * wait_time = {} => {}", id, wait_time, id * wait_time),
    }
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let bus_id_line = input.lines().nth(1).unwrap();

    let bus_ids = bus_id_line.trim()
        .split(",")
        .enumerate()
        .filter(|&(_, line)| line != "x")
        .map(|(id, line)| (id, line.parse::<u64>()))
        .filter(|(_, result)| result.is_ok())
        .map(|(id, result)| (id, result.unwrap()))
        .collect::<Vec<(usize, u64)>>();

    let mut solved = 1;
    let mut t = bus_ids[0].0 as u64;
    let mut inc = bus_ids[0].1;
    let mut iter = 10000;
    loop {
        iter -= 1;
        if iter == 0 {
            println!("solved = {}", solved);
            iter = 1000;
            //break;
        }
        for (t_plus, bus_id) in bus_ids.iter().take(solved+1) {
            println!("checking ({} + {}) % {} = {}", t, t_plus, bus_id, (t + (*t_plus as u64)) % bus_id);
        }
        let one_more_solved = bus_ids.iter()
               .take(solved+1)
               .all(|(t_plus, bus_id)| (t + (*t_plus as u64)) % bus_id == 0 );
        if one_more_solved {
            println!("one more");
            solved += 1;
            if solved == bus_ids.len() {
                break;
            }
            inc = inc * bus_ids[solved-1].1;
        }
        t += inc;
    }
    println!("{:?}", t);
    Ok(())
}
fn find_earliest_bus(start_time: u32, bus_ids: Vec<u32>) -> Option<(u32, u32)> {
    let earliest_time = bus_ids.iter()
        .map(|id| (*id, get_earliest_multiple(start_time, *id)))
        .min_by_key(|f| f.1);
    match earliest_time {
        None => None,
        Some((id, time)) => Some((id, time - start_time)),
    }
}

fn get_earliest_multiple(minimum: u32, multiplier: u32) -> u32 {
    let div = minimum / multiplier;
    if div * multiplier == minimum {
        minimum 
    } else {
        (div + 1) * multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_earliest_multiple() {
        assert_eq!(944, get_earliest_multiple(939, 59));
        assert_eq!(945, get_earliest_multiple(939, 7));
        assert_eq!(949, get_earliest_multiple(939, 13));
    }

    #[test]
    fn test_example1() {
        let start_time = 939;
        let bus_ids = vec![7, 13, 59, 31, 19];
        assert_eq!(Some((59, 5)), find_earliest_bus(start_time, bus_ids));
    }
    
    #[test]
    fn debug_helper() {
        part2().unwrap();
    }
}
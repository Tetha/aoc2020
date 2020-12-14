use std::collections::HashSet;
use std::str::FromStr;

use defaultmap::DefaultHashMap;
use regex::Regex;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let memory = run_program(input, true)?;
    println!("Result: {}", memory.sum_values());
    Ok(())
}
pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let memory = run_program(input, false)?;
    println!("Result: {}", memory.sum_values());
    Ok(())
}
fn run_program(input: &str, is_part1: bool) -> Result<DockingMemory, AdventError> {
    let store_regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    let mask_regex = Regex::new(r"mask = (?P<mask>\w+)").unwrap();

    let mut memory = DockingMemory::new();

    for store_line in input.lines() {
        let store_caps = store_regex.captures(store_line);
        match store_caps {
            None => {}, // ignore
            Some(caps) => {
                let address = caps["address"].to_string();
                let value = caps["value"].parse::<u64>()?;
                if is_part1 {
                    memory.store(address, value);
                } else {
                    let address = address.parse::<u64>()?;
                    memory.store_v2(address, value)
                }
                continue
            }
        }

        let mask_caps = mask_regex.captures(store_line);
        match mask_caps {
            None => println!("Cannot handle {}", store_line),
            Some(caps) => {
                let mask = caps["mask"].trim()
                         .chars()
                         .rev()
                         .map(|c| String::from(c).parse::<MaskValue>())
                         .collect::<Result<Vec<MaskValue>, AdventError>>()?;
                memory.masks = Some(mask);

                let mask = caps["mask"].trim()
                         .chars()
                         .rev()
                         .map(|c| String::from(c).parse::<MaskValueV2>())
                         .collect::<Result<Vec<MaskValueV2>, AdventError>>()?;
                memory.masks_v2 = Some(mask);
            }
        }
    }
    Ok(memory)
}
struct DockingMemory {
    memory: DefaultHashMap<String, u64>,
    masks: Option<Vec<MaskValue>>,
    masks_v2: Option<Vec<MaskValueV2>>,
}

impl DockingMemory {
    fn new() -> DockingMemory {
        DockingMemory{
            memory: DefaultHashMap::new(0),
            masks: None,
            masks_v2: None,
        }
    }

    fn store(&mut self, address: String, value: u64) {
        match &self.masks {
            None => println!("warning = mask uninitialized, discarding value"),
            Some(masks) => {
                let new_value = apply_many(masks, value);
                self.memory.insert(address, new_value);
            }
        }
    }

    fn store_v2(&mut self, address: u64, value: u64) {
        match &self.masks_v2 {
            None => println!("warning = mask uninitialized, discarding value"),
            Some(masks) => {
                let mut addresses = HashSet::<u64>::new();
                for (mask_idx, mask) in masks.iter().enumerate() {
                    addresses = mask.add_addresses(address, mask_idx, &addresses);
                    //println!("{} {:?} {:?}", mask_idx, mask, addresses);
                }

                //println!("address {} -> (mask = {:?}) -> {:?}", address, self.masks_v2, addresses);

                for address in addresses {
                    self.memory.insert(address.to_string(), value);
                }
            }
        }
    }

    fn sum_values(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[derive(Debug)]
enum MaskValue {
    PassThrough,
    SetToOne,
    SetToZero,
}

#[derive(Debug)]
enum MaskValueV2 {
    PassThrough,
    SetToOne,
    Floating
}

impl MaskValueV2 {
    fn add_addresses(&self, address: u64, index: usize, addresses_so_far: &HashSet<u64>) -> HashSet<u64> {
        let mut result = HashSet::new();
        match self {
            MaskValueV2::PassThrough => {
                let mask_value = address & (1 << index);
                if index == 0 {
                    if mask_value == 0 {
                        result.insert(0);
                    } else {
                        result.insert(1);
                    }
                } else {
                    if mask_value > 0 {
                        for address in addresses_so_far {
                            result.insert(2*address + 1);
                        }
                    } else {
                        for address in addresses_so_far {
                            result.insert(2*address);
                        }
                    }
                }
            }
            MaskValueV2::SetToOne => {
                if index == 0 {
                    result.insert(1);
                } else {
                    for address in addresses_so_far {
                        result.insert(address * 2 + 1);
                    }
                }
            }
            MaskValueV2::Floating => {
                if index == 0 {
                    result.insert(0);
                    result.insert(1);
                } else {
                    for address in addresses_so_far {
                        result.insert(address * 2 + 1);
                        result.insert(address * 2);
                    }
                }
            }
        }
        return result;
    }
}
fn apply_many(masks: &Vec<MaskValue>, value: u64) -> u64 {
    let mut result = value;
    for (mask_idx, mask) in masks.iter().enumerate() {
        result = mask.apply_at(result, mask_idx);
    }
    result
}
impl MaskValue {
    fn apply_at(&self, value: u64, index: usize) -> u64 {
        match self {
            MaskValue::PassThrough => value,
            MaskValue::SetToOne => value | (1 << index),
            MaskValue::SetToZero => value & !(1 << index),
        }
    }
}

impl FromStr for MaskValue {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(AdventError{cause: format!("expected length 1 {}", s)});
        }
        match s.chars().nth(0) {
            Some('X') => return Ok(MaskValue::PassThrough),
            Some('1') => return Ok(MaskValue::SetToOne),
            Some('0') => return Ok(MaskValue::SetToZero),
            _ => return Err(AdventError{cause: format!("cannot figure out <{}>", s)}),
        }
    }
}

impl FromStr for MaskValueV2 {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(AdventError{cause: format!("expected length 1 {}", s)});
        }
        match s.chars().nth(0) {
            Some('X') => return Ok(MaskValueV2::Floating),
            Some('1') => return Ok(MaskValueV2::SetToOne),
            Some('0') => return Ok(MaskValueV2::PassThrough),
            _ => return Err(AdventError{cause: format!("cannot figure out <{}>", s)}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_to_one() {
        assert_eq!(8, MaskValue::SetToOne.apply_at(0, 3));
    }

    #[test]
    fn test_set_to_zero() {
        assert_eq!(3, MaskValue::SetToZero.apply_at(7, 2));
    }

    #[test]
    fn test_example_one() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let memory = run_program(input, true).unwrap();
        assert_eq!(165, memory.sum_values());
    }

    #[test]
    fn test_example_two() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let memory = run_program(input, false).unwrap();
        assert_eq!(208, memory.sum_values());
    }
}
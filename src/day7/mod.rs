use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    println!("lines");
    let bag_specs = parse_many_lines(input);
    println!("reverse");
    let reverse_edges = reverse_edges(&bag_specs);
    println!("reachable");
    let reacheable_nodes = count_reachable_nodes("shiny gold", &reverse_edges);
    println!("Could reach {} bags", reacheable_nodes - 1);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let bag_specs = parse_many_lines(input);
    let contained_bags = count_contained_bags("shiny gold", &bag_specs);
    println!("Contains {}", contained_bags - 1);
    Ok(())
}

#[derive(Debug)]
struct BagSpec {
    color: String,
    inner_bags: HashMap<String, u32>,
}

fn count_contained_bags(current: &str, bag_specs: &HashMap<String, BagSpec>) -> u32 {
    let current_spec = &bag_specs[current];
    println!("Current: {:?}", current_spec);
    current_spec.inner_bags.iter().map(|(color, num)| num * count_contained_bags(color, bag_specs)).sum::<u32>() + 1
}

fn count_reachable_nodes(start: &str, edges: &HashMap<String, Vec<String>>) -> usize {
    let mut marked: HashSet<String> = HashSet::new();
    mark_reacheable_nodes(start, edges, &mut marked);
    marked.len()
}

fn mark_reacheable_nodes(node: &str, edges: &HashMap<String, Vec<String>>, marked: &mut HashSet<String>) {
    marked.insert(node.to_string());

    for edge in &edges[node] {
        if !marked.contains(edge) {
            mark_reacheable_nodes(edge, edges, marked);
        }
    }
}

fn reverse_edges(input: &HashMap<String, BagSpec>) -> HashMap<String, Vec<String>> {
    let mut result:HashMap<String, Vec<String>> = HashMap::new();
    for k in input.keys() {
        result.insert(k.clone(), Vec::new());
    }

    for bagspec in input.values() {
        for inner in bagspec.inner_bags.keys() {
            result.get_mut(inner).unwrap().push(bagspec.color.clone());
        }
    }    
    return result;
}

fn parse_many_lines(input: &str) -> HashMap<String, BagSpec> {
    let header = Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<other_bags>.*)\.$").unwrap();
    let inner_bag = Regex::new(r"(?P<num>\d+) (?P<color>\w+ \w+) bags?").unwrap();
    input.lines()
         .map(|l| parse_input(l, &header, &inner_bag))
         .filter(|ob| ob.is_some())
         .map(|ob| ob.unwrap())
         .map(|b| (b.color.clone(), b))
         .collect::<HashMap<String, BagSpec>>()
}

fn parse_input(input: &str, header: &Regex, inner_bag: &Regex) -> Option<BagSpec> {
    let caps = header.captures(input)?;

    let mut result = BagSpec{ color: caps["color"].to_string(), inner_bags: HashMap::new()};
    if &caps["other_bags"] == "no other bags" {
        return Some(result);
    }

    for other_bag in caps["other_bags"].split(",") {
        let inner_caps = inner_bag.captures(other_bag)?;
        let num = inner_caps["num"].parse::<u32>().unwrap(); // safe unwrap, see regex
        result.inner_bags.insert(inner_caps["color"].to_string(), num);
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bag_with_subbags() {
        let header = Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<other_bags>.*)\.$").unwrap();
        let inner_bag = Regex::new(r"(?P<num>\d+) (?P<color>\w+ \w+) bags?").unwrap();
        let line = parse_input("light red bags contain 1 bright white bag, 2 muted yellow bags.", &header, &inner_bag).unwrap();
        assert_eq!("light red", line.color);
        assert_eq!(1, line.inner_bags["bright white"]);
        assert_eq!(2, line.inner_bags["muted yellow"]);
    }

    #[test]
    fn test_parse_bag_without_subbags() {
        let header = Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<other_bags>.*)\.$").unwrap();
        let inner_bag = Regex::new(r"(?P<num>\d+) (?P<color>\w+ \w+) bags?").unwrap();
        let line = parse_input("dotted black bags contain no other bags.", &header, &inner_bag).unwrap();
        assert_eq!("dotted black", line.color);
        assert_eq!(0, line.inner_bags.len());
    }
}
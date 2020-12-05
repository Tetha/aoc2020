use std::collections::HashSet;
use std::str::FromStr;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let forest = input.parse::<Forest>()?;
    let trees = forest.count_trees(3, 1);
    println!("Found {} trees", trees);

    Ok(())
}
pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let forest = input.parse::<Forest>()?;
    let trees = forest.count_trees(1, 1)
                   * forest.count_trees(3, 1)
                   * forest.count_trees(5, 1)
                   * forest.count_trees(7, 1)
                   * forest.count_trees(1, 2);
    println!("Found {} trees", trees);

    Ok(())
}

#[derive(Debug)]
struct Forest {
    trees: HashSet<(u32, u32)>,
    width: u32,
    height: u32,
}

impl FromStr for Forest {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result  = Forest{trees: HashSet::new(), width: 0, height: 0};
        for (y, line) in s.lines().enumerate() {
            result.width = line.trim().chars().count() as u32;
            result.height = y as u32;
            for (x, c) in line.chars().enumerate() {
                if c == '#'  {
                    result.trees.insert((x as u32, y as u32));
                }
            }
        }
        return Ok(result);
    }
}

impl Forest {
    fn has_tree(&self, x: u32, y: u32) -> bool {
        let x_wrapped = x % self.width;
        return self.trees.contains(&(x_wrapped, y));
    }

    fn count_trees(&self, dx: u32, dy: u32) -> u32 {
        let mut current_x = 0;
        let mut current_y = 0;

        let mut trees = 0;
        while current_y <= self.height {
            if self.has_tree(current_x, current_y) {
                trees += 1;
            }
            current_x += dx;
            current_y += dy;
        }
        return trees;
    }
}
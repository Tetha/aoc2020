use std::alloc::System;
use std::collections::VecDeque;
use std::time::SystemTime;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let mut game = Game{
        round: 0,
        numbers: NumberRing::new(vec![3, 2, 6, 5, 1, 9, 4, 7, 8]),
        max_number: 9,
        current_number: 3,
    };

    println!("The result es <{}>", solve(&mut game));
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let mut numbers = vec![3, 2, 6, 5, 1, 9, 4, 7, 8];
    for i in 10..=1000000 {
        numbers.push(i);
    }
    assert_eq!(1000000, numbers.len());
    let mut game = Game{
        round: 0,
        numbers: NumberRing::new(numbers),
        max_number: 1000000,
        current_number: 3,
    };

    println!("The result es <{:?}>", solve_p2(&mut game));
    Ok(())
}

pub fn test() -> Result<(), AdventError> {
    let mut subject = Game{
        round: 0,
        numbers: NumberRing::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]),
        max_number: 9,
        current_number: 3,
    };
    println!("The result is <{}>", solve(&mut subject));
    Ok(())
}

fn solve<T> (game: &mut Game<T>) -> String 
    where T: NumberStorage {
    for _ in 1..=100 {
        game.step();
    }
    game.build_output()
}
fn solve_p2<T> (game: &mut Game<T>) -> Vec<u32> 
    where T: NumberStorage {
    for step in 1..=10000000 {
        if step % 1000000 == 0 {
            println!(".");
        }
        let start = SystemTime::now();
        game.step();
        let end = SystemTime::now();
        //println!("{}: {:?}", step, end.duration_since(start).unwrap());
    }
    game.numbers.rotate_until(1);
    game.numbers.remove_three()
}
struct Game<T> where T: NumberStorage {
    round: u32,
    numbers: T,
    max_number: u32,
    current_number: u32,
}

impl<T> Game<T> where T: NumberStorage {
    fn dump_state(&self) {
        print!("cups: ");
        for number in self.numbers.numbers() {
            if number == self.current_number {
                print!("({}) ", number);
            } else {
                print!("{} ", number);
            }
        }
        println!("");
    }

    fn build_output(&mut self) -> String {
        self.numbers.rotate_until(1);
        itertools::join(self.numbers.numbers().iter()
                            .skip(1)
                            .map(|u| u.to_string()), "")
                            
    }

    fn step(&mut self) {
        self.round += 1;
        //println!("--- move {} ---", self.round);
        //self.dump_state();
        //let time_pick_numbers = SystemTime::now();
        self.numbers.rotate_until(self.current_number);
        self.numbers.rotate_once();
        let taken_numbers = self.numbers.remove_three();
        let possible_next_target = self.numbers.current();
        //println!("pick up: {}", itertools::join(taken_numbers.iter(),  ", "));

        //let time_select_target = SystemTime::now();
        let mut target_label = if self.current_number == 1 {
            self.max_number
        } else {
            self.current_number - 1
        };
        while taken_numbers.contains(&target_label) {
            target_label -= 1;
            if target_label == 0 {
                target_label = self.max_number;
            }
        }
        //let time_insert = SystemTime::now();
        //println!("destination: {}", target_label);
        self.numbers.rotate_back_until(target_label);
        self.numbers.rotate_once();
        self.numbers.insert_many_at_end(&taken_numbers);

        let time_next_target = SystemTime::now();
        /* 
        self.numbers.rotate_until(self.current_number);
        self.numbers.rotate_once();
        self.current_number = *self.numbers.numbers.front().unwrap();
        assert_eq!(self.current_number, possible_next_target);
        */
        self.current_number = possible_next_target;
        let time_final = SystemTime::now();

        //println!("Time spent: extracting={:?}, computing={:?}, inserting={:?}, next_current={:?}",
        //    time_select_target.duration_since(time_pick_numbers).unwrap(),
        //    time_insert.duration_since(time_select_target).unwrap(),
        //    time_next_target.duration_since(time_insert).unwrap(),
        //    time_final.duration_since(time_next_target).unwrap(),
        //);
        //println!();
    }
}
struct NumberRing {
    numbers: VecDeque<u32>,
}

trait NumberStorage {
    fn rotate_until(&mut self, target_head: u32);
    fn rotate_once(&mut self);

    fn rotate_back_until(&mut self, target_head: u32);
    fn rotate_back_once(&mut self);

    fn remove_three(&mut self) -> Vec<u32>;
    fn insert_many_at_end(&mut self, elements: &Vec<u32>);

    fn numbers(&self) -> Vec<u32>;

    fn current(&self) -> u32;
}

impl NumberRing {
    fn new(numbers: Vec<u32>) -> NumberRing {
        return NumberRing{numbers: VecDeque::from(numbers)}
    }
}

impl NumberStorage for NumberRing {
    fn current(&self) -> u32 {
        *self.numbers.front().unwrap()
    }

    fn numbers(&self) -> Vec<u32> {
        return self.numbers.iter().copied().collect::<Vec<u32>>();
    }

    fn rotate_until(&mut self, target_head: u32) {
        loop {
            match self.numbers.front() {
                None => return,
                Some(x) if *x == target_head => return,
                Some(_) => self.rotate_once(),
            }
        }
    }
    fn rotate_back_until(&mut self, target_head: u32) {
        loop {
            match self.numbers.front() {
                None => return,
                Some(x) if *x == target_head => return,
                Some(_) => self.rotate_back_once(),
            }
        }
    }

    fn rotate_once(&mut self) {
        let current_head = self.numbers.pop_front().unwrap();
        self.numbers.push_back(current_head);
    }

    fn rotate_back_once(&mut self) {
        let current_head = self.numbers.pop_back().unwrap();
        self.numbers.push_front(current_head);
    }

    fn remove_three(&mut self) -> Vec<u32> {
        return vec![
            self.numbers.pop_front().unwrap(),
            self.numbers.pop_front().unwrap(),
            self.numbers.pop_front().unwrap(),
        ];
    }

    fn insert_many_at_end(&mut self, elements: &Vec<u32>) {
        self.numbers.extend(elements);
    }
}
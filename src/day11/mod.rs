use std::fmt::Display;
use std::str::FromStr;

use crate::AdventError;

pub fn test() -> Result<(), AdventError> {
    let input = include_str!("input_test");
    let mut state = input.parse::<FerryState>()?;

    loop {
        println!("{}", state);
        println!("-----------");
        let next_state = state.do_step(true);
        if next_state == state {
            break;
        } else {
            state = next_state;
        }
    }
    println!("{}", state.count_occupied_seats());
    Ok(())
}

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut state = input.parse::<FerryState>()?;

    loop {
        let next_state = state.do_step(false);
        if next_state == state {
            break;
        } else {
            state = next_state;
        }
    }
    println!("{}", state.count_occupied_seats());
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut state = input.parse::<FerryState>()?;

    loop {
        let next_state = state.do_step(true);
        if next_state == state {
            break;
        } else {
            state = next_state;
        }
    }
    println!("{}", state.count_occupied_seats());
    Ok(())
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum FerrySpot {
    Floor,
    EmptySeat,
    FullSeat,
}

#[derive(Debug, PartialEq)]
struct FerryState {
    width: usize,
    height: usize,
    cells: Vec<FerrySpot>,
}

impl Display for FerryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            if i % self.width == 0 && i > 0 {
                f.write_str("\n")?;
            }
            match cell {
                FerrySpot::Floor => f.write_str(".")?,
                FerrySpot::EmptySeat => f.write_str("L")?,
                FerrySpot::FullSeat => f.write_str("#")?,
            }
        }
        Ok(())
    }
}

impl FerryState {
    fn new(width: usize, height: usize) -> FerryState {
        let mut cells = Vec::new();
        for _ in 0..=width * height {
            cells.push(FerrySpot::Floor);
        }
        return FerryState{width, height, cells}
    }

    fn count_occupied_seats(&self) -> usize {
        self.cells.iter().filter(|s| s == &&FerrySpot::FullSeat).count()
    }
    fn set_cell(&mut self, x: usize, y: usize, cell: FerrySpot) {
        self.cells[y * self.width + x] = cell;
    }

    fn get_cell(&self, x: usize, y: usize) -> FerrySpot {
        return self.cells[y * self.width + x];
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<FerrySpot> {
        let mut result = Vec::new();
        if 1 <= x {
            if 1 <= y {
                result.push(self.get_cell(x-1, y-1));
            }
            result.push(self.get_cell(x-1, y));
            if y + 1 < self.height {
                result.push(self.get_cell(x - 1, y+1));
            }
        }


        if 1 <= y {
            result.push(self.get_cell(x, y-1));
        }

        if y + 1 < self.height {
            result.push(self.get_cell(x, y+1));
        }

        if x + 1 < self.width {
            if 1 <= y {
                result.push(self.get_cell(x+1, y-1));
            }
            result.push(self.get_cell(x+1, y));
            if y + 1 < self.height {
                result.push(self.get_cell(x+1, y+1));
            }
        }

        
        return result;
    }

    fn get_visible_neighbours(&self, x: usize, y: usize) -> Vec<FerrySpot> {
        let mut result = Vec::new();
        self.get_first_seat_cell(x, y, -1, -1)
            .map(|spot| result.push(spot));
        self.get_first_seat_cell(x, y, -1, 0)
            .map(|spot| result.push(spot));
        self.get_first_seat_cell(x, y, -1, 1)
            .map(|spot| result.push(spot));


        self.get_first_seat_cell(x, y, 0, -1)
            .map(|spot| result.push(spot));
        self.get_first_seat_cell(x, y, 0, 1)
            .map(|spot| result.push(spot));


        self.get_first_seat_cell(x, y, 1, -1)
            .map(|spot| result.push(spot));
        self.get_first_seat_cell(x, y, 1, 0)
            .map(|spot| result.push(spot));
        self.get_first_seat_cell(x, y, 1, 1)
            .map(|spot| result.push(spot));
        
        //println!("{}/{} has neighbours {:?}", x, y, result);
        return result;
    }
    
    fn get_first_seat_cell(&self, start_x: usize, start_y: usize, dx: i32, dy: i32) -> Option<FerrySpot> {
        let mut current_x = start_x as i32;
        let mut current_y = start_y as i32;

        loop {
            current_x += dx;
            current_y += dy;

            if !(0 <= current_x && (current_x as usize) < self.width) {
                return None;
            }

            if !(0 <= current_y && (current_y as usize) < self.height) {
                return None;
            }

            let cell = self.get_cell(current_x as usize, current_y as usize);

            match cell {
                FerrySpot::FullSeat | FerrySpot::EmptySeat => return Some(cell),
                FerrySpot::Floor => continue
            }
        }
    }

    fn do_step(&self, is_part2: bool) -> FerryState {
        let mut next_state =FerryState::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.width {
                match self.get_cell(x, y) {
                    FerrySpot::Floor => continue,
                    FerrySpot::EmptySeat => {
                        let neighbours = if is_part2 {
                            self.get_visible_neighbours(x, y)
                        } else {
                            self.get_neighbours(x, y)
                        };
                        if neighbours
                            .iter()
                            .filter(|n| n == &&FerrySpot::FullSeat)
                            .count() == 0 {
                            next_state.set_cell(x, y, FerrySpot::FullSeat);
                        } else {
                            next_state.set_cell(x, y, FerrySpot::EmptySeat);
                        }
                    }
                    FerrySpot::FullSeat => {
                        let neighbours = if is_part2 {
                            self.get_visible_neighbours(x, y)
                        } else {
                            self.get_neighbours(x, y)
                        };
                        let limit = if is_part2 { 5 } else { 4 };
                        if neighbours
                            .iter()
                            .filter(|n| n == &&FerrySpot::FullSeat)
                            .count() >= limit {
                            next_state.set_cell(x, y, FerrySpot::EmptySeat);
                        } else {
                            next_state.set_cell(x, y, FerrySpot::FullSeat);
                        }
                    }
                }
            }
        }
        return next_state;
    }
}

impl FromStr for FerryState {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut cells: Vec<FerrySpot> = Vec::new();
        
        for (y, line) in s.lines().enumerate() {
            width = line.len();
            height = y;
            for c in line.chars() {
                match c {
                    'L' => cells.push(FerrySpot::EmptySeat),
                    '.' => cells.push(FerrySpot::Floor),
                    '#' => cells.push(FerrySpot::FullSeat),
                    other => return Err(AdventError{cause: format!("unexpected char: {}", other)}),
                }
            }
        }
        println!("cells: {} / {} / {}", width, height, cells.len());
        Ok(FerryState{width, height: height + (1 as usize), cells})
    }
}
use std::str::FromStr;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let actions: Vec<Action> = input.lines()
                                    .map(|l| l.parse::<Action>() )
                                    .collect::<Result<Vec<Action>, AdventError>>()?;

    let mut position = ShipPosition::new();
    for action in actions {
        position.act(action);
    }
    println!("{} + {} = {}", position.x, position.y, position.x.abs() + position.y.abs());
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let actions: Vec<Action> = input.lines()
                                    .map(|l| l.parse::<Action>() )
                                    .collect::<Result<Vec<Action>, AdventError>>()?;
    let mut position = ShipPosition::new();
    for action in actions {
        position.act_waypoint(action);
    }
    println!("{} + {} = {}", position.x, position.y, position.x.abs() + position.y.abs());
    Ok(())
}
enum Action {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.chars().nth(0).ok_or(AdventError{cause: "missing command".to_string()})?;
        let parameter_unparsed: String = s.chars().skip(1).collect();
        let parameter = parameter_unparsed.parse::<i32>()?;

        match command {
            'F' => Ok(Action::Forward(parameter)),
            'N' => Ok(Action::North(parameter)),
            'S' => Ok(Action::South(parameter)),
            'E' => Ok(Action::East(parameter)),
            'W' => Ok(Action::West(parameter)),
            'L' => Ok(Action::Left(parameter)),
            'R' => Ok(Action::Right(parameter)),
            c => Err(AdventError{cause: format!("unknown command {}", c)}),
        }
    }
}
#[derive(Debug)]
enum Heading {
    North, East, South, West
}

#[derive(Debug)]
struct ShipPosition {
    x: i32,
    y: i32,

    waypoint_x: i32,
    waypoint_y: i32,
    heading: Heading,
}

impl ShipPosition {
    fn new() -> ShipPosition {
        ShipPosition{x: 0, y: 0, waypoint_x: 10, waypoint_y: -1, heading: Heading::East}
    }

    fn act_waypoint(&mut self, action: Action) {
        match action {
            Action::North(p) => self.waypoint_y -= p,
            Action::South(p) => self.waypoint_y += p,
            Action::West(p) => self.waypoint_x -= p,
            Action::East(p) => self.waypoint_x += p,
            Action::Left(p) => {
                assert_eq!(true, p == 90 || p == 180 || p == 270 || p == 360);
                // https://en.wikipedia.org/wiki/Rotation_matrix#Common_rotations
                let old_wp_x = self.waypoint_x;
                let old_wp_y = self.waypoint_y;
                if p == 90 {
                    self.waypoint_x = old_wp_y;
                    self.waypoint_y = - old_wp_x;
                }
                if p == 180 {
                    self.waypoint_x = - old_wp_x;
                    self.waypoint_y = - old_wp_y;
                }
                if p == 270 {
                    self.waypoint_x = - old_wp_y;
                    self.waypoint_y = old_wp_x;
                }
            }
            Action::Right(p) => {
                assert_eq!(true, p == 90 || p == 180 || p == 270 || p == 360);
                let old_wp_x = self.waypoint_x;
                let old_wp_y = self.waypoint_y;
                if p == 90 {
                    self.waypoint_x = - old_wp_y;
                    self.waypoint_y = old_wp_x;
                }
                if p == 180 {
                    self.waypoint_x = - old_wp_x;
                    self.waypoint_y = - old_wp_y;
                }
                if p == 270 {
                    self.waypoint_x = old_wp_y;
                    self.waypoint_y = - old_wp_x;
                }
            }
            Action::Forward(p) => {
                self.x += self.waypoint_x * p;
                self.y += self.waypoint_y * p;
            }
        }
        //println!("{:?}", self);
    }
    
    fn act(&mut self, action: Action) {
        match action {
            Action::North(p) => self.y -= p,
            Action::South(p) => self.y += p,
            Action::West(p) => self.x -= p,
            Action::East(p) => self.x += p,
            Action::Left(p) => {
                assert_eq!(true, p == 90 || p == 180 || p == 270 || p == 360);
                let mut remaining_turn = p;
                while remaining_turn > 0 {
                    self.heading = self.heading.turn_left();
                    remaining_turn -= 90;
                }
            }
            Action::Right(p) => {
                assert_eq!(true, p == 90 || p == 180 || p == 270 || p == 360);
                let mut remaining_turn = p;
                while remaining_turn > 0 {
                    self.heading = self.heading.turn_right();
                    remaining_turn -= 90;
                }
            }
            Action::Forward(p) => {
                match self.heading {
                    Heading::North => self.y -= p,
                    Heading::East => self.x += p,
                    Heading::South => self.y += p,
                    Heading::West => self.x -= p,
                }
            }
        }
    }
}

impl Heading {
    fn turn_left(&self) -> Heading {
        match self {
            Heading::North => Heading::West,
            Heading::West => Heading::South,
            Heading::South => Heading::East,
            Heading::East => Heading::North,
        }
    }

    fn turn_right(&self) -> Heading {
        match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }
}
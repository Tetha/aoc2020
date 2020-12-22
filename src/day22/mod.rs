use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use itertools::join;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut subject = input.parse::<GameState>()?;
    subject.iterate_until_won();
    println!("The winner has {} points", subject.compute_winner_score());
    Ok(())
}

pub fn test() -> Result<(), AdventError> {
    let input = include_str!("example");
    let subject = input.parse::<GameState>()?;
    let mut real_subject = RecursiveGameState{
        game_id: 1,
        round: 1,
        played_states: HashSet::new(),
        current_state: subject,
        winner: None,
    };
    real_subject.iterate();
    println!("The winner has {} points", real_subject.current_state.compute_winner_score());
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let subject = input.parse::<GameState>()?;
    let mut real_subject = RecursiveGameState{
        game_id: 1,
        round: 1,
        played_states: HashSet::new(),
        current_state: subject,
        winner: None,
    };
    real_subject.iterate();
    println!("The winner has {} points", real_subject.current_state.compute_winner_score());
    Ok(())
}
#[derive(Debug)]
struct RecursiveGameState {
    game_id: u32,
    round: u32,

    played_states: HashSet<GameState>,
    current_state: GameState,
    winner: Option<usize>,
}

impl RecursiveGameState {
    fn iterate(&mut self) {
        while self.winner.is_none() {
            self.step();
            self.round += 1;
        }
    }

    fn step(&mut self) {
        if self.played_states.contains(&self.current_state) {
            // *game* ends
            self.winner = Some(1);
            return;
        } else {
            self.played_states.insert(self.current_state.clone());
        }

        if self.current_state.deck_one.is_empty() {
            self.winner = Some(2);
            return;
        }

        if self.current_state.deck_two.is_empty() {
            self.winner = Some(1);
            return;
        }

        //println!("-- Round {} (Game {}) --", self.round, self.game_id);
        //println!("Player 1 deck: {}", join(self.current_state.deck_one.iter(), ", "));
        //println!("Player 2 deck: {}", join(self.current_state.deck_two.iter(), ", "));

        let card_player_one = self.current_state.deck_one.pop_front().unwrap();
        let card_player_two = self.current_state.deck_two.pop_front().unwrap();

        //println!("Player 1 plays: {}", card_player_one);
        //println!("Player 2 plays: {}", card_player_two);

        if !((card_player_one as usize) <= self.current_state.deck_one.len()
             && (card_player_two as usize) <= self.current_state.deck_two.len()) {
            // no more recursion, *round* ends
            if card_player_one > card_player_two {
                //println!("Player {} wins round {} of game {}!", 1, self.round, self.game_id);
                //println!();
                self.current_state.deck_one.push_back(card_player_one);
                self.current_state.deck_one.push_back(card_player_two);
            } else {
                //println!("Player {} wins round {} of game {}!", 2, self.round, self.game_id);
                //println!();
                self.current_state.deck_two.push_back(card_player_two);
                self.current_state.deck_two.push_back(card_player_one);
            }
        } else {
            //println!("Playing a sub-game to determine the winner...");
            // recursive combat
            let sub_deck_one = self.current_state.deck_one.iter().copied().take(card_player_one as usize).collect::<Vec<u32>>();
            let sub_deck_two = self.current_state.deck_two.iter().copied().take(card_player_two as usize).collect::<Vec<u32>>();

            let mut sub_game = RecursiveGameState{
                game_id: self.game_id + 1,
                round: 1,
                played_states: HashSet::new(),
                current_state: GameState::new(sub_deck_one, sub_deck_two),
                winner: None,
            };
            sub_game.iterate();
            //println!("...anyway, back to previous game.");
            match sub_game.winner {
                Some(1) => {
                    //println!("Player {} wins round {} of game {}!", 1, self.round, self.game_id);
                    //println!();
                    self.current_state.deck_one.push_back(card_player_one);
                    self.current_state.deck_one.push_back(card_player_two);
                },
                Some(2) => {
                    //println!("Player {} wins round {} of game {}!", 2, self.round, self.game_id);
                    //println!();
                    self.current_state.deck_two.push_back(card_player_two);
                    self.current_state.deck_two.push_back(card_player_one);
                },
                _ => panic!(format!("don't understand sub game result: {:?}", sub_game)),
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct GameState {
    deck_one: VecDeque<u32>,
    deck_two: VecDeque<u32>,
}

impl FromStr for GameState {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();

        let mut result = GameState{
            deck_one: VecDeque::new(),
            deck_two: VecDeque::new(),
        };

        for l in &mut lines {
            if l == "" {
                break;
            }
            result.deck_one.push_back(l.parse::<u32>()?);
        }

        lines.next();
        for l in &mut lines {
            if l == "" {
                break;
            }
            result.deck_two.push_back(l.parse::<u32>()?);
        }
        Ok(result)
    }
}

impl GameState {
    fn new(deck_one: Vec<u32>, deck_two: Vec<u32>) -> GameState {
        GameState{
            deck_one: VecDeque::from(deck_one),
            deck_two: VecDeque::from(deck_two),
        }
    }

    fn step(&mut self) {
        if self.deck_one.is_empty() || self.deck_two.is_empty() {
            return; // Game Over
        }
        
        let card_one = self.deck_one.pop_front().unwrap();
        let card_two = self.deck_two.pop_front().unwrap();

        if card_one > card_two {
            self.deck_one.push_back(card_one);
            self.deck_one.push_back(card_two);
        } else {
            self.deck_two.push_back(card_two);
            self.deck_two.push_back(card_one);
        }
    }

    fn iterate_until_won(&mut self) {
        while !self.deck_one.is_empty() && !self.deck_two.is_empty() {
            self.step();
        }
    }

    fn compute_winner_score(&self) -> u32 {
        let winning_deck = if self.deck_one.is_empty() {
            &self.deck_two
        } else {
            &self.deck_one
        };

        let mut score = 0;
        for (i, card) in winning_deck.iter().rev().enumerate() {
            score += ((i as u32) + 1) * *card;
        }
        return score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let deck_one = vec![2, 3, 4];
        let deck_two = vec![1, 2, 3];

        let mut subject = GameState::new(deck_one, deck_two);
        subject.step();

        assert_eq!(
            subject.deck_one.iter().copied().collect::<Vec<u32>>(),
            vec![3, 4, 2, 1],
        );
        assert_eq!(
            subject.deck_two.iter().copied().collect::<Vec<u32>>(),
            vec![2, 3],
        );
    }

    #[test]
    fn test_iterate() {
        let deck_one = vec![9, 2, 6, 3, 1];
        let deck_two = vec![5, 8, 4, 7, 10];

        let mut subject = GameState::new(deck_one, deck_two);
        subject.iterate_until_won();

        assert_eq!(
            subject.deck_one.iter().copied().collect::<Vec<u32>>(),
            vec![]
        );
        assert_eq!(
            subject.deck_two.iter().copied().collect::<Vec<u32>>(),
            vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]
        );
        assert_eq!(subject.compute_winner_score(), 306);
    }

    #[test]
    fn test_from_str() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let subject = input.parse::<GameState>().unwrap();

        assert_eq!(
            subject.deck_one.iter().copied().collect::<Vec<u32>>(),
            vec![9, 2, 6, 3, 1]
        );
        assert_eq!(
            subject.deck_two.iter().copied().collect::<Vec<u32>>(),
            vec![5, 8, 4, 7, 10]
        );
    }
}
use super::Solver;
use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
pub struct DayTwentyTwoSolver {
    p1_cards: VecDeque<usize>,
    p2_cards: VecDeque<usize>,
}

impl DayTwentyTwoSolver {
    fn new() -> DayTwentyTwoSolver {
        DayTwentyTwoSolver {
            p1_cards: VecDeque::new(),
            p2_cards: VecDeque::new(),
        }
    }

    fn print_state(&self) {
        // println!("Player 1's deck: {:?}", self.p1_cards);
        // println!("Player 2's deck: {:?}", self.p2_cards);
        // println!("{}", self.state_to_string());
    }

    fn play_game(&mut self) {
        let mut seen_states = HashSet::new();
        loop {
            self.print_state();
            // check for infinite loop
            let cur_state = self.state_to_string();
            if seen_states.contains(&cur_state) {
                println!("INFINITE LOOP: Player 1 wins!");
                break;
            }
            seen_states.insert(self.state_to_string());

            // Get current cards.  If either player has none left, the other wins
            let p1_card = match self.p1_cards.pop_front() {
                None => {
                    println!("Player 2 wins!");
                    break;
                }
                Some(card) => card,
            };
            let p2_card = match self.p2_cards.pop_front() {
                None => {
                    println!("Player 1 wins!");
                    break;
                }
                Some(card) => card,
            };

            if p1_card > p2_card {
                self.p1_cards.push_back(p1_card);
                self.p1_cards.push_back(p2_card);
            } else {
                self.p2_cards.push_back(p2_card);
                self.p2_cards.push_back(p1_card);
            }
        }
    }

    fn state_to_string(&self) -> String {
        let mut state_str = String::from("");
        let p1_state = self
            .p1_cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let p2_state = self
            .p2_cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<String>>()
            .join(",");

        format!("P1: {} | P2: {}", p1_state, p2_state)
    }

    fn create_sub_game(&self, p1_card_len: usize, p2_card_len: usize) -> DayTwentyTwoSolver {
        let mut sub_game = DayTwentyTwoSolver::new();
        for (index, card) in self.p1_cards.iter().enumerate() {
            if index >= p1_card_len {
                break;
            }
            sub_game.p1_cards.push_back(*card);
        }
        for (index, card) in self.p2_cards.iter().enumerate() {
            if index >= p2_card_len {
                break;
            }
            sub_game.p2_cards.push_back(*card);
        }

        sub_game
    }

    fn play_recursive_game(&mut self, depth: usize) {
        let mut seen_states = HashSet::new();
        loop {
            self.print_state();
            // check for infinite loop
            let cur_state = self.state_to_string();
            if seen_states.contains(&cur_state) {
                println!("DEPTH: {} INFINITE LOOP: Player 1 wins!", depth);
                break;
            }
            seen_states.insert(self.state_to_string());

            // Get current cards.  If either player has none left, the other wins
            let p1_card = match self.p1_cards.pop_front() {
                None => {
                    println!("DEPTH: {} Player 2 wins!", depth);
                    break;
                }
                Some(card) => card,
            };
            let p2_card = match self.p2_cards.pop_front() {
                None => {
                    println!("DEPTH: {} Player 1 wins!", depth);
                    break;
                }
                Some(card) => card,
            };

            // play game:
            let mut p2_wins_round = false;
            // check for recursion
            if self.p1_cards.len() >= p1_card && self.p2_cards.len() >= p2_card {
                // create new game and play that game
                let mut sub_solver = self.create_sub_game(p1_card, p2_card);
                sub_solver.play_recursive_game(depth + 1);

                if sub_solver.p1_cards.is_empty() {
                    p2_wins_round = true
                } // else either p1 won the subgame or it ended due to infinite loop
            } else {
                // normal game
                if p2_card > p1_card {
                    p2_wins_round = true;
                } // else p2 wins round
            }
            if p2_wins_round {
                self.p2_cards.push_back(p2_card);
                self.p2_cards.push_back(p1_card);
            } else {
                self.p1_cards.push_back(p1_card);
                self.p1_cards.push_back(p2_card);
            }
        }
    }
}

impl Solver for DayTwentyTwoSolver {
    fn from_input(input: &String) -> Result<Box<DayTwentyTwoSolver>, String> {
        let mut solver = DayTwentyTwoSolver::new();
        let mut p2_input = false;
        for line in input.lines() {
            if line == "Player 1:" || line.len() == 0 {
                continue;
            }
            if line == "Player 2:" {
                p2_input = true;
                continue;
            }
            let cur_card: usize = line.parse().expect("BAD INPUT!!");
            if p2_input {
                solver.p2_cards.push_back(cur_card);
            } else {
                solver.p1_cards.push_back(cur_card);
            }
        }
        Ok(Box::new(solver))
    }
    fn part_one(&self) -> Result<usize, &str> {
        let mut game = self.clone();
        game.play_game();
        let winning_deck = if game.p1_cards.is_empty() {
            game.p2_cards
        } else {
            game.p1_cards
        };
        Ok(winning_deck
            .iter()
            .enumerate()
            .map(|(index, card)| {
                println!("{}: {}", index, card);
                (winning_deck.len() - index) * card
            })
            .sum())
    }

    fn part_two(&self) -> Result<usize, &str> {
        let mut game = self.clone();
        game.play_recursive_game(0);
        let winning_deck = if game.p1_cards.is_empty() {
            game.p2_cards
        } else {
            game.p1_cards
        };
        Ok(winning_deck
            .iter()
            .enumerate()
            .map(|(index, card)| {
                println!("{}: {}", index, card);
                (winning_deck.len() - index) * card
            })
            .sum())
    }
}

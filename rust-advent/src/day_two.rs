use super::Solver;
use regex::Regex;
use std::convert::TryInto;
use std::num::ParseIntError;

#[derive(Debug)]
struct RuleLine {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl RuleLine {
    fn from_line(line: &str) -> Result<Self, ParseIntError> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"(\d*)-(\d*) ([a-z]): ([a-z]*)").unwrap();
        }

        let captures = LINE_RE.captures(line).unwrap();

        Ok(RuleLine {
            min: captures[1].parse()?,
            max: captures[2].parse()?,
            letter: captures[3].chars().next().unwrap(),
            password: captures[4].to_string(),
        })
    }

    fn is_valid_one(&self) -> bool {
        let valid_letters = self.password.chars().filter(|c| *c == self.letter).count();
        self.min <= valid_letters && valid_letters <= self.max
    }

    fn is_valid_two(&self) -> bool {
        let first = match self.password.chars().nth(self.min - 1) {
            Some(letter) => letter == self.letter,
            None => false,
        };
        let second = match self.password.chars().nth(self.max - 1) {
            Some(letter) => letter == self.letter,
            None => false,
        };
        (first && !second) || (second && !first)
    }
}

pub struct DayTwoSolver {
    all_rules: Vec<RuleLine>,
}

impl Solver for DayTwoSolver {
    fn from_input(input: &String) -> Result<Box<DayTwoSolver>, String> {
        let mut rules: Vec<RuleLine> = Vec::new();

        for line in input.lines() {
            rules.push(match RuleLine::from_line(line) {
                Ok(val) => val,
                Err(error) => return Err(error.to_string()),
            });
        }
        Ok(Box::new(DayTwoSolver { all_rules: rules }))
    }

    fn part_one(&self) -> Result<usize, &str> {
        let valid_rules = self.all_rules.iter().filter(|&r| r.is_valid_one()).count();

        Ok(valid_rules)
    }
    fn part_two(&self) -> Result<usize, &str> {
        let valid_rules = self.all_rules.iter().filter(|&r| r.is_valid_two()).count();
        Ok(valid_rules)
    }
}

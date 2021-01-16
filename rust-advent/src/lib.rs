#[macro_use]
extern crate lazy_static;
extern crate regex;

pub trait Solver {
    fn from_input(input: &String) -> Result<Box<Self>, String>
    where
        Self: Sized;

    fn part_one(&self) -> Result<usize, &str> {
        Err("Not implemented yet!")
    }
    fn part_two(&self) -> Result<usize, &str> {
        Err("Not implemented yet!")
    }
}

pub mod day_one;
pub mod day_three;
pub mod day_two;

pub mod day_nineteen;
pub mod day_twenty;

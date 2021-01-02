pub trait Solver {
    fn from_input(input: &String) -> Result<Self, String>
    where
        Self: Sized;

    fn part_one(&self) -> Result<u32, &str> {
        Err("Not implemented yet!")
    }
    fn part_two(&self) -> Result<u32, &str> {
        Err("Not implemented yet!")
    }
}

pub mod day_one;

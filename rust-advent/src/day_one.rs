use super::Solver;

pub struct DayOneSolver {
    all_ints: Vec<u32>,
}

impl Solver for DayOneSolver {
    fn from_input(input: &String) -> Result<Box<DayOneSolver>, String> {
        let mut all_ints: Vec<u32> = Vec::new();

        for line in input.lines() {
            all_ints.push(match line.parse() {
                Ok(val) => val,
                Err(error) => return Err(error.to_string()),
            });
        }

        Ok(Box::new(DayOneSolver { all_ints: all_ints }))
    }
    fn part_one(&self) -> Result<u32, &str> {
        for i in 0..self.all_ints.len() {
            let i_val = self.all_ints[i];
            for j in i..self.all_ints.len() {
                let j_val = self.all_ints[j];
                if i_val + j_val == 2020 {
                    return Ok(i_val * j_val);
                }
            }
        }
        Err("No solution found for part one")
    }

    fn part_two(&self) -> Result<u32, &str> {
        let len = self.all_ints.len();
        for i in 0..len {
            let i_val = self.all_ints[i];
            for j in i..len {
                let j_val = self.all_ints[j];
                for k in j..len {
                    let k_val = self.all_ints[k];
                    if i_val + j_val + k_val == 2020 {
                        return Ok(i_val * j_val * k_val);
                    }
                }
            }
        }
        Err("No solution found for part two")
    }
}

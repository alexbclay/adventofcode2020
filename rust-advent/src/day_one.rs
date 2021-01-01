pub fn say_hi() {
    println!("Hello from day one!");
}

pub struct Solver {
    all_ints: Vec<u32>,
}

impl Solver {
    pub fn new(input: String) -> Result<Solver, String> {
        let mut all_ints: Vec<u32> = Vec::new();

        for line in input.lines() {
            match line.parse() {
                Ok(val) => all_ints.push(val),
                Err(err) => {
                    println!("COULD NOT PARSE! {:?}", err);
                    return Err(format!("Could not parse line: {}", line));
                }
            };
        }

        Ok(Solver { all_ints: all_ints })
    }

    pub fn part_one(&self) -> Result<u32, &str> {
        for i in 0..self.all_ints.len() {
            let i_val = self.all_ints[i];
            for j in i..self.all_ints.len() {
                let j_val = self.all_ints[j];
                if i_val + j_val == 2020 {
                    return Ok(i_val * j_val);
                }
            }
        }
        Err("No solution found")
    }

    pub fn part_two(&self) -> Result<u32, &str> {
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
        Err("No solution found")
    }
}

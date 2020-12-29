pub fn say_hi() {
    println!("Hello from day one!");
}

pub struct Solver {
    input_lines: String,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        Solver { input_lines: input }
    }

    pub fn part_one(&self) -> i32 {
        for line in self.input_lines.lines() {
            if line.contains("9") {
                println!("{}", line);
            }
        }
        10
    }
    pub fn part_two(&self) -> i32 {
        12
    }
}

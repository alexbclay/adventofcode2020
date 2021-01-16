use super::Solver;

pub struct DayThreeSolver {
    lines: Vec<String>,
}

impl DayThreeSolver {
    fn tree_encounters(&self, right: usize, down: usize) -> usize {
        let mut row = 0;
        let mut col = 0;
        let mut tree_count = 0;
        let len = self.lines.len();
        let width = self.lines[0].len();
        loop {
            if self.lines[row].chars().nth(col).unwrap() == '#' {
                tree_count += 1;
            }
            row += down;
            if row >= len {
                break;
            }
            col = (col + right) % width;
        }
        tree_count
    }
}

impl Solver for DayThreeSolver {
    fn from_input(input: &String) -> Result<Box<DayThreeSolver>, String> {
        let all_lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
        Ok(Box::new(DayThreeSolver { lines: all_lines }))
    }
    fn part_one(&self) -> Result<usize, &str> {
        Ok(self.tree_encounters(3, 1))
    }
    fn part_two(&self) -> Result<usize, &str> {
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        Ok(slopes
            .iter()
            .map(|slope| self.tree_encounters(slope.0, slope.1))
            .product::<usize>())
    }
}

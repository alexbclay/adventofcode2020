use super::Solver;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct DayTwentyThreeSolver {
    next_cup: Vec<usize>,
    highest: usize,
    current_cup: usize,
}

impl DayTwentyThreeSolver {
    fn new() -> DayTwentyThreeSolver {
        DayTwentyThreeSolver {
            highest: 0,
            next_cup: vec![0; 1000001],
            current_cup: 0,
        }
    }

    fn print_state(&self, iters: usize) {
        // print the first "iters" number of cups in the game, or all of them if there are fewer than that
        let mut seen = HashSet::new();
        let mut cur_cup = self.current_cup;
        let mut printme = vec![];
        let mut count = 0;
        loop {
            if seen.contains(&cur_cup) || count >= iters {
                break;
            }
            count += 1;
            seen.insert(cur_cup);
            printme.push(cur_cup);
            cur_cup = self.next_cup[cur_cup];
        }
        println!("{:?}", printme);
    }

    fn do_turn(&mut self) {
        // Take a single turn

        // println!("Cup: {}", self.current_cup);

        // get the next 3 cups
        let mut next_three = vec![];
        let mut next_cup = self.next_cup[self.current_cup];

        let mut next_target = self.current_cup - 1;
        for _ in 0..3 {
            next_three.push(next_cup);
            // next_cup = self.map.get(&next_cup).unwrap();
            next_cup = self.next_cup[next_cup];
            if next_target == 0 {
                next_target = self.highest;
            }

            // if what would be the target cup is already taken out, find its replacement
            while next_three.contains(&next_target) {
                // println!("Already taken! {}", next_target);
                next_target -= 1;
                if next_target == 0 {
                    next_target = self.highest;
                }
            }
        }
        // println!("Next: {:?}", next_three);
        // println!("Destination: {}", next_target);
        // println!("New next: {}", next_cup);

        // == reorder! ==
        // current cup now skips the next three
        self.next_cup[self.current_cup] = next_cup;

        // save where the target cup is pointing
        let target_next = self.next_cup[next_target];

        // target now points to the first of the next three
        self.next_cup[next_target] = next_three[0];

        // last of next three points to former next of target
        self.next_cup[next_three[2]] = target_next;

        // current cup moves up one
        self.current_cup = next_cup;
    }
}

impl Solver for DayTwentyThreeSolver {
    fn from_input(input: &String) -> Result<Box<DayTwentyThreeSolver>, String> {
        let mut solver = DayTwentyThreeSolver::new();
        let nums: Vec<usize> = input
            .trim()
            .chars()
            .map(|l| l.to_string().parse().expect("BAD INPUT"))
            .collect();
        println!("{:?}", nums);
        println!("{}", solver.next_cup.len());
        let first_num = nums[0];
        let mut prev_num = &first_num;
        for num in &nums[1..] {
            println!("{}", num);
            solver.next_cup[*prev_num] = *num;
            if num > &solver.highest {
                solver.highest = *num;
            }
            prev_num = num;
        }
        solver.next_cup[*prev_num] = first_num;
        solver.current_cup = first_num;

        Ok(Box::new(solver))
    }
    fn part_one(&self) -> Result<usize, &str> {
        let mut game = self.clone();
        for _iter in 0..100 {
            // println!("-- move {} --", _iter);
            game.do_turn();
        }
        game.current_cup = 1;
        game.print_state(10);

        // TODO: is it worth turning it into a string?
        Ok(1)
    }

    fn part_two(&self) -> Result<usize, &str> {
        let mut game = self.clone();

        // add all the remaing cups until the size reaches 1000000
        for cup in game.highest + 1..1000000 {
            game.next_cup[cup] = cup + 1;
        }

        // set the last cup of the original input to point at the start of the rest of the million
        let mut last = game.current_cup;
        while game.next_cup[last] != game.current_cup {
            last = game.next_cup[last];
        }

        game.next_cup[last] = game.highest + 1;
        game.next_cup[1000000] = game.current_cup;
        game.highest = 1000000;
        // game.print_state(15);

        // println!("{}", game.next_cup.len());
        for iter in 0..10000000 {
            if iter % 1000000 == 0 {
                println!("Iter: {}", iter);
            }
            game.do_turn();
        }
        let one = game.next_cup[1];
        let two = game.next_cup[one];
        println!("one: {}", one);
        println!("two: {}", two);
        Ok(one * two)
    }
}

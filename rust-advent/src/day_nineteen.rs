use super::Solver;
use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;

#[derive(Debug)]
struct Rule {
    symbol: String,
    sub_rules: Vec<Vec<String>>,
    terminal: Option<String>,
}

impl Rule {
    fn get_predict_items(&self, position: usize, input_index: usize) -> Vec<Item> {
        let mut item_vec = vec![];
        for rule in &self.sub_rules {
            item_vec.push(Item {
                lhs: self.symbol.to_string(),
                rhs: rule.to_vec(),
                position: position,
                input_start: input_index,
            });
        }
        item_vec
    }

    fn get_scan_item(&self, input_index: usize) -> Item {
        Item {
            lhs: self.symbol.to_string(),
            rhs: vec![self.terminal.as_ref().unwrap().to_string()],
            position: 1,
            input_start: input_index,
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct Item {
    lhs: String,        // Production rule left side
    rhs: Vec<String>,   // Production rule right side
    position: usize,    // index of the partial parse
    input_start: usize, // number of characters of the input consumed
}

impl Item {
    fn get_next_symbol(&self) -> Option<&String> {
        self.rhs.get(self.position)
    }

    fn last_symbol(&self) -> Option<String> {
        if self.position >= self.rhs.len() {
            None
        } else {
            Some(self.rhs[self.position].to_string())
        }
    }

    fn move_position_right(&self) -> Item {
        Item {
            lhs: self.lhs.to_string(),
            rhs: self.rhs.to_vec(),
            position: self.position + 1,
            input_start: self.input_start,
        }
    }

    fn is_success(&self, start_symbol: &String) -> bool {
        self.position == self.rhs.len() && self.input_start == 0 && &self.lhs == start_symbol
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {:?} (pos: {}) ({})",
            self.lhs, self.rhs, self.position, self.input_start
        )
    }
}

impl cmp::PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        if self.lhs != other.lhs {
            return false;
        }
        if self.position != other.position {
            return false;
        }
        if self.input_start != other.input_start {
            return false;
        }

        if self.rhs.len() != other.rhs.len() {
            return false;
        }

        for (i, symbol) in self.rhs.iter().enumerate() {
            if &other.rhs[i] != symbol {
                return false;
            }
        }
        for (i, symbol) in other.rhs.iter().enumerate() {
            if &self.rhs[i] != symbol {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct StateSets {
    sets: Vec<Vec<Item>>,
    start_symbol: String,
}

impl StateSets {
    fn new(size: usize, start_symbol: String) -> StateSets {
        let mut empty_sets = vec![];
        for _ in 0..size + 1 {
            empty_sets.push(vec![]);
        }
        StateSets {
            sets: empty_sets,
            start_symbol: start_symbol.to_string(),
        }
    }

    fn initialize(&mut self, start_rule: &Rule) {
        self.add_all(0, &start_rule.get_predict_items(0, 0));
    }

    fn add(&mut self, set_index: usize, item: &Item) {
        // Add single item
        if set_index >= self.sets.len() {
            return;
        }
        if !self.sets[set_index].iter().any(|i| i == item) {
            self.sets[set_index].push(item.clone());
        }
    }
    fn add_all(&mut self, set_index: usize, items: &Vec<Item>) {
        // Add multiple items
        for item in items.iter() {
            self.add(set_index, item);
        }
    }

    fn len(&mut self, set_index: usize) -> usize {
        self.sets[set_index].len()
    }

    fn get_item(&self, set_index: usize, item_index: usize) -> Item {
        self.sets[set_index][item_index].clone()
    }

    fn do_completions(&mut self, state_index: usize, item: &Item) {
        self.add_all(
            state_index,
            &self.sets[item.input_start]
                .iter()
                .filter_map(|i| match i.last_symbol() {
                    None => None,
                    Some(symbol) => {
                        if symbol == item.lhs.to_string() {
                            Some(i.move_position_right())
                        } else {
                            None
                        }
                    }
                })
                .collect(),
        );
    }

    fn is_success(&self) -> bool {
        self.sets[self.sets.len() - 1]
            .iter()
            .any(|i| i.is_success(&self.start_symbol))
    }
}

impl fmt::Display for StateSets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");
        for (set_index, set) in self.sets.iter().enumerate() {
            output += &format!("-- S({}) --\n", set_index);
            for item in set.iter() {
                output += &format!("{}\n", item);
            }
        }
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub struct DayNineteenSolver {
    rules: HashMap<String, Rule>, // map of Symbol -> array of symbol arrays
    terminal_rules: Vec<String>,  // All symbols for rules which are terminal (produce "a" or "b")
    inputs: Vec<String>,          // input strings to be tested
    start_symbol: Option<String>,
}

impl DayNineteenSolver {
    fn new() -> DayNineteenSolver {
        DayNineteenSolver {
            rules: HashMap::new(),
            terminal_rules: vec![],
            inputs: vec![],
            start_symbol: None,
        }
    }

    fn set_rules_from_input(&mut self, input: &String) {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"(\d*): (.*)").unwrap();
        }
        let mut lhs: HashSet<String> = HashSet::new();
        let mut rhs: HashSet<String> = HashSet::new();

        let rule_lines = input
            .lines()
            .filter_map(|line| match RULE_RE.captures(line) {
                None => None,
                Some(captures) => Some((
                    captures.get(1).map_or("", |m| m.as_str()).to_string(),
                    captures.get(2).map_or("", |m| m.as_str()).to_string(),
                )),
            });
        for rule_tuple in rule_lines {
            // println!("{:?}", rule_tuple);
            let leftside: String = rule_tuple.0.to_string();
            // keep track of leftside symbols so that we can find the start symbol
            lhs.insert(rule_tuple.0.to_string());

            let rightside: String = rule_tuple.1.to_string();
            if rightside == "\"a\"" || rightside == "\"b\"" {
                // if it's a terminal rule, mark that
                self.terminal_rules.push(rule_tuple.0);
                self.rules.insert(
                    leftside.to_string(),
                    Rule {
                        symbol: leftside.to_string(),
                        sub_rules: vec![],
                        terminal: Some(rightside[1..2].to_string()),
                    },
                );
            } else {
                // not a terminal rule, so push
                let mut all_rules = Rule {
                    symbol: leftside.to_string(),
                    sub_rules: vec![],
                    terminal: None,
                };
                let mut cur_rule = vec![];
                let mut cur_symbol = String::from("");
                for letter in rightside.chars() {
                    match letter {
                        ' ' => {
                            // a space means the end of the current symbol, so add it to the current list
                            // and start a new symbol (if it's not empty)
                            if cur_symbol.len() > 0 {
                                // remember rightside symbols
                                rhs.insert(cur_symbol.to_string());
                                cur_rule.push(cur_symbol);

                                cur_symbol = String::from("");
                            }
                        }
                        '|' => {
                            // OR: push the current rule list to the list of rules and start a new rule
                            // and a new symbol
                            all_rules.sub_rules.push(cur_rule);
                            cur_rule = vec![];
                            cur_symbol = String::from("");
                        }
                        _ => {
                            // anything else is part of a symbol, so add it to the current symbol
                            cur_symbol.push(letter);
                        }
                    }
                }
                // remember rightside symbols
                rhs.insert(cur_symbol.to_string());

                cur_rule.push(cur_symbol);
                all_rules.sub_rules.push(cur_rule);
                self.rules.insert(leftside, all_rules);
            }
        }
        // println!("LHS: {:?}", lhs);
        // println!("RHS: {:?}", rhs);
        // println!("DIFF: {:?}", lhs.difference(&rhs).collect::<Vec<&String>>());
        self.start_symbol = lhs.difference(&rhs).next().map(|s| s.to_string());
    }

    fn set_input_lines(&mut self, input: &String) {
        lazy_static! {
            static ref STRING_RE: Regex = Regex::new(r"^[ab]+").unwrap();
        }
        self.inputs = input
            .lines()
            .filter_map(|line| {
                if STRING_RE.is_match(line) {
                    Some(line.to_string())
                } else {
                    None
                }
            })
            .collect();
    }

    fn earley_parse(&self, input: &String) -> bool {
        println!("=== PARSING STRING: {} ===", input);

        let start = self.start_symbol.as_ref();
        let mut state_sets = StateSets::new(input.len(), start.unwrap().to_string());

        let start_rule = self.rules.get(start.unwrap()).unwrap();

        state_sets.initialize(start_rule);

        // println!("{}", state_sets);

        for state_index in 0..input.len() + 1 {
            // for state_index in 0..3 {
            // println!(">>>>>>>>> outer loop #{} <<<<<<<<<<<", state_index);
            let mut inner_index = 0;
            loop {
                if inner_index >= state_sets.len(state_index) {
                    break;
                }

                let cur_item = state_sets.get_item(state_index, inner_index);
                // println!("EXAM: {}", cur_item);
                let next_symbol = match cur_item.get_next_symbol() {
                    None => {
                        // COMPLETE: this item's rule has completed its scan
                        // go back through the state sets to find other matches that could also result in this item
                        // println!("--> COMPLETE!");
                        state_sets.do_completions(state_index, &cur_item);

                        inner_index += 1;
                        // println!("cur_state:\n{}", state_sets);

                        continue;
                    }
                    Some(symbol) => symbol,
                };
                // println!("  next symbol: {:?}", next_symbol);

                let next_rule = self.rules.get(next_symbol).unwrap();
                match &next_rule.terminal {
                    None => {
                        // PREDICT: add predictions to the current state set
                        // println!("--> PREDICT!");
                        state_sets
                            .add_all(state_index, &next_rule.get_predict_items(0, state_index))
                    }
                    Some(terminal) => {
                        // SCAN: found a terminal.  If it matches the input, then put it the next state set
                        if terminal == &input.chars().nth(state_index).unwrap_or(' ').to_string() {
                            // println!("--> SCAN: Terminal {}", terminal);
                            state_sets.add(state_index + 1, &next_rule.get_scan_item(state_index));
                        } else {
                            // println!("--> SCAN failed {}", terminal);
                        }
                    }
                }
                // println!("\t next rules: {:?}", next_rule);

                // println!("\t cur_state:\n {}", state_sets);
                inner_index += 1;
                // if inner_index >= 15 {
                //     break;
                // }
            }
        }
        // println!("\t cur_state:\n {}", state_sets);

        state_sets.is_success()
    }
}

impl Solver for DayNineteenSolver {
    fn from_input(input: &String) -> Result<Box<DayNineteenSolver>, String> {
        let mut solver = DayNineteenSolver::new();
        solver.set_rules_from_input(input);
        solver.set_input_lines(input);

        // println!("{:?}", solver);
        Ok(Box::new(solver))
    }

    fn part_one(&self) -> Result<usize, &str> {
        Ok(self
            .inputs
            .iter()
            .filter(|input| self.earley_parse(input))
            .count()
            .try_into()
            .unwrap())
        // self.earley_parse(&self.inputs[0]);
        // Ok(1)
    }
    fn part_two(&self) -> Result<usize, &str> {
        Ok(2)
    }
}

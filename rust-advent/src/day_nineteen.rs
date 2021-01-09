use super::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    symbol: String,
    sub_rules: Vec<Vec<String>>,
    terminal: Option<String>,
}

impl Rule {
    fn get_items(&self) -> Vec<Item> {
        let mut item_vec = vec![];
        for rule in &self.sub_rules {
            item_vec.push(Item {
                lhs: self.symbol.to_string(),
                rhs: rule.to_vec(),
                position: 0,
            });
        }
        item_vec
    }
}

#[derive(Debug)]
struct Item {
    lhs: String,      // Production rule left side
    rhs: Vec<String>, // Production rule right side
    position: usize,  // index of the partial parse
}

#[derive(Debug)]
struct StateSet {
    states: Vec<Item>,
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
            println!("{:?}", rule_tuple);
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
                        terminal: Some(rightside.to_string()),
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
        println!("LHS: {:?}", lhs);
        println!("RHS: {:?}", rhs);
        println!("DIFF: {:?}", lhs.difference(&rhs).collect::<Vec<&String>>());
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

    fn earley_parse(&self, input: &String) {
        println!("=== PARSING STRING: {} ===", input);

        // initialize state set with empty index 0 state
        let mut state_sets: Vec<StateSet> = vec![StateSet { states: vec![] }];

        let start = self.start_symbol.as_ref();
        let start_rule = self.rules.get(start.unwrap()).unwrap();
        for item in start_rule.get_items() {
            state_sets[0].states.push(item);
            // state_sets[0].states.push(Item {
            //     lhs: start.unwrap().to_string(),
            //     rhs: rule.to_vec(),
            //     position: 0,
            // });
        }
        println!("{:?}", state_sets);

        // for state_index in 0..input.len(){
        for state_index in 0..1 {
            println!("outer loop #{}", state_index);
            let mut inner_index = 0;
            loop {
                let cur_item = &state_sets[state_index].states[inner_index];
                println!("\t examining {:?}", cur_item);

                // prediction
                let next_symbol = cur_item.rhs.get(cur_item.position).unwrap();
                println!("\t next symbol: {:?}", next_symbol);

                let next_rule = self.rules.get(next_symbol).unwrap();
                for item in next_rule.get_items() {
                    state_sets[state_index].states.push(item);
                }
                println!("\t next rules: {:?}", next_rule);

                println!("\t cur_state: {:?}", state_sets);
                break;
            }
        }
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

    fn part_one(&self) -> Result<u32, &str> {
        self.earley_parse(&self.inputs[0]);
        Ok(1)
    }
    fn part_two(&self) -> Result<u32, &str> {
        Ok(2)
    }
}

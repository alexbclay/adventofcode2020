use super::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct DayTwentyOneSolver {
    // possible ingredients and how many there are of each
    ingredients: HashMap<String, usize>,
    // map of allergies to possible ingredients
    allergy_ingredient_map: HashMap<String, HashSet<String>>,
    ingredient_allergy_map: HashMap<String, HashSet<String>>,
}

impl DayTwentyOneSolver {
    pub fn new() -> DayTwentyOneSolver {
        DayTwentyOneSolver {
            ingredients: HashMap::new(),
            allergy_ingredient_map: HashMap::new(),
            ingredient_allergy_map: HashMap::new(),
        }
    }
}

impl Solver for DayTwentyOneSolver {
    fn from_input(input: &String) -> Result<Box<DayTwentyOneSolver>, String> {
        let mut solver = DayTwentyOneSolver::new();

        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"(.*) \(contains (.*)\)").unwrap();
        }
        for line in input.lines() {
            let captures = LINE_RE.captures(line).unwrap();
            let ingredients = captures[1].to_string();
            let allergies = captures[2].to_string();
            let mut line_ingredient_set = HashSet::<String>::new();

            // Process the ingredients part of the line
            let mut cur_ingredient = String::from("");
            for letter in ingredients.chars() {
                if letter == ' ' {
                    // add the current ingredient to the set of ingredients on the line
                    line_ingredient_set.insert(cur_ingredient.clone());
                    // update the overall count of ingredients
                    *solver
                        .ingredients
                        .entry(cur_ingredient.clone())
                        .or_insert(0) += 1;
                    // add an empty set to the ingredient -> allergy map
                    solver
                        .ingredient_allergy_map
                        .entry(cur_ingredient)
                        .or_insert(HashSet::new());
                    cur_ingredient = String::from("");
                } else {
                    cur_ingredient.push(letter);
                }
            }

            // add the current ingredient to the set of ingredients on the line
            line_ingredient_set.insert(cur_ingredient.clone());
            // update the overall count of ingredients
            *solver
                .ingredients
                .entry(cur_ingredient.clone())
                .or_insert(0) += 1;
            // add an empty set to the ingredient -> allergy map
            solver
                .ingredient_allergy_map
                .entry(cur_ingredient)
                .or_insert(HashSet::new());

            // Process the allergy part of the line
            let mut cur_allergy = String::from("");
            for letter in allergies.chars() {
                if letter == ' ' {
                    continue;
                } else if letter == ',' {
                    solver
                        .allergy_ingredient_map
                        .entry(cur_allergy)
                        .and_modify(|cur_set| {
                            cur_set.retain(|item| line_ingredient_set.contains(item))
                        })
                        .or_insert(line_ingredient_set.clone());

                    cur_allergy = String::from("");
                } else {
                    cur_allergy.push(letter);
                }
            }
            solver
                .allergy_ingredient_map
                .entry(cur_allergy)
                .and_modify(|cur_set| cur_set.retain(|item| line_ingredient_set.contains(item)))
                .or_insert(line_ingredient_set.clone());
        }

        // create the inverse map of ingredient -> set of possible allergies
        for (allergy, ingredients) in &solver.allergy_ingredient_map {
            for ingredient in ingredients {
                solver
                    .ingredient_allergy_map
                    .entry(ingredient.to_string())
                    .and_modify(|cur_set| {
                        cur_set.insert(allergy.to_string());
                    });
            }
        }

        Ok(Box::new(solver))
    }

    fn part_one(&self) -> Result<usize, &str> {
        Ok(self
            .ingredient_allergy_map
            .iter()
            .filter_map(|(ingredient, allergies)| {
                if allergies.len() != 0 {
                    None
                } else {
                    Some(self.ingredients.get(ingredient).unwrap())
                }
            })
            .sum())
    }
    fn part_two(&self) -> Result<usize, &str> {
        println!("-- allergy -> ingredients");
        for (allergy, ingredients) in &self.allergy_ingredient_map {
            if ingredients.len() != 0 {
                println!("{} -> {:?}", allergy, ingredients);
            }
        }
        println!("-- ingredient -> allergies");
        for (ingredient, allergies) in &self.ingredient_allergy_map {
            if allergies.len() != 0 {
                println!("{} -> {:?}", ingredient, allergies);
            }
        }
        // copy the ingredient to allergy map so we can update it
        let mut final_map = self.ingredient_allergy_map.clone();

        // prime queue with all ingredient,allergy pairs that have a single allergy possibility
        let mut queue: VecDeque<(String, String)> = VecDeque::new();
        let mut final_list: Vec<(String, String)> = vec![];
        for (ingredient, allergies) in &self.ingredient_allergy_map {
            if allergies.len() == 1 {
                queue.push_back((
                    ingredient.to_string(),
                    allergies.iter().next().unwrap().to_string(),
                ));
                final_map.remove(ingredient);
            }
        }
        let mut loops = 0;
        loop {
            println!("LOOP {}: {:?}", loops, queue);
            println!("{:?}", final_list);

            let (ingredient, allergy) = match queue.pop_front() {
                None => break,
                Some(t) => t,
            };
            final_list.push((ingredient.to_string(), allergy.to_string()));
            loops += 1;
            for sub_ingredient in self.allergy_ingredient_map.get(&allergy).unwrap() {
                if sub_ingredient == &ingredient {
                    continue;
                }
                println!("{:?}", sub_ingredient);
                final_map
                    .entry(sub_ingredient.to_string())
                    .and_modify(|set| {
                        set.remove(&allergy);
                    });
                let remaining = final_map.get_mut(sub_ingredient).unwrap();
                if remaining.len() == 1 {
                    println!("\t\t{} {:?}", sub_ingredient, remaining);
                    queue.push_back((
                        sub_ingredient.to_string(),
                        remaining.iter().next().unwrap().to_string(),
                    ));
                }
            }
        }
        final_list.sort_by(|(ingr_a, allg_a), (ingr_b, allg_b)| allg_a.cmp(allg_b));

        let solution_string = final_list
            .iter()
            .map(|(ingredient, allergy)| ingredient.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("{:?}", solution_string);
        Ok(2)
    }
}

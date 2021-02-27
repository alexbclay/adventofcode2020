use super::Solver;
use std::collections::{HashMap, HashSet};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HexCoord {
    x: isize,
    y: isize,
}

impl HexCoord {
    fn get_neighbors(&self) -> Vec<HexCoord> {
        let mut neighbors = vec![];

        neighbors.push(
            // NE
            HexCoord { x: 1, y: 1 } + self,
        );
        neighbors.push(
            // NW
            HexCoord { x: 1, y: 0 } + self,
        );
        neighbors.push(
            // SE
            HexCoord { x: -1, y: 0 } + self,
        );
        neighbors.push(
            // SW
            HexCoord { x: -1, y: -1 } + self,
        );
        neighbors.push(
            // E
            HexCoord { x: 0, y: 1 } + self,
        );
        neighbors.push(
            // W
            HexCoord { x: 0, y: -1 } + self,
        );
        neighbors
    }
}

impl Add for HexCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<'a> Add<&'a HexCoord> for HexCoord {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Sum<&'a HexCoord> for HexCoord {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(HexCoord { x: 0, y: 0 }, Add::add)
    }
}

#[derive(Debug)]
pub struct DayTwentyFourSolver {
    lines: Vec<Vec<HexCoord>>,
}

impl DayTwentyFourSolver {
    fn new() -> DayTwentyFourSolver {
        DayTwentyFourSolver { lines: vec![] }
    }

    fn initial_tiles(&self) -> HashSet<HexCoord> {
        let total = self
            .lines
            .iter()
            .map(|tile_vec| tile_vec.iter().sum::<HexCoord>());
        let mut set = HashSet::new();
        for line in total {
            if set.contains(&line) {
                set.remove(&line);
            } else {
                set.insert(line);
            }
        }
        set
    }
}

impl Solver for DayTwentyFourSolver {
    fn from_input(input: &String) -> Result<Box<DayTwentyFourSolver>, String> {
        let mut solver = DayTwentyFourSolver::new();

        for line in input.lines() {
            let mut cur_vec: Vec<HexCoord> = vec![];
            let mut prev_token = 'a';
            for token in line.chars() {
                if prev_token == 'n' {
                    if token == 'e' {
                        cur_vec.push(HexCoord { x: 1, y: 1 })
                    } else {
                        // no input validation
                        cur_vec.push(HexCoord { x: 1, y: 0 })
                    }
                } else if prev_token == 's' {
                    if token == 'e' {
                        cur_vec.push(HexCoord { x: -1, y: 0 })
                    } else {
                        // no input validation
                        cur_vec.push(HexCoord { x: -1, y: -1 })
                    }
                } else {
                    // previous token was e or w
                    if token == 'e' {
                        cur_vec.push(HexCoord { x: 0, y: 1 })
                    } else if token == 'w' {
                        cur_vec.push(HexCoord { x: 0, y: -1 })
                    }
                }
                prev_token = token;
            }
            solver.lines.push(cur_vec);
        }

        Ok(Box::new(solver))
    }
    fn part_one(&self) -> Result<usize, &str> {
        let set = self.initial_tiles();
        Ok(set.len())
    }

    fn part_two(&self) -> Result<usize, &str> {
        let mut black_tiles = self.initial_tiles();

        for day in 1..101 {
            let mut neighbors = HashMap::new();

            for tile in &black_tiles {
                // add one to the neighbor count of all black tile neighbors
                for neighbor in tile.get_neighbors() {
                    neighbors
                        .entry(neighbor)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }

            let mut new_black_tiles = HashSet::new();

            for (tile, neighbor_count) in &neighbors {
                // println!("[{}] {:?} -> {}", if black_tiles.contains(tile) { "B"} else {"W"}, tile, neighbor_count);

                let is_black = black_tiles.contains(tile);
                if is_black && (*neighbor_count == 1 || *neighbor_count == 2) {
                    // keep black tiles with 1 black neighbors (ie flip when n=0 or n>2)
                    new_black_tiles.insert(tile.clone());
                    // println!("  stay black");
                }
                // else if is_black && (*neighbor_count == 0 || *neighbor_count >2 ) {
                //     println!("  flip to white");
                // }
                else if !is_black && *neighbor_count == 2 {
                    // flip white tiles with exactly 2 black neighbors
                    new_black_tiles.insert(tile.clone());
                    // println!("  flip to black");
                }
            }
            black_tiles = new_black_tiles;
            // println!("Day {}: {}", day, black_tiles.len());
        }
        Ok(black_tiles.len())
    }
}

use super::Solver;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Debug)]
enum Flip {
    NoFlip,
    Flipped,
}

#[derive(Debug)]
struct Transform {
    rotation: Rotation,
    flip: Flip,
}

struct SideVals {
    // Flipped just reverses these values
    normal: u32,  // 1010011
    inverse: u32, // 1100101
}

impl SideVals {
    fn matches(&self, other: &SideVals) -> Option<Flip> {
        if self.normal == other.inverse {
            Some(Flip::NoFlip)
        } else if self.normal == other.normal {
            Some(Flip::Flipped)
        } else {
            None
        }
    }
}

impl fmt::Display for SideVals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.normal, self.inverse)
    }
}

fn to_numbers(bin: &Vec<bool>) -> SideVals {
    // convert the boolean array into an int
    let arr = bin.iter().map(|b| if *b { '1' } else { '0' });

    let first_str: String = arr.collect();
    let first = u32::from_str_radix(&first_str, 2).unwrap();
    let second_str: String = first_str.chars().rev().collect();
    let second = u32::from_str_radix(&second_str, 2).unwrap();
    SideVals {
        normal: first,
        inverse: second,
    }
}

struct Tile {
    id: usize,
    data: Vec<Vec<bool>>, // Raw data at no rotation, no flip
    flip: Flip,
    rotation: Rotation,
    all: [SideVals; 4],
}

impl Tile {
    fn new(id: usize, data: Vec<Vec<bool>>) -> Tile {
        // calculate the numerical representations of each side
        let north = to_numbers(&data[0]);
        let south = to_numbers(&data.last().unwrap());
        let east = to_numbers(&data.iter().map(|row| *row.last().unwrap()).collect());
        let west = to_numbers(&data.iter().map(|row| row[0]).collect());

        Tile {
            id: id,
            data: data,
            flip: Flip::NoFlip,
            rotation: Rotation::Deg0,
            all: [north, east, south, west],
        }
    }

    fn match_any_side(&self, other: &Tile) -> Option<(usize, usize, Transform)> {
        for (idx, side) in self.all.iter().enumerate() {
            for (other_idx, other_side) in other.all.iter().enumerate() {
                match side.matches(other_side) {
                    None => (),
                    Some(val) => {
                        let diff = if idx > other_idx {
                            idx - other_idx
                        } else {
                            idx + 4 - other_idx
                        };
                        let rotation = match diff {
                            0 => Rotation::Deg0,
                            1 => Rotation::Deg270,
                            2 => Rotation::Deg180,
                            3 => Rotation::Deg90,
                            4 => Rotation::Deg0,
                            _ => panic!("WRONG INDEX DIFF!!!"),
                        };
                        return Some((
                            idx,
                            other_idx,
                            Transform {
                                rotation: rotation,
                                flip: val,
                            },
                        ));
                    }
                }
            }
        }
        None
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!(
            "ID: {} N: {} E: {} S: {} W: {}\n",
            self.id, self.all[0], self.all[1], self.all[2], self.all[3],
        );
        for row in &self.data {
            let mut row_out = row.iter().fold(String::from(""), |mut acc, c| {
                if *c {
                    acc.push('1');
                } else {
                    acc.push('0')
                }
                acc
            });
            row_out.push('\n');
            output.push_str(&row_out);
        }
        write!(f, "{}", output)
    }
}

pub struct DayTwentySolver {
    tiles: HashMap<usize, Tile>,
}

impl Solver for DayTwentySolver {
    fn from_input(input: &String) -> Result<Box<DayTwentySolver>, String> {
        lazy_static! {
            static ref TILE_NAME_RE: Regex = Regex::new(r"Tile (\d*):").unwrap();
            static ref TILE_LINE_RE: Regex = Regex::new(r"^[.#]+$").unwrap();
        }
        let mut tile_map = HashMap::new();
        let mut cur_id = 0;
        let mut cur_vec: Vec<Vec<bool>> = vec![];
        for line in input.lines() {
            if TILE_LINE_RE.is_match(line) {
                let bool_line = line.chars().map(|c| c == '#');
                cur_vec.push(bool_line.collect())
            } else {
                match TILE_NAME_RE.captures(line) {
                    Some(captures) => {
                        // add the previous tile to the map
                        if cur_id != 0 {
                            tile_map.insert(cur_id, Tile::new(cur_id, cur_vec));
                        }
                        cur_vec = vec![];
                        // get the new id
                        let id = captures.get(1).map_or("", |m| m.as_str()).to_string();
                        cur_id = id.parse().unwrap();
                    }
                    None => (),
                }
            }
        }
        // I always forget to add the last one...
        tile_map.insert(cur_id, Tile::new(cur_id, cur_vec));

        Ok(Box::new(DayTwentySolver { tiles: tile_map }))
    }

    fn part_one(&self) -> Result<usize, &str> {
        let mut corners: Vec<usize> = vec![];
        // For each tile, find all other tiles that match in some way
        let mut by_count: HashMap<usize, Vec<usize>> = HashMap::new();

        for tile_a in self.tiles.values() {
            let mut match_count = 0;
            for tile_b in self.tiles.values() {
                if tile_a.id == tile_b.id {
                    continue;
                }
                match_count += match tile_a.match_any_side(tile_b) {
                    None => 0,
                    Some((side_a, side_b, trans)) => {
                        // println!("A: {} B:{} T:{:?}", side_a, side_b, trans);
                        println!("\tmatches: {}", tile_b.id);
                        1
                    }
                }
            }
            // If there were only 2 matches, this HAS to be a corner
            if match_count == 2 {
                corners.push(tile_a.id.try_into().unwrap());
            }
            println!("{}: {}", tile_a.id, match_count);
        }

        println!("{:?}", corners);
        // The inputs that we were given do not result in more than 4 corners, hopefully
        if corners.len() != 4 {
            panic!("This problem is harder than expected!");
        }
        let product: usize = corners.iter().fold(1, |acc, id| acc * id);
        Ok(product)
    }
    fn part_two(&self) -> Result<usize, &str> {
        // make graph of links
        // stitch together (drop borders)

        // remove sea monsters

        // count remaining '#' cells

        Ok(2)
    }
}

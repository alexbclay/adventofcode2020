mod graph;
mod tiles;

use super::Solver;
use crate::day_twenty::graph::TileGraph;
use crate::day_twenty::tiles::*;
use regex::Regex;
use std::collections::HashMap;

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
        let mut cur_arr = [[false; 10]; 10];
        let mut tile_row = 0;
        for line in input.lines() {
            if TILE_LINE_RE.is_match(line) {
                for (idx, c) in line.chars().enumerate() {
                    cur_arr[tile_row][idx] = c == '#';
                }
                tile_row += 1;
            } else {
                match TILE_NAME_RE.captures(line) {
                    Some(captures) => {
                        // add the previous tile to the map
                        if cur_id != 0 {
                            tile_map.insert(cur_id, Tile::new(cur_id, cur_arr));
                        }
                        cur_arr = [[false; 10]; 10];
                        tile_row = 0;

                        // get the new id
                        let id = captures.get(1).map_or("", |m| m.as_str()).to_string();
                        cur_id = id.parse().unwrap();
                    }
                    None => (),
                }
            }
        }
        // I always forget to add the last one...
        tile_map.insert(cur_id, Tile::new(cur_id, cur_arr));

        Ok(Box::new(DayTwentySolver { tiles: tile_map }))
    }

    fn part_one(&self) -> Result<usize, &str> {
        let graph = TileGraph::from(self.tiles.clone());

        // If there were only 2 matches, this HAS to be a corner
        let corners = graph.by_count.get(&2).unwrap();
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
        let graph = TileGraph::from(self.tiles.clone());

        // stitch tiles, find monsters and total non-monster '#'
        Ok(graph.check_for_monsters())
    }
}

use super::tiles::Tile;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct TileGraph {
    pub by_count: HashMap<usize, Vec<usize>>, // number of connections -> list of ids
    tiles: HashMap<usize, Tile>,
}

impl TileGraph {
    pub fn from(tiles: HashMap<usize, Tile>) -> TileGraph {
        // keep track of how many neighbors each tile has.  We will use this to find corners
        let mut by_count: HashMap<usize, Vec<usize>> = HashMap::new();

        // pick the first tile, then do a BFS on connected tiles
        // each link should then be oriented to match their parent tile
        // which should result in all tiles being the same orientation and ready for stitching
        let mut oriented_tiles: HashMap<usize, Tile> = HashMap::new();
        let mut tiles_to_do: VecDeque<Tile> = VecDeque::new();
        let mut tiles_seen: HashSet<usize> = HashSet::new();

        // Seed the TO DO queue with a random tile
        let first_id = *tiles.keys().next().unwrap();
        tiles_to_do.push_back(tiles.get(&first_id).unwrap().clone());

        loop {
            // get the next tile off the list
            let mut cur_tile = match tiles_to_do.pop_front() {
                // nothing left in TO DO queue, so we're done processing
                None => break,
                // next tile in the queue!
                Some(tile) => tile,
            };
            // already saw this tile, no need to double process it
            if tiles_seen.contains(&cur_tile.id) {
                continue;
            }

            // mark this tile as seen, so we don't process it again later
            tiles_seen.insert(cur_tile.id);

            // find the remaining connections for the current tile
            for connection in tiles.values() {
                if connection.id == cur_tile.id {
                    // a tile will always match itself, but that's dumb so skip it
                    continue;
                }
                match cur_tile.get_matched_tile(connection) {
                    None => (),
                    Some((direction, new_tile)) => {
                        // add newly connected tile to list of tiles to do (if it's not already been there)
                        cur_tile.linked_ids[direction] = new_tile.id;

                        if !tiles_seen.contains(&connection.id) {
                            tiles_to_do.push_back(new_tile);
                        }
                    }
                }
            }
            // add the final updated version of this tile to the tile map
            oriented_tiles.insert(cur_tile.id, cur_tile.clone());

            // also add its id to the the by_count map
            by_count
                .entry(
                    cur_tile
                        .linked_ids
                        .iter()
                        .filter(|id| *id != &0)
                        .collect::<Vec<&usize>>()
                        .len(),
                )
                .and_modify(|v| v.push(cur_tile.id))
                .or_insert(vec![cur_tile.id]);
        }

        TileGraph {
            by_count: by_count,
            tiles: oriented_tiles,
        }
    }

    fn get_tile_arr(&self) -> Vec<Vec<Option<Tile>>> {
        let corner_ids = self.by_count.get(&2).unwrap();
        // find the upper-left corner
        let mut start_corner_id = 0;
        for cid in corner_ids {
            let corner_edges = self.tiles.get(&cid).unwrap().linked_ids;
            if corner_edges[0] == 0 && corner_edges[3] == 0 {
                start_corner_id = *cid;
                break;
            }
        }
        if start_corner_id == 0 {
            panic!("Could not find the correct corner!");
        }

        // this is the number of tiles per side
        let sides = self.by_count.get(&3).unwrap().len() / 4 + 2;
        let corner_tile = self.tiles.get(&start_corner_id).unwrap();

        let mut tile_arr: Vec<Vec<Option<Tile>>> = vec![vec![None; sides]; sides];
        let mut cur_tile = corner_tile;
        let mut row = 0;
        let mut col = 0;

        loop {
            tile_arr[row][col] = Some(cur_tile.clone());

            let cur_edges = cur_tile.linked_ids;

            // if eastern linked tile is 0, we're reached the end of the row
            if cur_edges[1] == 0 {
                let first_tile_in_row = &tile_arr[row][0].as_ref().unwrap();

                if first_tile_in_row.linked_ids[2] == 0 {
                    // South is 0, so no more tiles below this row
                    break;
                } else {
                    // move on to the tile south of the beginning of the row
                    cur_tile = self.tiles.get(&first_tile_in_row.linked_ids[2]).unwrap();
                }
                col = 0;
                row += 1;
            } else {
                // we have an eastern tile, move on to use it
                cur_tile = self.tiles.get(&cur_edges[1]).unwrap();
                col += 1;
            }
        }
        tile_arr
    }

    pub fn stitch(&self) -> Vec<Vec<bool>> {
        println!("========= STITCH =========");
        let tile_arr = self.get_tile_arr();

        // total resolution is the number of tiles per side * tile size - 2 (we exclude the borders)
        let image_resolution = tile_arr.len() * 8;
        // TODO: Build image array from these tiles
        for row in tile_arr.iter() {
            let mut out = "".to_string();
            for col in row.iter() {
                match col.as_ref() {
                    None => out.push_str("N/A"),
                    Some(t) => out.push_str(&format!("{}", t.id)[..]),
                };
                out.push(' ');
            }
            println!("{}", out);
        }

        let mut image = vec![vec![false; image_resolution]; image_resolution];

        for (row_idx, tile_row) in tile_arr.iter().enumerate() {
            for (col_idx, tile) in tile_row.iter().enumerate() {
                for (tile_row_idx, tile_row) in tile.as_ref().unwrap().data[1..9].iter().enumerate()
                {
                    for (tile_col_idx, value) in tile_row[1..9].iter().enumerate() {
                        // println!("{} {} : {} {} => {}", row_idx, col_idx, tile_row_idx, tile_col_idx, value);
                        let final_row = row_idx * 8 + tile_row_idx;
                        let final_col = col_idx * 8 + tile_col_idx;
                        // println!("{} {} ", final_row, final_col);
                        image[final_row][final_col] = *value;
                    }
                }
            }
        }

        for (r, row) in image.iter().enumerate() {
            if r % 8 == 0 {
                println!("");
            }
            let mut row_str = "".to_string();
            for (c, col) in row.iter().enumerate() {
                if c % 8 == 0 {
                    row_str.push(' ');
                }
                row_str.push(if *col { '#' } else { '.' });
            }
            println!("{}", row_str);
        }

        image
    }

    pub fn check_for_monsters(&self) -> usize {
        let mut image = self.stitch();

        // monster is a set of coords to check in a 3 row x 20 col box
        /*
        . 01234567890123456789
        0                   #
        1 #    ##    ##    ###
        2  #  #  #  #  #  #
         */
        let monster = vec![
            (0, 18),
            (1, 0),
            (1, 5),
            (1, 6),
            (1, 11),
            (1, 12),
            (1, 17),
            (1, 18),
            (1, 19),
            (2, 1),
            (2, 4),
            (2, 7),
            (2, 10),
            (2, 13),
            (2, 16),
        ];
        fn rotate90(image: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
            let size = image.len();
            let mut rotated = vec![vec![false; size]; size];

            for (row_idx, row) in image.iter().enumerate() {
                for (col_idx, val) in row.iter().enumerate() {
                    rotated[col_idx][size - row_idx - 1] = *val
                }
            }
            rotated
        }

        // to check for monsters, we need to scan this box over the image
        // start row from 0 to image.len() - 3, start col from 0 to image.len() - 10
        // then we need to check each rotation and each flip (so 4x2)

        let size = image.len();
        let mut monsters_found = 0;
        for rotations in 0..4 {
            println!("ROTATIONS: {}", rotations);
            image = rotate90(image);
            for (r, row) in image.iter().enumerate() {
                if r % 8 == 0 {
                    println!("");
                }
                let mut row_str = "".to_string();
                for (c, col) in row.iter().enumerate() {
                    if c % 8 == 0 {
                        row_str.push(' ');
                    }
                    row_str.push(if *col { '#' } else { '.' });
                }
                println!("{}", row_str);
            }
            for row_idx in 0..size - 3 {
                for col_idx in 0..size - 20 {
                    let mut monster_found = true;
                    for (mon_row, mon_col) in monster.iter() {
                        if !image[row_idx + mon_row][col_idx + mon_col] {
                            monster_found = false;
                            break;
                        }
                    }
                    if monster_found {
                        println!("FOUND ONE!");
                        monsters_found += 1;
                    }
                    let mut monster_found = true;
                    for (mon_row, mon_col) in monster.iter() {
                        if !image[row_idx + mon_row][col_idx + (19 - mon_col)] {
                            monster_found = false;
                            break;
                        }
                    }
                    if monster_found {
                        println!("FOUND ONE! FLIPPED");
                        monsters_found += 1;
                    }
                }
            }
        }
        // total we're looking for is the count of all TRUE cells, minus the number of found monster cells
        let total_octothorpe: usize = image
            .iter()
            .map(|row| row.iter().filter(|&val| *val).count())
            .sum();
        total_octothorpe - (15 * monsters_found)
    }
}

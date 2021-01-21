use std::fmt;

#[derive(Debug, Clone)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Debug, Clone)]
pub enum Flip {
    NoFlip,
    Horiz,
    Vert,
}

#[derive(Debug, Clone)]
pub struct Transform {
    rotation: Rotation,
    flip: Flip,
}

#[derive(Clone, Copy)]
pub struct SideVals {
    // Flipped just reverses these values
    normal: u32,  // 1010011
    inverse: u32, // 1100101
}

impl SideVals {
    fn matches(&self, other: &SideVals) -> Option<bool> {
        if self.normal == other.inverse {
            Some(false)
        } else if self.normal == other.normal {
            Some(true)
        } else {
            None
        }
    }

    fn inverse(&self) -> SideVals {
        SideVals {
            normal: self.inverse,
            inverse: self.normal,
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

#[derive(Clone)]
pub struct Tile {
    pub id: usize,
    pub data: [[bool; 10]; 10], // Raw data at no rotation, no flip
    all: [SideVals; 4],
    pub linked_ids: [usize; 4],
}

impl Tile {
    pub fn new(id: usize, data: [[bool; 10]; 10]) -> Tile {
        // calculate the numerical representations of each side, calculated clockwise
        /*
        001    North: 1, East: 4, South: 1, West: 6
        100
        100
         */
        let north = to_numbers(&data[0].to_vec());
        let south = to_numbers(&data[9].to_vec()).inverse();
        let east = to_numbers(&data.iter().map(|row| row[9]).collect());
        let west = to_numbers(&data.iter().map(|row| row[0]).collect()).inverse();

        // to make sure the sidevals don't change when you rotate, South and West need to be inverted
        Tile {
            id: id,
            data: data,
            all: [north, east, south, west],
            linked_ids: [0; 4],
        }
    }

    pub fn match_any_side(&self, other: &Tile) -> Option<(usize, Transform)> {
        for (idx, side) in self.all.iter().enumerate() {
            for (other_idx, other_side) in other.all.iter().enumerate() {
                match side.matches(other_side) {
                    None => (),
                    Some(do_flip) => {
                        // rotation needs to put the other tile so its opposite face
                        // matches our tile's face
                        // self: 0 means other: 2
                        let target_side = (idx + 2) % 4;
                        let diff = if target_side > other_idx {
                            target_side - other_idx
                        } else {
                            target_side + 4 - other_idx
                        };
                        let rotation = match diff {
                            0 => Rotation::Deg0,
                            1 => Rotation::Deg90,
                            2 => Rotation::Deg180,
                            3 => Rotation::Deg270,
                            4 => Rotation::Deg0,
                            _ => panic!("WRONG INDEX DIFF!!!"),
                        };
                        return Some((
                            idx,
                            Transform {
                                rotation: rotation,
                                flip: if do_flip {
                                    if idx == 0 || idx == 2 {
                                        Flip::Horiz
                                    } else {
                                        Flip::Vert
                                    }
                                } else {
                                    Flip::NoFlip
                                },
                            },
                        ));
                    }
                }
            }
        }
        None
    }
    pub fn get_matched_tile(&self, other: &Tile) -> Option<(usize, Tile)> {
        match self.match_any_side(other) {
            None => None,
            Some((my_side, transform)) => Some((my_side, other.apply(transform))),
        }
    }

    fn apply(&self, transform: Transform) -> Tile {
        let mut rot_data = [[false; 10]; 10];

        // Apply rotation first
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                match &transform.rotation {
                    Rotation::Deg0 => rot_data[row_idx][col_idx] = *val,
                    Rotation::Deg90 => rot_data[col_idx][9 - row_idx] = *val,
                    Rotation::Deg180 => rot_data[9 - row_idx][9 - col_idx] = *val,
                    Rotation::Deg270 => rot_data[9 - col_idx][row_idx] = *val,
                }
            }
        }

        // Then apply flip in the appropriate direction
        let mut new_data = [[false; 10]; 10];
        for (row_idx, row) in rot_data.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                match &transform.flip {
                    Flip::NoFlip => new_data[row_idx][col_idx] = *val,
                    Flip::Vert => new_data[9 - row_idx][col_idx] = *val,
                    Flip::Horiz => new_data[row_idx][9 - col_idx] = *val,
                }
            }
        }

        Tile::new(self.id, new_data)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!(
            "ID: {} N: {} E: {} S: {} W: {}\n{:?}\n",
            self.id, self.all[0], self.all[1], self.all[2], self.all[3], self.linked_ids
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

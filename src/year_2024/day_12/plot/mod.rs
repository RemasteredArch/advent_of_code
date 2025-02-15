use std::{fmt::Display, hash::Hash};

use crate::Integer;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Plot {
    grid: Box<[Box<[Plant]>]>,
    columns: usize,
    rows: usize,
}

impl Plot {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let mut grid = vec![];

        let mut columns = None;

        for line in input.lines() {
            let mut row = vec![];

            for char in line.chars() {
                row.push(Plant::new(char).ok_or(ParseError::InvalidPlant)?);
            }

            match columns {
                Some(num) => {
                    if num != row.len() {
                        return Err(ParseError::UnevenGrid);
                    }
                }
                None => columns = Some(row.len()),
            }

            grid.push(row.into_boxed_slice());
        }

        let rows = grid.len();
        let columns = columns.unwrap_or(0);

        if rows < 1 || columns < 1 {
            return Err(ParseError::EmptyGrid);
        }

        Ok(Self {
            grid: grid.into_boxed_slice(),
            columns,
            rows,
        })
    }

    pub fn fencing_quote(&self) -> Integer {
        struct Grid {
            grid: Box<[Box<[Plant]>]>,
            regions: Vec<(Integer, Integer)>,
        }

        impl Grid {
            pub fn get(&self, coordinates: Coordinates) -> Option<Plant> {
                self.grid
                    .get(coordinates.row)?
                    .get(coordinates.column)
                    .copied()
            }

            fn get_mut(&mut self, coordinates: Coordinates) -> Option<&mut Plant> {
                self.grid
                    .get_mut(coordinates.row)?
                    .get_mut(coordinates.column)
            }

            fn null(&mut self, coordinates: Coordinates) {
                if let Some(plant) = self.get_mut(coordinates) {
                    *plant = Plant::NULL;
                }
            }

            pub fn visit(&mut self, coordinates: Coordinates) {
                let plant = match self.get(coordinates) {
                    Some(plant) if plant != Plant::NULL => plant,
                    _ => return,
                };

                self.regions.push((0, 0));
                self.visit_impl(plant, coordinates);
            }

            /// Returns `true` if the plant at the `coordinates` matches `region_type`.
            ///
            /// Adds to [`Self::regions`].
            fn visit_impl(&mut self, region_type: Plant, coordinates: Coordinates) -> bool {
                // Escape if plant at `coordinates` is non-matching, otherwise mark it as visited
                // and proceed.
                if self
                    .get(coordinates)
                    .is_none_or(|plant| plant != region_type)
                {
                    return false;
                }
                self.null(coordinates);

                let non_matching_edges = Direction::all()
                    .iter()
                    .filter(|&&edge| {
                        eprintln!(
                            "=> {region_type} @ {coordinates} + {edge:?} => {:?}{}",
                            coordinates.step(edge),
                            coordinates
                                .step(edge)
                                .ok()
                                .and_then(|next_coordinates| self.get(next_coordinates))
                                .map_or("", |next_plant| if next_plant == Plant::NULL {
                                    ", (null)"
                                } else if next_plant != region_type {
                                    ", (non matching)"
                                } else {
                                    ""
                                })
                        );
                        let next_coordinates = match coordinates.step(edge) {
                            Ok(next_coordinates) => next_coordinates,
                            Err(AddError::OutOfBounds) => return true,
                            Err(AddError::Overflow) => {
                                panic!("overflowed while attempted to advance coordinates")
                            }
                        };

                        !self.visit_impl(region_type, next_coordinates)
                    })
                    .count();

                let region = self
                    .regions
                    .last_mut()
                    .expect("`Self::visit` includes a `push`");

                eprintln!("{region_type} @ {coordinates}: area {} + 1 = {}, perimeter {} + {non_matching_edges} = {}",
                    region.0,
                    region.0 + 1,
                    region.1,
                    region.1 + non_matching_edges as Integer,
                );

                *region = (
                    // Area
                    region.0 + 1,
                    // Perimeter
                    region.1 + non_matching_edges as Integer,
                );

                true
            }
        }

        let mut grid = Grid {
            grid: self.grid.clone(),
            regions: vec![],
        };

        for row_index in 0..self.rows {
            for column_index in 0..self.columns {
                grid.visit(Coordinates::new(column_index, row_index));
            }
        }

        dbg!(grid.regions)
            .iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum()
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// When the grid is not at least one row and one column.
    EmptyGrid,
    /// When a rows in the grid has a different length than the first row.
    UnevenGrid,
    /// When a character fails to pass [`Plant::new`].
    InvalidPlant,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Plant {
    char: char,
}

impl Plant {
    /// A value reserved to represent popped characters.
    pub const NULL: Self = Self { char: '0' };

    pub const fn new(char: char) -> Option<Self> {
        if char == Self::NULL.get() {
            return None;
        }

        Some(Self { char })
    }

    pub const fn get(self) -> char {
        self.char
    }
}

impl Display for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates {
    pub column: usize,
    pub row: usize,
}

impl Coordinates {
    pub const fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    pub fn step(&self, direction: Direction) -> Result<Self, AddError> {
        // This is nasty. There's got to be a better way!
        fn add(unsigned: usize, signed: isize) -> Result<usize, AddError> {
            let as_signed: isize = unsigned.try_into().map_err(|_| AddError::Overflow)?;
            if as_signed.checked_add(signed).is_none_or(|v| v < 0) {
                return Err(AddError::OutOfBounds);
            }

            unsigned
                .checked_add_signed(signed)
                .ok_or(AddError::Overflow)
        }

        let (column, row) = match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };

        Ok(Self {
            column: add(self.column, column)?,
            row: add(self.row, row)?,
        })
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.column, self.row)
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum AddError {
    OutOfBounds,
    Overflow,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const fn all() -> [Self; 4] {
        [Self::North, Self::South, Self::East, Self::West]
    }
}

use std::{collections::HashMap, hash::Hash};

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
                row.push(Plant::new(char));
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
        // `<Plant, (Area, Perimeter)>`
        let mut plant_areas_perimters = HashMap::<Plant, (Integer, Integer)>::new();

        let mut increment_or_insert = |plant: &Plant, perimeter: Integer| {
            match plant_areas_perimters.get_mut(plant) {
                Some(plant) => {
                    // Area
                    plant.0 += 1;
                    // Perimeter
                    plant.1 += perimeter;
                }
                None => {
                    plant_areas_perimters.insert(*plant, (1, perimeter));
                }
            };
        };

        for (row_index, row) in self.grid.iter().enumerate() {
            for (column_index, plant) in row.iter().enumerate() {
                increment_or_insert(
                    plant,
                    self.perimeter(Coordinates::new(column_index, row_index))
                        .expect("`.enumerate` should return in-bounds indices"),
                );
            }
        }

        plant_areas_perimters
            .values()
            .map(|(area, perimeter)| area * perimeter)
            .sum()
    }

    /// Get the number of edges exposed to nothing or a different type of plant.
    fn perimeter(&self, plant: Coordinates) -> Option<Integer> {
        fn type_at_edge(plot: &Plot, plant: Coordinates, edge: Direction) -> Option<Plant> {
            let plant = plant.step(edge).ok()?;

            Some(*plot.grid.get(plant.row)?.get(plant.column)?)
        }

        let plant_type = *self.grid.get(plant.row)?.get(plant.column)?;

        let non_matching_edges = Direction::all()
            .iter()
            .filter(|&&direction| {
                // Keep only if there is no plant at that edge (i.e., that's the edge of the plot)
                // or the plant does not match the current plant.
                type_at_edge(self, plant, direction).is_none_or(|t| t != plant_type)
            })
            .count();

        Some(non_matching_edges as Integer)
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyGrid,
    UnevenGrid,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Plant {
    char: char,
}

impl Plant {
    pub const fn new(char: char) -> Self {
        Self { char }
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

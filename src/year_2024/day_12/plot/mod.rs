mod grid;
mod places;

use grid::{BulkGrid, StandardGrid};
use places::{Coordinates, Plant};

use crate::Integer;

use std::hash::Hash;

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
        let mut grid = StandardGrid::new(&self.grid);

        for row_index in 0..self.rows {
            for column_index in 0..self.columns {
                grid.visit(Coordinates::new(column_index, row_index));
            }
        }

        grid.regions()
            .iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum()
    }

    pub fn fencing_quote_bulk(&self) -> Integer {
        let mut grid = BulkGrid::new(&self.grid);

        for row_index in 0..self.rows {
            for column_index in 0..self.columns {
                grid.visit(Coordinates::new(column_index, row_index));
            }
        }

        dbg!(grid.into_regions())
            .iter()
            .map(|(area, edges)| area * edges)
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

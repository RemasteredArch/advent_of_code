#![allow(unused)]

use std::str::CharIndices;

/// A grid of characters. Every line is guaranteed to be of the same length.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Grid {
    // Using [`Box`] instead of [`std::rc::Rc`] because [`Self::get_row`] is the only option that
    // could benefit.
    grid: Box<[Box<str>]>,
    columns: usize,
    rows: usize,
}

impl Grid {
    pub fn new(input: &str) -> Option<Self> {
        let init: Option<usize> = None;

        // Check that every row is of the same length and get that length.
        let columns = input.lines().try_fold(None, |prev, row| match prev {
            None => Some(Some(row.len())),
            Some(prev_len) => {
                if prev_len == row.len() {
                    Some(Some(prev_len))
                } else {
                    None
                }
            }
        })??;

        let rows = input.lines().count();
        let grid = input.lines().map(Into::into).collect();

        Some(Self {
            grid,
            rows,
            columns,
        })
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn char(&self, index: GridIndex) -> Option<char> {
        self.row(index.row())?.chars().nth(index.column())
    }

    pub fn row(&self, index: usize) -> Option<Box<str>> {
        self.grid.get(index).cloned()
    }

    pub fn column(&self, index: usize) -> Option<Box<str>> {
        self.grid.iter().map(|row| row.chars().nth(index)).collect()
    }

    pub fn directional(
        &self,
        index: GridIndex,
        length: usize,
        direction: Direction,
    ) -> Option<Box<str>> {
        let mut str = String::new();
        let mut index = index;

        // Handling one outside of the for loop, because stepping the index could error.
        str.push(self.char(index)?);
        for _ in { 1..length } {
            index = index.step(1, direction)?;
            str.push(self.char(index)?);
        }

        Some(str.into())
    }

    pub fn char_indices(&self) -> impl Iterator<Item = (GridIndex, char)> + use<'_> {
        // Assumes that these will never change.
        let max_column_index = self.columns() - 1;
        let max_row_index = self.rows() - 1;

        let mut rows = self.grid.iter().enumerate().map(move |(row_index, row)| {
            row.char_indices().map(move |(column_index, char)| {
                (
                    GridIndex::new(column_index, row_index, max_column_index, max_row_index)
                        .expect("`self.grid.iter()` will not exceed the bounds of `self.grid`"),
                    char,
                )
            })
        });

        rows.flatten()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GridIndex {
    column: usize,
    row: usize,
    max_row: usize,
    max_column: usize,
}

impl GridIndex {
    pub fn new(column: usize, row: usize, max_column: usize, max_row: usize) -> Option<Self> {
        if column <= max_column && row <= max_row {
            Some(Self {
                column,
                row,
                max_row,
                max_column,
            })
        } else {
            None
        }
    }

    pub fn from_grid(column: usize, row: usize, grid: &Grid) -> Option<Self> {
        Self::new(column, row, grid.columns() - 1, grid.rows() - 1)
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn max_row(&self) -> usize {
        self.max_row
    }

    pub fn max_column(&self) -> usize {
        self.max_column
    }

    pub fn step_row(&self, step: isize) -> Option<Self> {
        let next = self.row().wrapping_add_signed(step);

        if next <= self.max_row() {
            Self::new(self.column, next, self.max_column, self.max_row)
        } else {
            None
        }
    }

    pub fn step_column(&self, step: isize) -> Option<Self> {
        let next = self.column().checked_add_signed(step)?;

        if next <= self.max_column() {
            Self::new(next, self.row, self.max_column, self.max_row)
        } else {
            None
        }
    }

    pub fn step(&self, steps: usize, direction: Direction) -> Option<Self> {
        // Parameter is `usize` to avoid negative values, for which [`Direction::reverse`] should
        // be used instead.
        let steps = steps as isize;

        match direction {
            Direction::North => self.step_row(-steps),
            Direction::South => self.step_row(steps),
            Direction::East => self.step_column(steps),
            Direction::West => self.step_column(-steps),
            Direction::Northeast => self.step_row(-steps)?.step_column(steps),
            Direction::Northwest => self.step_row(-steps)?.step_column(-steps),
            Direction::Southeast => self.step_row(steps)?.step_column(steps),
            Direction::Southwest => self.step_row(steps)?.step_column(-steps),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl Direction {
    pub fn reverse(&self) -> Self {
        use Direction::*;

        match self {
            North => South,
            East => West,
            South => North,
            West => East,
            Northeast => Southwest,
            Northwest => Southeast,
            Southeast => Northwest,
            Southwest => Northeast,
        }
    }
}

#[cfg(test)]
mod test;

use std::{collections::HashSet, fmt::Display};

use crate::Integer;

const MIN_HEIGHT: u8 = 0;
const MAX_HEIGHT: u8 = 9;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Island {
    grid: Box<[Box<[Height]>]>,
    columns: usize,
    rows: usize,
}

impl Island {
    pub fn parse(input: &str) -> Option<Self> {
        let mut grid = vec![];

        let mut columns = None;

        for line in input.lines() {
            let mut row = vec![];

            for char in line.chars() {
                row.push(Height::new(char.to_digit(10)?.try_into().ok()?)?);
            }

            match columns {
                Some(num) => {
                    if num != row.len() {
                        return None;
                    }
                }
                None => columns = Some(row.len()),
            }

            grid.push(row.into_boxed_slice());
        }

        let rows = grid.len();
        let columns = columns?;

        if rows < 1 || columns < 1 {
            return None;
        }

        Some(Self {
            grid: grid.into_boxed_slice(),
            columns,
            rows,
        })
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "used in tests"))]
    pub fn new(input: Vec<Vec<u8>>) -> Option<Self> {
        let mut grid = vec![];

        let mut columns = None;

        for row in input {
            let mut new_row = vec![];

            for column in row {
                new_row.push(Height::new(column)?);
            }

            match columns {
                Some(num) => {
                    if num != new_row.len() {
                        return None;
                    }
                }
                None => columns = Some(new_row.len()),
            }

            grid.push(new_row.into_boxed_slice());
        }

        let columns = columns?;
        let rows = grid.len();

        Some(Self {
            grid: grid.into_boxed_slice(),
            columns,
            rows,
        })
    }

    pub fn first_row(&self) -> Box<[Position]> {
        let row_index = 0;

        self.grid
            .first()
            .expect("`new` guarantees `len >= 1`")
            .iter()
            .enumerate()
            .map(|(column_index, &height)| {
                Position::new(Coordinates::new(column_index, row_index), height)
            })
            .collect()
    }

    pub fn first_column(&self) -> Box<[Position]> {
        let column_index = 0;

        self.grid
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                Position::new(
                    Coordinates::new(column_index, row_index),
                    *row.first().expect("`new` guarantees `len >= 1`"),
                )
            })
            .collect()
    }

    pub fn last_row(&self) -> Box<[Position]> {
        let row_index = self.rows - 1;

        self.grid
            .last()
            .expect("`new` guarantees `len >= 1`")
            .iter()
            .enumerate()
            .map(|(column_index, &height)| {
                Position::new(Coordinates::new(column_index, row_index), height)
            })
            .collect()
    }

    pub fn last_column(&self) -> Box<[Position]> {
        let column_index = self.columns - 1;

        self.grid
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                Position::new(
                    Coordinates::new(column_index, row_index),
                    *row.last().expect("`new` guarantees `len >= 1`"),
                )
            })
            .collect()
    }

    pub fn edges(&self) -> [Box<[Position]>; 4] {
        [
            self.first_column(),
            self.first_row(),
            self.last_column(),
            self.last_row(),
        ]
    }

    /// All [`Coordinates`] along the edges of [`Self`] with a [`Height`] of `0`.
    pub fn trailheads(&self) -> HashSet<Coordinates> {
        let mut positions = HashSet::new();

        for position in self.edges().iter().flatten() {
            if position.height().get() == 0 {
                positions.insert(position.coordinates());
            }
        }

        positions
    }

    pub fn get(&self, coordinates: Coordinates) -> Option<Height> {
        self.grid
            .get(coordinates.row)
            .and_then(|row| row.get(coordinates.column))
            .copied()
    }

    pub fn sum_all_trails(&self) -> Integer {
        self.trailheads()
            .iter()
            .map(|&trailhead| self.sum_trails(Position::new(trailhead, Height::default())))
            .sum()
    }

    fn sum_trails(&self, trailhead: Position) -> Integer {
        let next_height = trailhead.height().get() + 1;

        Direction::all()
            .iter()
            .filter_map(|direction| {
                let coordinates = trailhead.coordinates().step(*direction).ok()?;

                if self.get(coordinates)?.get() == next_height {
                    Some(coordinates)
                } else {
                    None
                }
            })
            .map(|coordinates| {
                self.sum_trail_impl(
                    coordinates,
                    trailhead.coordinates(),
                    Integer::from(trailhead.height().get()),
                )
            })
            .sum()
    }

    fn sum_trail_impl(
        &self,
        coordinates: Coordinates,
        previous: Coordinates,
        sum: Integer,
    ) -> Integer {
        let Some(height) = self.get(coordinates) else {
            return sum;
        };

        let next_height = height.get() + 1;

        Direction::all()
            .iter()
            .filter_map(|direction| {
                let coordinates = coordinates.step(*direction).ok()?;

                if self.get(coordinates)?.get() == next_height && coordinates != previous {
                    Some(coordinates)
                } else {
                    None
                }
            })
            .map(|next_coordinates| {
                self.sum_trail_impl(
                    next_coordinates,
                    coordinates,
                    sum + Integer::from(height.get()),
                )
            })
            .sum()
    }
}

impl Display for Island {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = self
            .grid
            .iter()
            .map(|row| {
                let mut str = row.iter().map(ToString::to_string).collect::<String>();
                str.push('\n');
                str
            })
            .collect::<String>();

        assert_eq!(
            str.pop()
                .expect("the last character of every line is a newline"),
            '\n'
        );

        write!(f, "{str}")
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    coordinates: Coordinates,
    height: Height,
}

impl Position {
    pub const fn new(coordinates: Coordinates, height: Height) -> Self {
        Self {
            coordinates,
            height,
        }
    }

    pub const fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    pub const fn height(&self) -> Height {
        self.height
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Height {
    height: u8,
}

impl Height {
    pub const fn new(height: u8) -> Option<Self> {
        #[allow(clippy::absurd_extreme_comparisons, reason = "allows it to be changed")]
        if height < MIN_HEIGHT || height > MAX_HEIGHT {
            return None;
        }

        Some(Self { height })
    }

    pub const fn get(self) -> u8 {
        self.height
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.height)
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates {
    column: usize,
    row: usize,
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

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum AddError {
    OutOfBounds,
    Overflow,
}

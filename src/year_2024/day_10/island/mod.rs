mod position;
#[cfg(test)]
mod test;

use position::{Coordinates, Direction, Height, Position};

use crate::Integer;

use std::{collections::HashSet, fmt::Display};

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
            .map(|&trailhead| self.sum_trail(Position::new(trailhead, Height::default())))
            .sum()
    }

    fn sum_trail(&self, trailhead: Position) -> Integer {
        let next_height = trailhead.height().get() + 1;

        Direction::all()
            .iter()
            .filter_map(|&direction| {
                let coordinates = trailhead.coordinates().step(direction).ok()?;

                if self.get(coordinates)?.get() == next_height {
                    Some(coordinates)
                } else {
                    None
                }
            })
            .map(|coordinates| self.sum_trail_impl(coordinates, trailhead.coordinates()))
            .fold(HashSet::new(), |accumulated, next_set| {
                accumulated.union(&next_set).copied().collect()
            })
            .len()
            .try_into()
            .expect("a stack overflow will probably occur long before `usize` overflows `Integer`")
    }

    fn sum_trail_impl(
        &self,
        coordinates: Coordinates,
        previous: Coordinates,
    ) -> HashSet<Coordinates> {
        let Some(height) = self.get(coordinates) else {
            return HashSet::default();
        };

        if height.get() == Height::MAX {
            return HashSet::from([coordinates]);
        }

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
            .map(|next_coordinates| self.sum_trail_impl(next_coordinates, coordinates))
            .fold(HashSet::new(), |accumulated, next_set| {
                accumulated.union(&next_set).copied().collect()
            })
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

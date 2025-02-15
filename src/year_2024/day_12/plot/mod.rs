mod places;

use places::{AddError, Coordinates, Direction, Plant};

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
        let mut grid = Grid::new(&self.grid);

        for row_index in 0..self.rows {
            for column_index in 0..self.columns {
                grid.visit(Coordinates::new(column_index, row_index));
            }
        }

        grid.regions
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

struct Grid<'a> {
    plot: Box<[Box<[Plant]>]>,
    regions: Vec<(Integer, Integer)>,
    original: &'a [Box<[Plant]>],
}

impl<'a> Grid<'a> {
    pub fn new(grid: &'a [Box<[Plant]>]) -> Self {
        Self {
            plot: grid.into(),
            regions: vec![],
            original: grid,
        }
    }

    fn get_impl(grid: &[Box<[Plant]>], coordinates: Coordinates) -> Option<Plant> {
        grid.get(coordinates.row)?.get(coordinates.column).copied()
    }

    pub fn get(&self, coordinates: Coordinates) -> Option<Plant> {
        Self::get_impl(&self.plot, coordinates)
    }

    fn get_mut(&mut self, coordinates: Coordinates) -> Option<&mut Plant> {
        self.plot
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
        match self.get(coordinates) {
            // Matching and unvisited plant, continue.
            Some(plant) if plant == region_type => (),
            // Visited plant; return `true` if it was previously matching, but do not continue.
            Some(plant) if plant == Plant::NULL => {
                return Self::get_impl(self.original, coordinates)
                    .is_some_and(|plant| plant == region_type);
            }
            // No plant or non-matching plant, return `false`.
            _ => return false,
        }

        if let Some(plant) = self.get_mut(coordinates) {
            *plant = Plant::new('!').unwrap();
        }
        self.null(coordinates);

        let non_matching_edges = Direction::all()
            .iter()
            .filter(|&&edge| {
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

        *region = (
            // Area
            region.0 + 1,
            // Perimeter
            region.1 + non_matching_edges as Integer,
        );

        true
    }
}

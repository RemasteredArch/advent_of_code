use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    grid: Grid,
    location: GuardLocation,
}

impl Guard {
    pub fn new(input: &str) -> Option<Self> {
        let (grid, location) = Grid::new(input)?;

        Some(Self { grid, location })
    }

    /// # Errors
    ///
    /// Returns [`None`] if the coordinates as [`isize`] overflow.
    pub fn all_locations(&self) -> Option<Box<[Coord]>> {
        let mut locations = vec![self.coord()];

        let mut next = self.step();
        loop {
            match next {
                Ok(g) => {
                    if !g
                        .coord()
                        .is_within_bounds(self.grid.columns(), self.grid.rows())
                    {
                        break;
                    }

                    locations.push(g.coord());
                    next = g.step();
                }
                Err(AddError::OutOfBounds) => break,
                Err(AddError::Overflow) => return None,
            }
        }

        Some(locations.into_boxed_slice())
    }

    pub fn step(&self) -> Result<Self, AddError> {
        // Cheap clone with [`Rc`].
        let grid = self.grid.clone();

        let next_location = self.location.step()?;

        if self.grid.is_obstacle(next_location.coord()) {
            return Self {
                grid,
                location: self.location.rotate(),
            }
            .step();
        }

        Ok(Self {
            grid,
            location: next_location,
        })
    }

    pub fn coord(&self) -> Coord {
        self.location.coord()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    // This could actually be [`Rc<[Rc<[usize]>]>`] and create [`Coord`]s in functions from the row
    // index.
    obstacles: Rc<[Rc<[Coord]>]>,
    columns: usize,
    rows: usize,
}

impl Grid {
    pub fn new(input: &str) -> Option<(Self, GuardLocation)> {
        let mut lines = input.lines();
        let columns = input.lines().next()?.len();

        // Check that every row is of the same length.
        if lines.any(|s| s.len() != columns) {
            return None;
        }

        let rows = input.lines().count();

        let mut guard = None;

        let mut grid: Vec<Vec<Coord>> = vec![];
        for (line_index, line) in input.lines().enumerate() {
            let mut row = Vec::new();

            for (char_index, char) in line.char_indices() {
                macro_rules! guard {
                    ($direction:ident) => {
                        guard = Some(GuardLocation::new(
                            Coord::new(char_index, line_index),
                            Direction::$direction,
                        ))
                    };
                }

                match char {
                    '.' => continue,
                    '#' => row.push(Coord::new(char_index, line_index)),
                    '^' => guard!(North),
                    'v' => guard!(South),
                    '>' => guard!(East),
                    '<' => guard!(West),
                    _ => return None,
                }
            }

            grid.push(row);
        }

        Some((
            Self {
                obstacles: grid.into_iter().map(Into::into).collect(),
                columns,
                rows,
            },
            guard?,
        ))
    }

    pub fn obstacles(&self) -> Rc<[Rc<[Coord]>]> {
        self.obstacles.clone()
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Whether there is an obstacle at a given [`Coord`].
    pub fn is_obstacle(&self, coord: Coord) -> bool {
        let Some(row) = self.obstacles.get(coord.row()) else {
            return false;
        };

        row.iter().any(|&c| c == coord)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GuardLocation {
    coord: Coord,
    direction: Direction,
}

impl GuardLocation {
    pub fn new(coord: Coord, direction: Direction) -> Self {
        Self { coord, direction }
    }

    pub fn step(&self) -> Result<Self, AddError> {
        Ok(Self {
            coord: self.coord.step(self.direction)?,
            direction: self.direction,
        })
    }

    pub fn rotate(&self) -> Self {
        Self {
            coord: self.coord,
            direction: self.direction.rotate(),
        }
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    column: usize,
    row: usize,
}

impl Coord {
    pub fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    pub fn step(&self, direction: Direction) -> Result<Self, AddError> {
        // This is nasty. There's got to be a better way!
        fn add(unsigned: usize, signed: isize) -> Result<usize, AddError> {
            let as_signed: isize = unsigned.try_into().map_err(|_| AddError::Overflow)?;
            if as_signed < signed {
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

    pub fn is_within_bounds(&self, columns: usize, row: usize) -> bool {
        self.column < columns && self.row < row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.column, self.row)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Rotate 90 degrees.
    pub fn rotate(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddError {
    OutOfBounds,
    Overflow,
}

use std::fmt::Display;

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

/// Represents a span between two [`Coordinates`].
///
/// It is guaranteed to share one location or run along one [`Axis`], such both [`Self::start`] and
/// [`Self::end`] share the same [`Coordinates::column`], the same [`Coordinates::row`], or both.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Span {
    start: Coordinates,
    end: Coordinates,
}

impl Span {
    pub const fn new(start: Coordinates, end: Coordinates) -> Option<Self> {
        if start.column != end.column && start.row != end.row {
            return None;
        }

        Some(Self { start, end })
    }

    /// Measures the direction of an arrow pointing from [`Self::start`] to [`Self::end`]. Returns
    /// [`None`] if [`Self::start`] and [`Self::end`] are at the same [`Coordinates`].
    pub fn direction(&self) -> Option<Direction> {
        match self.start.column.cmp(&self.end.column) {
            std::cmp::Ordering::Less => return Some(Direction::West),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => return Some(Direction::East),
        }

        match self.start.row.cmp(&self.end.row) {
            std::cmp::Ordering::Less => Some(Direction::South),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(Direction::North),
        }
    }

    /// Measures the axis that an arrow point from [`Self::start`] to [`Self::end`] lays along.
    /// Returns [`None`] if [`Self::start`] and [`Self::end`] are at the same [`Coordinates`].
    pub fn axis(&self) -> Option<Axis> {
        if self.start == self.end {
            return None;
        }

        if self.start.column == self.end.column {
            return Some(Axis::Vertical);
        }

        Some(Axis::Horizontal)
    }

    pub fn is_within(&self, location: Coordinates) -> bool {
        let Some(axis) = self.axis() else {
            return self.start == location;
        };

        match axis {
            Axis::Horizontal => (self.start.column..=self.end.column).contains(&location.column),
            Axis::Vertical => (self.start.row..=self.end.row).contains(&location.row),
        }
    }

    pub fn is_adjacent(&self, location: Coordinates) -> bool {
        fn is_adjacent(lhs: Coordinates, rhs: Coordinates, direction: Direction) -> bool {
            lhs.step(direction)
                .is_ok_and(|next_coordinates| next_coordinates == rhs)
        }

        let Some(direction) = self.direction() else {
            return Direction::all()
                .iter()
                .any(|&direction| is_adjacent(self.start, location, direction));
        };

        is_adjacent(self.start, location, direction.flip())
            || is_adjacent(self.end, location, direction)
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

    pub const fn axis(self) -> Axis {
        match self {
            Self::North | Self::South => Axis::Vertical,
            Self::East | Self::West => Axis::Horizontal,
        }
    }

    pub const fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

/// The horizontal or vertical axis that a [`Span`] runs along.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum Axis {
    /// Runs along a row, such that [`Coordinates::column`] changes but [`Coordinates::row`] does
    /// not.
    ///
    /// This is `x` in an `(x, y)` plane.
    Horizontal,
    /// Runs along a column, such that [`Coordinates::row`] changes but [`Coordinates::column`]
    /// does not.
    ///
    /// This is `y` in an `(x, y)` plane.
    Vertical,
}

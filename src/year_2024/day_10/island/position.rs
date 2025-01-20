use std::fmt::Display;

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
    pub const MAX: u8 = 9;
    pub const MIN: u8 = 0;

    pub const fn new(height: u8) -> Option<Self> {
        #[allow(clippy::absurd_extreme_comparisons, reason = "allows it to be changed")]
        if height < Self::MIN || height > Self::MAX {
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

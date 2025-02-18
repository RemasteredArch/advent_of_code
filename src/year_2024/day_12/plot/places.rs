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

/// Represents a span between two [`Coordinates`], with one exposed edge ([`Direction`]).
///
/// It is guaranteed to share one location or run along one [`Axis`], such both [`Self::start`] and
/// [`Self::end`] share the same [`Coordinates::column`], the same [`Coordinates::row`], or both.
/// The [`Axis`] runs perpendicular to [`Self::exposed_edge`].
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Span {
    start: Coordinates,
    end: Coordinates,
    exposed_edge: Direction,
}

impl Span {
    pub fn new(start: Coordinates, end: Coordinates, exposed_edge: Direction) -> Option<Self> {
        if Self::is_diagonal(start, end) {
            return None;
        }

        // Verify that the axis the span runs along, if it runs a distance, is perpendicular to the
        // exposed edge.
        if Self::guess_axis(start, end).is_some_and(|axis| axis == exposed_edge.axis()) {
            return None;
        }

        Some(Self {
            start,
            end,
            exposed_edge,
        })
    }

    /// Create a new [`Self`] between the same two points.
    pub const fn new_no_run(location: Coordinates, exposed_edge: Direction) -> Self {
        Self {
            start: location,
            end: location,
            exposed_edge,
        }
    }

    /// Verify that two points run diagonally, not along an [`Axis`].
    const fn is_diagonal(start: Coordinates, end: Coordinates) -> bool {
        start.column != end.column && start.row != end.row
    }

    /// Return the axis that two [`Coordinates`] run along, or return [`None`] if they are
    /// equal.
    ///
    /// Assumes that the [`Coordinates`] *do* run along an axis, not diagonally.
    fn guess_axis(start: Coordinates, end: Coordinates) -> Option<Axis> {
        if start == end {
            return None;
        }

        if start.column == end.column {
            return Some(Axis::Vertical);
        }

        Some(Axis::Horizontal)
    }

    /// Measures the [`Direction`] of an arrow pointing from [`Self::start`] to [`Self::end`].
    /// Returns [`None`] if [`Self::start`] and [`Self::end`] are at the same [`Coordinates`].
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

    /// Measures the [`Direction`] of an arrow pointing from [`Self::start`] to [`Self::end`].
    /// Returns [`Axis::direction_positive`] if [`Self::start`] and [`Self::end`] are at the same
    /// [`Coordinates`] and thus do not have a [`Direction`] of their own.
    pub fn direction_or_positive(&self) -> Direction {
        self.direction()
            .unwrap_or_else(|| self.axis().direction_positive())
    }

    /// Measures the axis that an arrow point from [`Self::start`] to [`Self::end`] lays along.
    pub const fn axis(&self) -> Axis {
        self.exposed_edge.axis().rotate()
    }

    pub const fn exposed_edge(&self) -> Direction {
        self.exposed_edge
    }

    pub fn contains(&self, location: Coordinates) -> bool {
        match self.axis() {
            Axis::Horizontal => (self.start.column..=self.end.column).contains(&location.column),
            Axis::Vertical => (self.start.row..=self.end.row).contains(&location.row),
        }
    }

    pub fn is_adjacent(&self, location: Coordinates) -> bool {
        fn is_adjacent(lhs: Coordinates, rhs: Coordinates, direction: Direction) -> bool {
            lhs.step(direction)
                .is_ok_and(|next_coordinates| next_coordinates == rhs)
        }

        self.axis()
            .directions()
            .iter()
            .any(|&direction| is_adjacent(self.start, location, direction))
    }

    #[must_use]
    pub fn append(&mut self, location: Coordinates) -> Option<()> {
        if !self.contains(location) && !self.is_adjacent(location) {
            return None;
        }

        self.extend_to(location)
    }

    #[must_use]
    pub fn extend_to(&mut self, location: Coordinates) -> Option<()> {
        if self.contains(location) {
            return Some(());
        }

        eprint!("[extending: {self}  =>  {location}]");

        let direction = self.direction_or_positive();
        let from_start = Self::new(self.start, location, self.exposed_edge)?
            .direction()
            .expect("`contains` should catch overlapping coordinates");

        if from_start == direction {
            self.end = location;
        } else {
            self.start = location;
        }

        eprintln!("  =>  [{self}]");

        Some(())
    }

    pub const fn flip(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
            exposed_edge: self.exposed_edge,
        }
    }

    /// Returns `true` if either either end of `other` is adjacent to or contained within either
    /// end of `self`.
    pub fn is_adjacent_or_contained(&self, other: Self) -> bool {
        let other = other.normalize_direction(other);

        (self.is_adjacent(other.start) || self.contains(other.start))
            || (self.is_adjacent(other.end) || self.contains(other.end))
    }

    /// Return a copy of `other`, flipped if necessary, such that it faces in the same
    /// [`Direction`] as `self`. If `self` has no [`Direction`], `other` is flipped anyways.
    ///
    /// ```text
    /// <---------O   other   O->
    ///    o---->     self    o
    ///
    ///
    /// O-------->    other   <-O
    ///   o---->      self    o
    /// ```
    fn normalize_direction(&self, other: Self) -> Self {
        if self.direction() == other.direction() {
            other
        } else {
            other.flip()
        }
    }

    /// Whether `self` sits on the same [`Axis`]; e.g., two horizontal spans sharing the same
    /// [`Coordinates::row`].
    pub fn linear(&self, other: Self) -> bool {
        if self.axis() != other.axis() {
            return false;
        }

        match self.axis() {
            Axis::Horizontal => self.start.row == other.start.row,
            Axis::Vertical => self.start.column == other.start.column,
        }
    }

    #[must_use]
    pub fn join(&mut self, mut other: Self) -> Option<()> {
        enum Side {
            Left,
            Right,
        }

        const fn further_along(lhs: Coordinates, rhs: Coordinates, direction: Direction) -> Side {
            let is_lhs = match direction {
                Direction::North => lhs.row < rhs.row,
                Direction::South => lhs.row > rhs.row,
                Direction::East => lhs.column > rhs.column,
                Direction::West => lhs.column < rhs.column,
            };

            if is_lhs {
                Side::Left
            } else {
                Side::Right
            }
        }

        if self.exposed_edge() != other.exposed_edge() {
            return None;
        }

        if !self.linear(other) {
            return None;
        }

        if *self == other {
            return Some(());
        }

        eprintln!("    + ({self}) != ({other})!");

        self.is_adjacent_or_contained(other);

        eprintln!("    + adjacent or contained!");

        other = self.normalize_direction(other);

        // `direction = self.direction <or> other.direction <or> default`
        //
        // There has got to be a cleaner syntax for this.
        let direction = self
            .direction()
            .or_else(|| {
                // Bad fallback?
                other.direction()
            })
            .unwrap_or_else(|| self.axis().direction_positive());

        match further_along(self.start, other.start, direction.flip()) {
            Side::Left => (),
            Side::Right => self.start = other.start,
        }

        match further_along(self.end, other.end, direction) {
            Side::Left => (),
            Side::Right => self.end = other.end,
        }

        eprintln!("    + fused! now {self}");

        Some(())
    }

    pub fn len(&self) -> usize {
        let Some(direction) = self.direction() else {
            // Start and end at in the same location, i.e., it only spans one coordinate.
            return 1;
        };

        let (greater, lesser) = match direction {
            Direction::North => (self.start.row, self.end.row),
            Direction::South => (self.end.row, self.start.row),
            Direction::East => (self.end.column, self.start.column),
            Direction::West => (self.start.column, self.end.column),
        };

        greater - lesser + 1
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} (exposed edge {})",
            self.start,
            self.end,
            self.exposed_edge()
        )
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

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => "North",
                Self::South => "South",
                Self::East => "East",
                Self::West => "West",
            },
        )
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

impl Axis {
    pub const fn rotate(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }

    pub const fn directions(self) -> [Direction; 2] {
        match self {
            Self::Vertical => [Direction::North, Direction::South],
            Self::Horizontal => [Direction::East, Direction::West],
        }
    }

    /// Returns the [`Direction`] pointing towards positive values along [`Self`], where `(0, 0)`
    /// positive `x` points to the right and positive `y` downwards:
    ///
    /// ```text
    ///      -y
    ///       ^
    ///       |
    /// -x <--+--> +x
    ///       |
    ///       v
    ///      +y
    /// ```
    pub const fn direction_positive(self) -> Direction {
        match self {
            Self::Horizontal => Direction::East,
            Self::Vertical => Direction::South,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Coordinates, Direction, Span};

    const fn coord(column: usize, row: usize) -> Coordinates {
        Coordinates::new(column, row)
    }

    macro_rules! span {
        ($start_column:expr, $start_row:expr; $end_column:expr, $end_row:expr) => {
            Span::new(
                Coordinates::new($start_column, $start_row),
                Coordinates::new($end_column, $end_row),
                Direction::South,
            )
            .expect("macro receives valid, hard-coded input")
        };
    }

    #[test]
    fn span_contains() {
        assert!(!span!(2, 5; 2, 5).contains(coord(3, 5)));
    }

    #[test]
    fn span_extend() {
        macro_rules! extend {
            [$((
                $start_column:expr, $start_row:expr; $end_column:expr, $end_row:expr; $exposed_edge:ident;
                $coordinates_column:expr, $coordinates_row:expr
                => $expected_start_column:expr, $expected_start_row:expr; $expected_end_column:expr, $expected_end_row:expr; $expected_exposed_edge:ident
            )),+ $(,)?] => {
                $(assert_eq!(
                    {
                        let mut span = Span::new(
                            Coordinates::new($start_column, $start_row),
                            Coordinates::new($end_column, $end_row),
                            Direction::$exposed_edge,
                        )
                        .expect("macro receives valid, hard-coded input");

                        span
                            .extend_to(Coordinates::new($coordinates_column, $coordinates_row))
                            .expect("macro receives valid, hard-coded input");

                        span
                    },
                    Span::new(
                        Coordinates::new($expected_start_column, $expected_start_row),
                        Coordinates::new($expected_end_column, $expected_end_row),
                        Direction::$expected_exposed_edge,
                    )
                    .expect("macro receives valid, hard-coded input")
                ));+
            };
        }

        extend![
            (5, 2; 5, 2; East;   5, 3  =>  5, 2; 5, 3; East),
            (5, 0; 5, 0; North;  4, 0  =>  5, 0; 4, 0; North),
            (2, 1; 2, 1; East;   2, 2  =>  2, 1; 2, 2; East),
            (1, 5; 1, 5; South;  0, 5  =>  1, 5; 0, 5; South),
            (5, 4; 5, 4; East;   5, 5  =>  5, 4; 5, 5; East),
            (0, 1; 0, 1; West;   0, 0  =>  0, 0; 0, 1; West),
            (2, 0; 2, 0; North;  1, 0  =>  2, 0; 1, 0; North),
            (2, 0; 1, 0; North;  3, 0  =>  3, 0; 1, 0; North),
            (4, 0; 4, 0; South;  3, 0  =>  4, 0; 3, 0; South),
            (5, 2; 5, 2; West;   5, 1  =>  5, 1; 5, 2; West),
            (2, 2; 2, 2; South;  1, 2  =>  2, 2; 1, 2; South),
            (5, 5; 5, 5; South;  4, 5  =>  5, 5; 4, 5; South),
            (4, 3; 4, 3; North;  3, 3  =>  4, 3; 3, 3; North),
            (3, 4; 3, 4; West;   3, 3  =>  3, 3; 3, 4; West),
            (1, 5; 1, 5; North;  2, 5  =>  2, 5; 1, 5; North),
            (1, 5; 0, 5; South;  2, 5  =>  2, 5; 0, 5; South),
            (2, 5; 0, 5; South;  3, 5  =>  3, 5; 0, 5; South),
        ];
    }
}

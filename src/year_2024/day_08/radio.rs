use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Radios {
    radios: HashMap<Frequency, Vec<Location>>,
    columns: usize,
    rows: usize,
}

impl Radios {
    pub fn parse(input: &str) -> Option<Self> {
        // Is this `O(n)`? Should it be handled in the for loop?
        let rows = input.lines().count();
        let columns = input.lines().next()?.len();

        let mut radios: HashMap<Frequency, Vec<Location>> = HashMap::new();

        for (row_index, line) in input.lines().enumerate() {
            for (column_index, char) in line.char_indices() {
                // Expecting all rows to be of the same length.
                if column_index > columns - 1 {
                    return None;
                }

                let Some(frequency) = Frequency::new(char) else {
                    continue;
                };

                let radio = Location::new(column_index, row_index);

                match radios.get_mut(&frequency) {
                    Some(vec) => {
                        vec.push(radio);
                    }
                    None => {
                        radios.insert(frequency, vec![radio]);
                    }
                };
            }
        }

        Some(Self {
            radios,
            columns,
            rows,
        })
    }

    pub fn antinodes(&self) -> HashSet<Location> {
        let mut locations = HashSet::new();
        for radios in self.radios.values() {
            for radio in radios {
                for other in radios {
                    if radio == other {
                        continue;
                    }

                    let mut bounds_check_and_insert = |antinode: Option<Location>| {
                        if let Some(antinode) =
                            antinode.filter(|a| a.is_in_bounds(self.columns, self.rows))
                        {
                            locations.insert(antinode);
                        }
                    };

                    let (left, right) = radio.antinodes(other);
                    bounds_check_and_insert(left);
                    bounds_check_and_insert(right);
                }
            }
        }

        locations
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Location {
    column: usize,
    row: usize,
}

impl Location {
    pub fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    pub fn is_in_bounds(&self, columns: usize, rows: usize) -> bool {
        self.column() < columns && self.row() < rows
    }

    pub fn antinodes(&self, other: &Self) -> (Option<Location>, Option<Location>) {
        (self.antinode(other), other.antinode(self))
    }

    /// Get the [`Location`] on the opposite side of `other` from `self`.
    fn antinode(&self, other: &Self) -> Option<Self> {
        let (rise, run) = self.rise_run(other);

        let column = self.column().checked_add_signed(run)?;
        let row = self.row().checked_add_signed(rise)?;

        Some(Self { column, row })
    }

    pub fn rise_run(&self, other: &Self) -> (isize, isize) {
        let rise = other.row() as isize - self.row() as isize;
        let run = other.column() as isize - self.column() as isize;

        (rise, run)
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Frequency {
    frequency: char,
}

impl Frequency {
    pub fn new(frequency: char) -> Option<Self> {
        if !frequency.is_ascii_alphanumeric() {
            return None;
        }

        Some(Self { frequency })
    }

    pub fn get(&self) -> char {
        self.frequency
    }
}

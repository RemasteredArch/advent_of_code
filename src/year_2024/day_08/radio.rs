use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
};

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

    pub fn from_pairs(input: Vec<(Frequency, Location)>) -> Self {
        let mut radios = HashMap::<Frequency, Vec<Location>>::new();
        let mut columns = 0;
        let mut rows = 0;

        for (frequency, radio) in input.into_iter() {
            columns = columns.max(radio.column() + 1);
            rows = rows.max(radio.row() + 1);

            match radios.get_mut(&frequency) {
                Some(vec) => {
                    vec.push(radio);
                }
                None => {
                    radios.insert(frequency, vec![radio]);
                }
            };
        }

        Self {
            radios,
            columns,
            rows,
        }
    }

    pub fn from_pairs_bounded(
        input: Vec<(Frequency, Location)>,
        columns: usize,
        rows: usize,
    ) -> Option<Self> {
        let radios = Self::from_pairs(input);

        if radios.columns > columns || radios.rows > rows {
            None
        } else {
            Some(Self {
                radios: radios.radios,
                columns,
                rows,
            })
        }
    }

    /// Returns a pair of [`Frequency`] and [`Location`] for every radio in [`Self`].
    ///
    /// The output is not sorted or deduplicated. The sorting is (probably) non-deterministic.
    pub fn radio_pairs(&self) -> Vec<(Frequency, Location)> {
        self.radios
            .iter()
            .map(|(frequency, radios)| {
                radios
                    .iter()
                    .map(move |radio| (*frequency, *radio))
                    .collect::<Vec<_>>()
            })
            .reduce(|mut acculumated, mut next| {
                acculumated.append(&mut next);
                acculumated
            })
            .unwrap_or(vec![])
    }

    pub fn antinode_pairs(&self) -> HashSet<Location> {
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

                    let (left, right) = radio.antinode_pair(other);
                    bounds_check_and_insert(left);
                    bounds_check_and_insert(right);
                }
            }
        }

        locations
    }

    pub fn all_antinodes(&self) -> HashSet<Location> {
        let mut locations = HashSet::new();

        for radios in self.radios.values() {
            for radio in radios {
                for other in radios {
                    if radio == other {
                        continue;
                    }

                    for antinode in radio.all_antinodes(other).iter() {
                        if antinode.is_in_bounds(self.columns, self.rows) {
                            locations.insert(*antinode);
                        }
                    }
                }
            }
        }

        println!(
            "{self}\n\n{}",
            Radios::from_pairs_bounded(
                locations
                    .iter()
                    .map(|&l| (Frequency { frequency: '#' }, l))
                    .collect(),
                self.columns,
                self.rows
            )
            .unwrap()
        );

        locations
    }
}

impl Display for Radios {
    /// Formats [`Self`] as a grid, with locations that don't have radios represented as `'.'`.
    /// Where two radios overlap, the displayed frequency is (probably) non-deterministic.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut radios = self.radio_pairs();
        radios.dedup_by_key(|(_, location)| *location);
        // Reverse order, so the locations closest to (0, 0) are at the end of the array.
        //
        // Sorts by row instead of by column because it will be printed row-by-row instead of
        // column-by-column.
        radios.sort_unstable_by(|(_, left_location), (_, right_location)| {
            right_location.cmp_by_row(left_location)
        });

        let mut output = String::with_capacity((self.columns + 1) * self.rows);

        for row in 0..self.rows {
            for column in 0..self.columns {
                output.push(
                    if radios
                        .last()
                        .is_some_and(|(_, location)| *location == Location::new(column, row))
                    {
                        let (frequency, _) = radios
                            .pop()
                            .expect("use of `.last()` proved the existence of an element");

                        frequency.get()
                    } else {
                        '.'
                    },
                );
            }

            output.push('\n');
        }

        write!(f, "{output}")
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Location {
    column: usize,
    row: usize,
}

impl Location {
    pub const fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    pub fn is_in_bounds(&self, columns: usize, rows: usize) -> bool {
        self.column() < columns && self.row() < rows
    }

    pub fn all_antinodes(&self, other: &Self) -> Vec<Self> {
        fn antinodes_along_line(
            output: &mut Vec<Location>,
            mut last: Location,
            mut second_last: Location,
        ) {
            while let Some(next) = second_last.antinode(&last) {
                output.push(next);

                second_last = last;
                last = next;
            }
        }

        let mut locations = vec![];

        let Some(first) = self.antinode(other) else {
            return locations;
        };
        locations.push(first);

        antinodes_along_line(&mut locations, first, *other);
        dbg!(&locations);
        antinodes_along_line(&mut locations, *self, *other);
        dbg!(&locations);

        locations
    }

    pub fn antinode_pair(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        (self.antinode(other), other.antinode(self))
    }

    /// Get the [`Location`] on the opposite side of `other` from `self`.
    fn antinode(&self, other: &Self) -> Option<Self> {
        let (rise, run) = self.rise_run(other);

        let column = self.column().checked_add_signed(-run)?;
        let row = self.row().checked_add_signed(-rise)?;

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

    pub fn cmp_by_column(&self, rhs: &Self) -> Ordering {
        self.cmp(rhs)
    }

    pub fn cmp_by_row(&self, rhs: &Self) -> Ordering {
        let row_ordering = self.row().cmp(&rhs.row());

        match row_ordering {
            Ordering::Less | Ordering::Greater => row_ordering,
            Ordering::Equal => self.column().cmp(&rhs.column()),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.column(), self.row())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

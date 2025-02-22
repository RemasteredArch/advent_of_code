use std::collections::HashMap;

use super::{
    super::LARGE_EXAMPLE_INPUT,
    grid::BulkGrid,
    places::{Coordinates, Direction, Span},
    Plot,
};

use crate::Integer;

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
        (5, 0; 5, 0; North;  4, 0  =>  4, 0; 5, 0; North),
        (2, 0; 2, 0; North;  1, 0  =>  1, 0; 2, 0; North),
        (2, 0; 1, 0; North;  3, 0  =>  3, 0; 1, 0; North),
        (4, 3; 4, 3; North;  3, 3  =>  3, 3; 4, 3; North),
        (1, 5; 1, 5; North;  2, 5  =>  1, 5; 2, 5; North),
        (1, 5; 1, 5; South;  0, 5  =>  0, 5; 1, 5; South),
        (4, 0; 4, 0; South;  3, 0  =>  3, 0; 4, 0; South),
        (2, 2; 2, 2; South;  1, 2  =>  1, 2; 2, 2; South),
        (5, 5; 5, 5; South;  4, 5  =>  4, 5; 5, 5; South),
        (1, 5; 0, 5; South;  2, 5  =>  2, 5; 0, 5; South),
        (2, 5; 0, 5; South;  3, 5  =>  3, 5; 0, 5; South),
        (5, 2; 5, 2; East;   5, 3  =>  5, 2; 5, 3; East),
        (2, 1; 2, 1; East;   2, 2  =>  2, 1; 2, 2; East),
        (5, 4; 5, 4; East;   5, 5  =>  5, 4; 5, 5; East),
        (0, 1; 0, 1; West;   0, 0  =>  0, 0; 0, 1; West),
        (5, 2; 5, 2; West;   5, 1  =>  5, 1; 5, 2; West),
        (3, 4; 3, 4; West;   3, 3  =>  3, 3; 3, 4; West),
    ];
}

#[test]
fn span_adjacency() {
    let span_8_0 = Span::new_no_run(coord(8, 0), Direction::West);
    let span_8_4 = Span::new_no_run(coord(8, 4), Direction::West);

    assert!(!span_8_0.is_adjacent(coord(8, 4)));
    assert!(!span_8_0.is_adjacent_or_contained(span_8_4));
    assert!(span_8_0.clone().join(span_8_4).is_none());

    let grid = unsafe {
        BulkGrid::with_regions(vec![(
            10,
            // Two values that should *not* join.
            HashMap::from([
                (coord(8, 0), vec![Direction::West]),
                (coord(8, 4), vec![Direction::West]),
            ]),
        )])
    };
    assert_eq!(grid.into_regions(), vec![(10, 2)]);
}

#[test]
fn span_fuse() {
    let mut span_4_1 = Span::new(coord(4, 0), coord(1, 0), Direction::North).unwrap();
    let span_0_1 = Span::new(coord(0, 0), coord(1, 0), Direction::North).unwrap();

    let span_4_0 = Span::new(coord(4, 0), coord(0, 0), Direction::North).unwrap();

    span_4_1.join(span_0_1).unwrap();

    assert_eq!(span_4_1, span_4_0);
}

#[test]
fn bulk_grid_regions() {
    fn sort(
        lhs: &(Integer, Integer, Integer),
        rhs: &(Integer, Integer, Integer),
    ) -> std::cmp::Ordering {
        macro_rules! cmp {
            ($lhs:expr, $rhs:expr) => {
                let cmp = $lhs.cmp($rhs);

                if matches!(cmp, std::cmp::Ordering::Less | std::cmp::Ordering::Greater) {
                    return cmp;
                }
            };
        }

        cmp!(lhs.0, &rhs.0);
        cmp!(lhs.1, &rhs.1);
        rhs.2.cmp(&rhs.2)
    }

    const EXPECTED_REGIONS: &[(char, Integer, Integer, Integer)] = &[
        ('R', 12, 10, 120), // A region of plant `'R'` with area `2` and `10` sides (costing `120`).
        ('I', 4, 4, 16),    // A region of plant `'I'` with area `4` and `4` sides (costing `16`).
        ('C', 14, 22, 308), // A region of plant `'C'` with area `4` and `22` sides (costing `308`).
        ('F', 10, 12, 120), // A region of plant `'F'` with area `0` and `12` sides (costing `120`).
        ('V', 13, 10, 130), // A region of plant `'V'` with area `3` and `10` sides (costing `130`).
        ('J', 11, 12, 132), // A region of plant `'J'` with area `1` and `12` sides (costing `132`).
        ('C', 1, 4, 4),     // A region of plant `'C'` with area `1` and `4` sides (costing `4`).
        ('E', 13, 8, 104),  // A region of plant `'E'` with area `3` and `8` sides (costing `104`).
        ('I', 14, 16, 224), // A region of plant `'I'` with area `4` and `16` sides (costing `224`).
        ('M', 5, 6, 30),    // A region of plant `'M'` with area `5` and `6` sides (costing `30`).
        ('S', 3, 6, 18),    // A region of plant `'S'` with area `3` and `6` sides (costing `18`).
    ];

    let plot = Plot::parse(LARGE_EXAMPLE_INPUT).unwrap();

    let mut grid = super::grid::BulkGrid::new(&plot.grid);

    for row_index in 0..plot.rows {
        for column_index in 0..plot.columns {
            grid.visit(Coordinates::new(column_index, row_index));
        }
    }

    let mut regions = grid
        .into_regions()
        .into_iter()
        .map(|(area, edges)| (area, edges, area * edges))
        .collect::<Vec<_>>();
    regions.sort_by(sort);

    let mut expected = EXPECTED_REGIONS.to_vec();
    expected.sort_by(|lhs, rhs| sort(&(lhs.1, lhs.2, lhs.3), &(rhs.1, rhs.2, rhs.3)));

    eprintln!("Diff:");
    eprintln!("    Actual:          Expected:");
    for i in 0..regions.len().max(expected.len()) {
        let fmt = |(area, sides, cost): (Integer, Integer, Integer)| {
            format!("({area:2}, {sides:2}, {cost:3})",)
        };

        let actual = regions[i];
        let expected = expected[i];
        let plant = expected.0;
        let expected = (expected.1, expected.2, expected.3);

        match actual.cmp(&expected) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater => {
                eprintln!("! {plant} {} != {}", fmt(actual), fmt(expected));
            }
            std::cmp::Ordering::Equal => {
                eprintln!("  {plant} {} == {}", fmt(actual), fmt(expected));
            }
        };
    }

    assert_eq!(
        regions,
        expected
            .into_iter()
            .map(|(_, area, edges, cost)| (area, edges, cost))
            .collect::<Vec<_>>()
    );
}

#[test]
fn part_one() {
    use super::super::{EXAMPLE_INPUT, LARGE_EXAMPLE_INPUT, SIMPLE_EXAMPLE_INPUT};

    fn part_one(input_expected: &[(&str, Integer)]) {
        for (input, expected) in input_expected {
            assert_eq!(Plot::parse(input).unwrap().fencing_quote(), *expected);
        }
    }

    part_one(&[
        (EXAMPLE_INPUT, 140),
        (LARGE_EXAMPLE_INPUT, 1930),
        (SIMPLE_EXAMPLE_INPUT, 772),
    ]);
}

#[test]
fn part_two() {
    use super::super::{EXAMPLE_INPUT, LARGE_EXAMPLE_INPUT, SIMPLE_EXAMPLE_INPUT_PART_TWO};

    fn part_two(input_expected: &[(&str, Integer)]) {
        for (input, expected) in input_expected {
            assert_eq!(Plot::parse(input).unwrap().fencing_quote_bulk(), *expected);
        }
    }

    part_two(&[
        (EXAMPLE_INPUT, 80),
        (LARGE_EXAMPLE_INPUT, 1206),
        (SIMPLE_EXAMPLE_INPUT_PART_TWO, 368),
    ]);
}

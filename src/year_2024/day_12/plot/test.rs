use super::{
    super::LARGE_EXAMPLE_INPUT,
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

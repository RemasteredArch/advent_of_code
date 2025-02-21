use super::places::{Coordinates, Direction, Span};

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

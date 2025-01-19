use std::collections::HashSet;

use crate::year_2024::day_10::island::{Coordinates, Height, Position};

use super::{super::EXAMPLE_INPUT, Island};

macro_rules! island {
    [ $( $($height:expr),+ ; )+ ] => {
        Island::new(
            ::std::vec![
                $(
                    ::std::vec![ $($height),+ ]
                ),+
            ]
        ).expect("macro should be used with appropriate heights")
    };
}

macro_rules! pos {
    ($column:expr, $row:expr, $height:expr) => {
        Position::new(
            Coordinates::new($column, $row),
            Height::new($height).expect("macro should be used with appropriate heights"),
        )
    };
}

#[test]
fn island_parse_display() {
    let island = Island::parse(EXAMPLE_INPUT).unwrap();

    assert_eq!(EXAMPLE_INPUT, island.to_string());
    assert_eq!(
        island,
        island![
            0, 1, 2, 3;
            1, 2, 3, 4;
            8, 7, 6, 5;
            9, 8, 7, 6;
        ],
    );
}

#[test]
fn edges() {
    let labels = ["first_column", "first_row", "last_column", "last_row"];

    let expected = [
        [pos!(0, 0, 0), pos!(0, 1, 1), pos!(0, 2, 8), pos!(0, 3, 9)], // `first_column`
        [pos!(0, 0, 0), pos!(1, 0, 1), pos!(2, 0, 2), pos!(3, 0, 3)], // `first_row`
        [pos!(3, 0, 3), pos!(3, 1, 4), pos!(3, 2, 5), pos!(3, 3, 6)], // `last_column`
        [pos!(0, 3, 9), pos!(1, 3, 8), pos!(2, 3, 7), pos!(3, 3, 6)], // `last_row`
    ]
    .map(|edge| edge.to_vec().into_boxed_slice());

    let island = island![
        0, 1, 2, 3;
        1, 2, 3, 4;
        8, 7, 6, 5;
        9, 8, 7, 6;
    ];

    let edges = island.edges();

    for (edge_index, edge) in edges.iter().enumerate() {
        for (position_index, position) in edge.iter().enumerate() {
            assert_eq!(
                expected[edge_index][position_index], *position,
                "{}[{position_index}]",
                labels[edge_index],
            );
        }
    }

    assert_eq!(expected, edges);
}

#[test]
fn trailheads() {
    let island = island![
        0, 1, 2, 3;
        1, 2, 3, 4;
        8, 7, 6, 5;
        9, 8, 7, 6;
    ];

    assert_eq!(island.trailheads(), HashSet::from([Coordinates::new(0, 0)]));
}

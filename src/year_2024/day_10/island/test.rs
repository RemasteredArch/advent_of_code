use std::collections::HashSet;

use super::{super::EXAMPLE_INPUT, Coordinates, Island};

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
fn trailheads() {
    let island = island![
        0, 1, 2, 3;
        1, 2, 3, 4;
        8, 7, 6, 5;
        9, 8, 7, 6;
    ];

    assert_eq!(island.trailheads(), HashSet::from([Coordinates::new(0, 0)]));

    let island = island![
        0, 1, 2, 3;
        1, 0, 3, 4;
        8, 7, 6, 5;
        9, 0, 7, 6;
    ];

    assert_eq!(
        island.trailheads(),
        HashSet::from([
            Coordinates::new(0, 0),
            Coordinates::new(1, 1),
            Coordinates::new(1, 3)
        ])
    );
}

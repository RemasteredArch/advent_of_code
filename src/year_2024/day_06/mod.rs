#![allow(unused, dead_code)]

use std::collections::HashSet;

use grid::{Coord, Guard};

mod grid;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub fn part_one() -> u32 {
    part_one_impl(INPUT)
}

fn part_one_impl(input: &str) -> u32 {
    let guard = Guard::new(input).unwrap();

    HashSet::<Coord>::from_iter(
        guard
            .all_locations()
            .expect("reasonably-sized grids won't cause `isize` overflows")
            .iter()
            .copied(),
    )
    .len()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod test {
    use super::{part_one_impl, EXAMPLE_INPUT};

    #[test]
    fn part_one() {
        assert_eq!(41, part_one_impl(EXAMPLE_INPUT));
    }
}

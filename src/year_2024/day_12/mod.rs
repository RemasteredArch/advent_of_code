mod plot;

use plot::Plot;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
/// Expects [`part_one`] to return `140` and [`part_two`] to return `80`.
const EXAMPLE_INPUT: &str = "AAAA
BBCD
BBCC
EEEC";
/// Expects [`part_one`] to return `772`.
const SIMPLE_EXAMPLE_INPUT: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
/// Expects [`part_two`] to return `368`.
const SIMPLE_EXAMPLE_INPUT_PART_TWO: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
/// Expects [`part_one`] to return `1930` and [`part_two`] to return `1206`.
const LARGE_EXAMPLE_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

pub fn part_one() -> Integer {
    let plot = Plot::parse(INPUT).unwrap();

    plot.fencing_quote()
}

pub fn part_two() -> Integer {
    let plot = Plot::parse(INPUT).unwrap();

    plot.fencing_quote_bulk()
}

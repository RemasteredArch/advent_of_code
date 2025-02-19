mod plot;

use plot::Plot;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "AAAA
BBCD
BBCC
EEEC";
const COMPLEX_EXAMPLE_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
/// Expects [`part_two`] to return `368`.
const COMPLEX_EXAMPLE_INPUT_PART_TWO: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
const SIMPLE_EXAMPLE_INPUT: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
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
    let plot = Plot::parse(LARGE_EXAMPLE_INPUT).unwrap();

    plot.fencing_quote_bulk()
}

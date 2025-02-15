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
const SIMPLE_EXAMPLE_INPUT: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

pub fn part_one() -> Integer {
    let plot = Plot::parse(INPUT).unwrap();

    plot.fencing_quote()
}

pub fn part_two() -> Integer {
    todo!("implement part two")
}

mod plot;

use plot::Plot;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "AAAA
BBCD
BBCC
EEEC";

pub fn part_one() -> Integer {
    let plot = Plot::parse(INPUT).unwrap();

    plot.fencing_quote()
}

pub fn part_two() -> Integer {
    todo!("implement part two")
}

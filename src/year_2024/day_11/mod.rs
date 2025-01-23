mod stones;

use stones::Stones;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "125 17";

pub fn part_one() -> Integer {
    let stones = Stones::parse(EXAMPLE_INPUT).unwrap().blink_n(25);

    stones.len() as Integer
}

pub fn part_two() -> Integer {
    let stones = Stones::parse(EXAMPLE_INPUT).unwrap().blink_n(75);

    stones.len() as Integer
}

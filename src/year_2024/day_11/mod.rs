mod stones;

use stones::Stones;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "0 1 10 99 999";

pub fn part_one() -> Integer {
    let mut stones = Stones::parse(EXAMPLE_INPUT).unwrap();

    stones.blink_n(25);

    stones.len() as Integer
}

pub fn part_two() -> Integer {
    todo!("implement part two")
}

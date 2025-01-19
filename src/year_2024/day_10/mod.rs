mod trail;

use trail::Trail;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "0123
1234
8765
9876";

pub fn part_one() -> Integer {
    let trail = Trail::parse(EXAMPLE_INPUT).unwrap();

    trail.sum_all_trails()
}

pub fn part_two() -> Integer {
    todo!("implement part two")
}

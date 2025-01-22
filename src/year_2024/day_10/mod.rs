mod island;

use island::Island;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "0123
1234
8765
9876";

pub fn part_one() -> Integer {
    let island = Island::parse(INPUT).unwrap();

    island.count_all_trail_endpoints()
}

pub fn part_two() -> Integer {
    let island = Island::parse(INPUT).unwrap();

    island.count_all_trails()
}

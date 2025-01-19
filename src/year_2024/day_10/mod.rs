mod trail;

use trail::Island;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "0123
1234
8765
9876";

pub fn part_one() -> Integer {
    let island = Island::parse(EXAMPLE_INPUT).unwrap();

    println!("{island}");

    island.sum_all_trails()
}

pub fn part_two() -> Integer {
    todo!("implement part two")
}

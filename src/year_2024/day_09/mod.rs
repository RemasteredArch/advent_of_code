mod filesystem;

use filesystem::Filesystem;

use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "2333133121414131402";

pub fn part_one() -> Integer {
    let fs = Filesystem::parse(EXAMPLE_INPUT);
    println!("{fs}");

    todo!("finish part one")
}

pub fn part_two() -> Integer {
    todo!("implement part two")
}

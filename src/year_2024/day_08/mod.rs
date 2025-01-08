mod radio;

use radio::Radios;

use crate::Integer;

#[allow(dead_code)]
const INPUT: &str = include_str!("./data.txt");
#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

// - Each antenna is tuned to a frequency
// - A frequency is a single lowercase letter, uppercase letter, or digit
// - An Antinode accepts a particular frequency
// - An antinode occurs on any point on a line with two antennas at equal frequencies, where the
//   second antenna is twice as far away
// - Except where it would overflow off the map, each antenna pair creates two antinodes
// - Antinodes can occur at the same location as another antenna
pub fn part_one() -> Integer {
    let radios = Radios::parse(EXAMPLE_INPUT).unwrap();
    println!("{radios}");

    radios.antinodes().len().try_into().unwrap()
}

pub fn part_two() -> Integer {
    todo!("part two")
}

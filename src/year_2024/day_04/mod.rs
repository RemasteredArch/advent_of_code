mod grid;
#[cfg(test)]
mod test;

const INPUT: &str = include_str!("./data.txt");
const _INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

pub fn part_one() -> u32 {
    const PATTERN: &str = "XMAS";

    grid::Grid::new(INPUT).unwrap().search_all(PATTERN)
}

pub fn part_two() -> u32 {
    grid::Grid::new(INPUT).unwrap().search_all_cross("MAS")
}

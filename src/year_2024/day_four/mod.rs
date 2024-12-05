use grid::{Direction, Grid, GridIndex};

mod grid;
#[cfg(test)]
mod test;

const _INPUT: &str = include_str!("./data.txt");
const INPUT: &str = "MMMSXXMASM
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
    let grid = Grid::new(INPUT).unwrap();
    let index = GridIndex::from_grid(0, 0, &grid).unwrap();
    dbg!(
        &grid,
        index,
        grid.char(index),
        grid.directional(index, 4, Direction::Southeast)
    );
    todo!()
}

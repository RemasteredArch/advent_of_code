use super::grid::{Direction, Grid, GridIndex};

const GRID: &str = "0123
1234
2345
3456";

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_directional() -> Result {
    let grid = Grid::new(GRID).ok_or("failed to construct grid")?;
    let origin = GridIndex::from_grid(0, 0, &grid).ok_or("failed to construct index (0,0)")?;
    let last = GridIndex::from_grid(3, 3, &grid).ok_or("failed to construct index (3,3)")?;

    // Using explicit errors instead of `Ok(_)` for the sake of clearer error messages.
    assert_eq!('0', grid.char(origin).ok_or("failed to retrieve (0, 0)")?);
    assert_eq!('6', grid.char(last).ok_or("failed to retrieve (3, 3)")?);
    assert_eq!(
        '2',
        grid.char(
            origin
                .step(1, Direction::Southeast)
                .ok_or("failed to step (0, 0) -> (1, 1)")?
        )
        .ok_or("failed to retrieve (1, 1)")?
    );

    assert_eq!(
        "0246".to_string().into_boxed_str(),
        grid.directional(origin, 4, Direction::Southeast)
            .ok_or("failed to retrieve (0,0)..=(3, 3)")?
    );
    assert_eq!(
        "6420".to_string().into_boxed_str(),
        grid.directional(last, 4, Direction::Northwest)
            .ok_or("failed to retrieve (0,0)..=(3, 3)")?
    );

    let start_of_third_row =
        GridIndex::from_grid(0, 2, &grid).ok_or("failed to construct index (0,2)")?;
    let end_of_third_row =
        GridIndex::from_grid(3, 2, &grid).ok_or("failed to construct index (3,2)")?;
    assert_eq!(
        Some("2345".to_string().into_boxed_str()),
        grid.directional(start_of_third_row, 4, Direction::East)
    );
    assert_eq!(
        Some("5432".to_string().into_boxed_str()),
        grid.directional(end_of_third_row, 4, Direction::West)
    );

    Ok(())
}

#[test]
fn test_grid_iter() -> Result {
    macro_rules! grid_indices {
        ($grid:expr, $(($column:expr, $row:expr, $char:expr)),+) => {

            vec![
                $((
                    super::grid::GridIndex::from_grid($column, $row, $grid)
                       .ok_or(format!("failed to construct index ({}, {})", $column, $row))?,
                    $char,
                )),+
            ]
        };
    }
    let grid = Grid::new("012\n345").ok_or("failed to construct grid")?;

    assert_eq!(
        {
            println!("lhs\n");
            grid_indices!(
                &grid,
                (0, 0, '0'),
                (1, 0, '1'),
                (2, 0, '2'),
                (0, 1, '3'),
                (1, 1, '4'),
                (2, 1, '5')
            )
        },
        {
            println!("rhs\n");
            grid.char_indices().collect::<Vec<_>>()
        }
    );

    Ok(())
}

#[test]
fn test_cross() -> Result {
    /// Expecting 9 matches:
    ///
    /// ```txt
    /// .M.S......
    /// ..A..MSMS.
    /// .M.S.MAA..
    /// ..A.ASMSM.
    /// .M.S.M....
    /// ..........
    /// S.S.S.S.S.
    /// .A.A.A.A..
    /// M.M.M.M.M.
    /// ..........
    /// ```
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

    assert_eq!(
        9,
        Grid::new(INPUT)
            .ok_or("failed to construct grid")?
            .search_all_cross("MAS")
    );

    Ok(())
}

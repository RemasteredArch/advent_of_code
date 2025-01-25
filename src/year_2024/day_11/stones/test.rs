use super::Stones;

#[test]
fn blink() {
    let mut stones = Stones::parse("125 17").unwrap();

    macro_rules! next_blink {
        ($stones:expr, $expected:expr) => {
            assert_eq!(
                {
                    #[allow(clippy::unreadable_literal)]
                    let mut expected = $expected.map(super::Stone::new);
                    expected.sort();
                    expected.to_vec().into_boxed_slice()
                },
                {
                    $stones.blink_n(1);
                    let mut result = $stones.as_slice();
                    result.sort();
                    result
                }
            )
        };
    }

    next_blink!(stones, [253000, 1, 7]);
    next_blink!(stones, [253, 0, 2024, 14168]);
    next_blink!(stones, [512072, 1, 20, 24, 28676032]);
    next_blink!(stones, [512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
    next_blink!(
        stones,
        [1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
    );
    next_blink!(
        stones,
        [
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3,
            2
        ]
    );

    assert_eq!(22, stones.len());
}

use super::Stones;

#[test]
fn blink() {
    let mut stones = Stones::parse("125 17").unwrap();

    let mut blink = move || {
        stones = stones.blink();
        stones.to_string()
    };

    assert_eq!("253000 1 7", blink());
    assert_eq!("253 0 2024 14168", blink());
    assert_eq!("512072 1 20 24 28676032", blink());
    assert_eq!("512 72 2024 2 0 2 4 2867 6032", blink());
    assert_eq!("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32", blink());

    let last = blink();

    assert_eq!(
        "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
        last
    );

    assert_eq!(22, Stones::parse(&last).unwrap().len());
}

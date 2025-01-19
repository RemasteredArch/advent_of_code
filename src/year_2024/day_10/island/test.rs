use super::{super::EXAMPLE_INPUT, Island};

#[test]
fn island_parse_display() {
    let island = Island::parse(EXAMPLE_INPUT).unwrap();

    assert_eq!(EXAMPLE_INPUT, island.to_string());
}

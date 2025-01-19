use super::{super::EXAMPLE_INPUT, Trail};

#[test]
fn trail_parse_display() {
    let trail = Trail::parse(EXAMPLE_INPUT).unwrap();

    assert_eq!(EXAMPLE_INPUT, trail.to_string());
}


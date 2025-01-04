mod base;
mod equation;

use crate::Integer;
use equation::Equation;

#[allow(dead_code)]
const INPUT: &str = include_str!("./data.txt");
#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

pub fn part_one() -> Integer {
    let equations = parse_input(INPUT).unwrap();

    sum_valid(&equations, Equation::is_valid_binary)
}

pub fn part_two() -> Integer {
    let equations = parse_input(INPUT).unwrap();

    sum_valid(&equations, Equation::is_valid_ternary)
}

fn sum_valid(equations: &[Equation], mut validate: impl FnMut(&Equation) -> bool) -> Integer {
    equations
        .iter()
        .filter_map(move |e| {
            if !validate(e) {
                return None;
            }

            Some(e.expected_value())
        })
        .sum()
}

fn parse_input(input: &str) -> Option<Box<[Equation]>> {
    // Instinctually, I want to instantiate with `Vec::with_capacity(input.lines().count());`, but
    // I assume that's probably `O(n)`.
    let mut equations: Vec<Equation> = vec![];

    // Expecting input like `3267: 81 40 27`.
    for line in input.lines() {
        let (expected_value, inputs) = line.split_once(": ")?;

        let expected_value: Integer = expected_value.parse().ok()?;
        let inputs: Box<[Integer]> = inputs
            .split(' ')
            // Should this ignore errors like this?
            .filter_map(|n| n.parse().ok())
            .collect();

        equations.push(Equation::new(expected_value, inputs));
    }

    Some(equations.into_boxed_slice())
}

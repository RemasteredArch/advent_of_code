// TODO: Make days into traits.
//
// TODO: Proper error handling!

macro_rules! safe_or_break {
    ($safe:ident, $($fn:expr),+) => {
        $(
            $safe = $fn;

            if $safe == Report::Unsafe {
                break;
            }
        )+
    };
}

const INPUT: &str = include_str!("./data.txt");
const _INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
const ACCEPTABLE_DIFFERENCE: std::ops::RangeInclusive<u32> = 1..=3;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Report {
    Safe,
    Unsafe,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        matches!(self, Self::Safe)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    pub fn from_int(lhs: u32, rhs: u32) -> Self {
        if lhs < rhs {
            Direction::Increasing
        } else {
            Direction::Decreasing
        }
    }
}

pub fn part_one() -> u32 {
    let reports = parse(INPUT);

    reports
        .into_iter()
        .filter(|report| {
            let mut previous = None;
            let mut direction = None;
            let mut safe = Report::Safe;

            for level in report.iter() {
                let Some(last) = previous else {
                    previous = Some(level);
                    continue;
                };

                safe_or_break!(
                    safe,
                    verify_difference(*last, *level),
                    verify_or_set_direction(*last, *level, &mut direction)
                );

                previous = Some(level);
            }

            match safe {
                Report::Safe => true,
                Report::Unsafe => false,
            }
        })
        .count()
        .try_into()
        .unwrap()
}

// This works, but could definitely do with some optimization.
pub fn part_two() -> u32 {
    fn evaluate_direction(report: &[u32]) -> Option<Direction> {
        // Could probably find offending indicies using `.find()`
        if report.is_sorted() {
            return Some(Direction::Increasing);
        }
        if report.iter().rev().is_sorted() {
            return Some(Direction::Decreasing);
        }
        None
    }

    fn evaluate_stepping(report: &[u32]) -> Report {
        let mut prev = None;
        for value in report.iter() {
            let Some(last) = prev else {
                prev = Some(value);
                continue;
            };

            if !ACCEPTABLE_DIFFERENCE.contains(&last.abs_diff(*value)) {
                return Report::Unsafe;
            }

            prev = Some(value);
        }

        Report::Safe
    }

    fn evaluate(report: &[u32]) -> Report {
        if evaluate_direction(report).is_some() && evaluate_stepping(report).is_safe() {
            Report::Safe
        } else {
            Report::Unsafe
        }
    }

    fn delete_until_safe(report: &[u32]) -> Report {
        for i in 0..report.len() {
            let mut report = report.to_owned();
            report.remove(i);

            if evaluate(report.as_slice()).is_safe() {
                return Report::Safe;
            }
        }

        Report::Unsafe
    }

    let reports = parse(INPUT);

    reports
        .into_iter()
        .filter(|report| {
            let mut safe = evaluate(report);
            if !safe.is_safe() {
                safe = delete_until_safe(report);
            }
            safe.is_safe()
        })
        .count()
        .try_into()
        .unwrap()
}

/// Adjacent levels must differ by at least one and at most three.
fn verify_difference(previous_level: u32, level: u32) -> Report {
    let difference = previous_level.max(level) - previous_level.min(level);

    if !ACCEPTABLE_DIFFERENCE.contains(&difference) {
        Report::Unsafe
    } else {
        Report::Safe
    }
}

/// A level must always increase or always decrease.
fn verify_or_set_direction(
    previous_level: u32,
    level: u32,
    expected: &mut Option<Direction>,
) -> Report {
    let new_direction = Direction::from_int(previous_level, level);

    match expected {
        // Set the starting direction.
        None => {
            *expected = Some(new_direction);
        }
        // A level must always increase or always decrease.
        Some(d) => {
            if *d != new_direction {
                return Report::Unsafe;
            }
        }
    }

    Report::Safe
}

/// Convert `/(\d+ )+ \d+/` (numbers separated by spaces) to a two-dimensional vector.
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|str| str.parse::<u32>().expect("numbers separated by spaces"))
                .collect()
        })
        .collect()
}

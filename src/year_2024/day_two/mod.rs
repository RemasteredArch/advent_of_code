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
const ACCEPTANCE_DIFFERENCE: std::ops::RangeInclusive<u32> = 1..=3;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Report {
    Safe,
    Unsafe,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TolerantReport {
    Safe,
    SingleUnsafe,
    DoubleUnsafe,
}

impl TolerantReport {
    pub fn advance(&mut self) {
        *self = match self {
            Self::Safe => Self::SingleUnsafe,
            _ => Self::DoubleUnsafe,
        };
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

// This does not currently work for the full data set.
pub fn part_two() -> u32 {
    fn evaluate(report: &Vec<u32>) -> (TolerantReport, Option<usize>) {
        let mut previous = None;
        let mut direction = None;
        let mut safe = TolerantReport::Safe;
        let mut first_unsafe_index = None;

        for (index, level) in report.iter().enumerate() {
            let Some(last) = previous else {
                previous = Some(level);
                continue;
            };

            if verify_difference(*last, *level) == Report::Unsafe
                || verify_or_set_direction(*last, *level, &mut direction) == Report::Unsafe
            {
                print!("   ╭─ {report:?} at {level}\n   ╰─ {safe:?} -> ");
                safe.advance();
                println!("{safe:?}\n");
                match safe {
                    TolerantReport::SingleUnsafe => first_unsafe_index = Some(index),
                    TolerantReport::DoubleUnsafe => break,
                    _ => (),
                }
            }

            previous = Some(level);
        }

        println!("╭─ {report:?}\n╰─ {safe:?}\n");

        (safe, first_unsafe_index)
    }

    let reports = parse(INPUT);

    reports
        .into_iter()
        .filter(|report| {
            let (safe, unsafe_index) = evaluate(report);
            match safe {
                TolerantReport::Safe => true,
                TolerantReport::SingleUnsafe => {
                    println!("# Retrying {report:?} ({safe:?})\n");
                    let mut report = report.clone();
                    report.remove(unsafe_index.unwrap());
                    let result = evaluate(&report).0;
                    println!("# Retried {report:?} ({result:?})\n");
                    result == TolerantReport::Safe
                }
                TolerantReport::DoubleUnsafe => false,
            }
        })
        .count()
        .try_into()
        .unwrap()
}

/// Adjacent levels must differ by at least one and at most three.
fn verify_difference(previous_level: u32, level: u32) -> Report {
    let difference = previous_level.max(level) - previous_level.min(level);

    if !ACCEPTANCE_DIFFERENCE.contains(&difference) {
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

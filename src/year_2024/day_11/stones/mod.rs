use std::{fmt::Display, sync::Mutex};

use crate::Integer;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Stones {
    stones: Vec<Stone>,
}

impl Stones {
    pub fn parse(input: &str) -> Option<Self> {
        let mut stones = vec![];

        for str in input.split(' ') {
            stones.push(Stone::new(str.parse::<Integer>().ok()?));
        }

        Some(Self { stones })
    }

    pub fn blink_n(&mut self, blinks: usize) {
        { 0..blinks }.for_each(|_| self.blink());
    }

    pub fn blink(&mut self) {
        struct StonesIter {
            stones: Mutex<Vec<Stone>>,
            iter_index: usize,
        }

        impl StonesIter {
            pub fn next(&self) -> Option<(usize, Stone)> {
                let stones = &mut self.stones.lock().ok()?;

                stones
                    .get(self.iter_index)
                    .copied()
                    .map(|stone| (self.iter_index, stone))
            }

            #[must_use]
            pub fn set(&self, index: usize, value: Stone) -> Option<()> {
                let stones = &mut self.stones.lock().ok()?;

                *stones.get_mut(index)? = value;

                Some(())
            }

            pub fn insert(&self, index: usize, value: Stone) {
                let stones = &mut self.stones.lock().unwrap();

                stones.insert(index, value);
            }
        }

        let iter = StonesIter {
            stones: Mutex::new(self.stones),
            iter_index: 0,
        };

        while let Some((index, stone)) = iter.next() {
            let (left, right) = stone.blink();

            iter.set(index, left);

            if let Some(stone) = right {
                iter.insert(index + 1, stone)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.stones.len()
    }
}

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.stones
                .iter()
                .map(Stone::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Stone {
    number: Integer,
}

impl Stone {
    pub const fn new(number: Integer) -> Self {
        Self { number }
    }

    pub const fn number(self) -> Integer {
        self.number
    }

    pub fn blink(self) -> (Self, Option<Self>) {
        if self.number == 0 {
            return (Self::new(1), None);
        }

        let as_str = self.to_string();
        let len = as_str.len();

        if len % 2 == 0 {
            let (left, right) = as_str.split_at(len / 2);

            let parse = |str: &str| {
                Self::new(
                    str.parse::<Integer>()
                        .expect("strings produced by `number` should be numbers"),
                )
            };

            return (parse(left), Some(parse(right)));
        }

        (Self::new(self.number * 2024), None)
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

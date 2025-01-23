#[cfg(test)]
mod test;

use std::{fmt::Display, sync::Mutex};

use crate::Integer;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Stones {
    stones: Vec<Stone>,
}

impl Stones {
    pub fn parse(input: &str) -> Option<Self> {
        let mut stones = vec![];

        for str in input.split(' ').map(str::trim_ascii_end) {
            stones.push(Stone::new(str.parse::<Integer>().ok()?));
        }

        Some(Self { stones })
    }

    pub fn blink_n(&mut self, blinks: usize) {
        { 0..blinks }.for_each(|_| *self = self.blink());
    }

    pub fn blink(&self) -> Self {
        struct StonesIter {
            stones: Mutex<Vec<Stone>>,
            iter_index: Mutex<usize>,
        }

        impl StonesIter {
            pub const fn new(stones: Vec<Stone>) -> Self {
                Self {
                    stones: Mutex::new(stones),
                    iter_index: Mutex::new(0),
                }
            }

            pub fn next(&self) -> Option<(usize, Stone)> {
                let stones = self.stones.lock().ok()?;

                let index = self.advance_index();

                stones.get(index).copied().map(|stone| (index, stone))
            }

            fn advance_index(&self) -> usize {
                let mut index_lock = self.iter_index.lock().unwrap();

                let index = *index_lock;
                *index_lock += 1;
                drop(index_lock);

                index
            }

            fn index(&self) -> usize {
                *self.iter_index.lock().unwrap()
            }

            #[must_use]
            pub fn set(&self, index: usize, value: Stone) -> Option<()> {
                *self.stones.lock().ok()?.get_mut(index)? = value;

                Some(())
            }

            pub fn insert(&self, index: usize, value: Stone) {
                self.stones.lock().unwrap().insert(index, value);

                if index <= self.index() {
                    self.advance_index();
                }
            }

            pub fn into_inner(self) -> Stones {
                Stones {
                    stones: self.stones.into_inner().unwrap(),
                }
            }
        }

        let iter = StonesIter::new(self.stones.clone());

        while let Some((index, stone)) = iter.next() {
            let (left, right) = stone.blink();

            iter.set(index, left)
                .expect("`enumerate` should provide valid indices");

            if let Some(stone) = right {
                iter.insert(index + 1, stone);
            }
        }

        iter.into_inner()
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

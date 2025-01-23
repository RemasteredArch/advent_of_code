#[cfg(test)]
mod test;

use std::{cell::RefCell, fmt::Display, sync::Mutex, time::Instant};

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

    pub fn blink_n(mut self, blinks: usize) -> Self {
        let now = Instant::now();

        for i in 0..blinks {
            println!("{i} ({:#?})", now.elapsed());
            self = Self::blink(RefCell::new(self));
        }

        self
    }

    pub fn blink(self_cell: RefCell<Self>) -> Self {
        struct StonesIter {
            stones: RefCell<Vec<Stone>>,
            iter_index: RefCell<usize>,
        }

        impl Iterator for StonesIter {
            type Item = (usize, Stone);

            fn next(&mut self) -> Option<Self::Item> {
                let index = self.advance_index();

                self.stones
                    .get_mut()
                    .get(index)
                    .map(|&stone| (index, stone))
            }
        }

        impl StonesIter {
            pub const fn new(stones: Vec<Stone>) -> Self {
                Self {
                    stones: RefCell::new(stones),
                    iter_index: RefCell::new(0),
                }
            }

            fn advance_index(&self) -> usize {
                let mut index_lock = self.iter_index.borrow_mut();

                let index = *index_lock;
                *index_lock += 1;
                drop(index_lock);

                index
            }

            fn index(&self) -> usize {
                *self.iter_index.borrow_mut()
            }

            #[must_use]
            pub fn set(&self, index: usize, value: Stone) -> Option<()> {
                *self.stones.borrow_mut().get_mut(index)? = value;

                Some(())
            }

            pub fn insert(&self, index: usize, value: Stone) {
                self.stones.borrow_mut().insert(index, value);

                if index <= self.index() {
                    self.advance_index();
                }
            }

            pub fn into_inner(self) -> Stones {
                Stones {
                    stones: self.stones.into_inner(),
                }
            }
        }

        let mut iter = StonesIter::new(self_cell.into_inner().stones);

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

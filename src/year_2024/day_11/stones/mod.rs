#[cfg(test)]
mod test;

use std::{cell::RefCell, fmt::Display, time::Instant};

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

    pub fn blink_n(self, blinks: usize) -> Self {
        let now = Instant::now();

        let mut iter = StonesIter::new(self.stones);

        for i in 0..blinks {
            println!("{i} ({:#?}) ({})", now.elapsed(), iter.dbg_display());
            iter.blink();
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

struct StonesIter {
    stones: RefCell<StoneBufs>,
}

impl Iterator for StonesIter {
    type Item = Stone;

    fn next(&mut self) -> Option<Self::Item> {
        self.stones.borrow_mut().pop()
    }
}

impl StonesIter {
    pub fn new(stones: Vec<Stone>) -> Self {
        Self {
            stones: RefCell::new(StoneBufs::new(stones)),
        }
    }

    pub fn blink(&mut self) {
        while let Some(stone) = self.next() {
            let (left, right) = stone.blink();

            let stones = self.stones.get_mut();

            stones.push(left);

            if let Some(stone) = right {
                stones.push(stone);
            }
        }

        // Should this be here?
        self.stones.get_mut().swap();
    }

    pub fn into_inner(self) -> Stones {
        Stones {
            // Is `into_drain` correct? Why not `into_current`?
            stones: self.stones.into_inner().into_drain(),
        }
    }

    pub fn dbg_display(&self) -> String {
        self.stones.borrow().dbg_display_drain()
    }
}

struct StoneBufs {
    buf_a: Vec<Stone>,
    buf_b: Vec<Stone>,
    current: Buf,
    drain_index: usize,
}

impl StoneBufs {
    pub fn new(stones: Vec<Stone>) -> Self {
        Self {
            buf_a: Vec::with_capacity(stones.len()),
            buf_b: stones,
            current: Buf::A,
            drain_index: 0,
        }
    }

    pub fn push(&mut self, value: Stone) {
        self.current_mut().push(value);
    }

    pub fn pop(&mut self) -> Option<Stone> {
        // ```text
        // 0123456
        //   ^ drain_index: 4
        //     actual_index: 7 - (4 + 1) = 2
        // ```
        let actual_index = self.drain().len().checked_sub(self.drain_index + 1)?;

        self.drain_index += 1;

        Some(
            *self
                .drain()
                .get(actual_index)
                .expect("`checked_sub` will overflow before `get`"),
        )
    }

    pub fn swap(&mut self) {
        assert!(self.drain_index == self.drain().len() - 1);
        self.drain_index = 0;

        self.current_mut().truncate(0);

        self.current = match self.current {
            Buf::A => Buf::B,
            Buf::B => Buf::A,
        }
    }

    fn current_mut(&mut self) -> &mut Vec<Stone> {
        match self.current {
            Buf::A => &mut self.buf_a,
            Buf::B => &mut self.buf_b,
        }
    }

    const fn drain(&self) -> &Vec<Stone> {
        match self.current {
            Buf::A => &self.buf_b,
            Buf::B => &self.buf_a,
        }
    }

    pub fn into_current(self) -> Vec<Stone> {
        match self.current {
            Buf::A => self.buf_a,
            Buf::B => self.buf_b,
        }
    }

    pub fn into_drain(self) -> Vec<Stone> {
        match self.current {
            Buf::A => self.buf_b,
            Buf::B => self.buf_a,
        }
    }

    fn dbg_display_drain(&self) -> String {
        // What should I consider the "true" state of the [`StonesBuf`]? What does it mean to
        // be `n` elements long when you're actually two [`Vec`]s in a trench coat?
        format!(
            "drain: {} stones, total {}",
            self.drain().len(),
            self.drain()
                .iter()
                .map(|stone| stone.number())
                .sum::<Integer>()
        )
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum Buf {
    A,
    B,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, Default)]
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

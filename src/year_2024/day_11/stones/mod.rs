#[cfg(test)]
mod test;

use std::{cell::RefCell, collections::HashMap, fmt::Display, time::Instant};

use crate::Integer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stones {
    stones: StoneMaps,
}

impl Stones {
    pub fn parse(input: &str) -> Option<Self> {
        let mut stones = HashMap::new();

        for str in input.split(' ').map(str::trim_ascii_end) {
            increment_or_insert_n(&mut stones, Stone::new(str.parse::<Integer>().ok()?), 1);
        }

        Some(Self {
            stones: StoneMaps::new(stones),
        })
    }

    pub fn blink_n(&mut self, blinks: usize) {
        print!("Starting as: {self} ({})", self.stones);

        let now = Instant::now();
        for i in 0..blinks {
            self.stones.blink();
            println!(
                // Lengths are chosen based on the longest observed outputs. This is only debug
                // logging, I'm okay with some magic numbers and nasty code.
                "Finished iteration  {:>2}  in  {:>10}  (now {:>15} stones, {:>4} unique){}",
                i + 1,
                format!("{:#?}", now.elapsed()),
                self.stones.len(),
                self.stones.unique_len(),
                if i < blinks - 1 {
                    format!(", starting iteration {:>2}", i + 2)
                } else {
                    String::new()
                }
            );
        }
    }

    pub fn len(&self) -> usize {
        self.stones.len()
    }

    /// For large maps, this can be obscene amounts of memory! For the example input, this is 476
    /// TiB of memory!
    #[cfg_attr(not(test), expect(dead_code, reason = "used in tests"))]
    pub fn as_slice(&self) -> Box<[Stone]> {
        self.stones.as_slice()
    }
}

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stones)
    }
}

fn increment_or_insert_n<K: Eq + std::hash::Hash>(
    map: &mut HashMap<K, usize>,
    key: K,
    count: usize,
) {
    match map.get_mut(&key) {
        Some(current_count) => *current_count += count,
        None => {
            map.insert(key, count);
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StoneMaps {
    map_a: RefCell<HashMap<Stone, usize>>,
    map_b: RefCell<HashMap<Stone, usize>>,
    current: Buf,
    cached_blinks: RefCell<HashMap<Stone, (Stone, Option<Stone>)>>,
}

impl StoneMaps {
    pub fn new(stones: HashMap<Stone, usize>) -> Self {
        Self {
            map_a: RefCell::new(HashMap::with_capacity(stones.len())),
            map_b: RefCell::new(stones),
            current: Buf::A,
            cached_blinks: RefCell::new(HashMap::new()),
        }
    }

    pub fn blink(&mut self) {
        for (stone, count) in self.drain_mut().drain() {
            let (stone, maybe_stone) = self.blink_stone(stone);

            increment_or_insert_n(&mut self.current_mut(), stone, count);

            if let Some(stone) = maybe_stone {
                increment_or_insert_n(&mut self.current_mut(), stone, count);
            }
        }

        self.swap();
    }

    fn swap(&mut self) {
        assert!(self.drain().is_empty());

        self.current = match self.current {
            Buf::A => Buf::B,
            Buf::B => Buf::A,
        };
    }

    fn current_mut(&self) -> std::cell::RefMut<'_, HashMap<Stone, usize>> {
        match self.current {
            Buf::A => self.map_a.borrow_mut(),
            Buf::B => self.map_b.borrow_mut(),
        }
    }

    fn drain(&self) -> std::cell::Ref<'_, HashMap<Stone, usize>> {
        match self.current {
            Buf::A => self.map_b.borrow(),
            Buf::B => self.map_a.borrow(),
        }
    }

    fn drain_mut(&self) -> std::cell::RefMut<'_, HashMap<Stone, usize>> {
        match self.current {
            Buf::A => self.map_b.borrow_mut(),
            Buf::B => self.map_a.borrow_mut(),
        }
    }

    fn blink_stone(&self, stone: Stone) -> (Stone, Option<Stone>) {
        let maybe_stones = self.cached_blinks.borrow().get(&stone).copied();

        maybe_stones.unwrap_or_else(|| {
            let result = stone.blink();
            self.cached_blinks.borrow_mut().insert(stone, result);
            result
        })
    }

    pub fn len(&self) -> usize {
        self.drain().values().sum()
    }

    pub fn unique_len(&self) -> usize {
        self.drain().len()
    }

    /// For large maps, this can be obscene amounts of memory! For the example input, this is 476
    /// TiB of memory!
    pub fn as_slice(&self) -> Box<[Stone]> {
        let mut vec: Vec<Stone> = vec![];

        for (&stone, &count) in self.drain().iter() {
            vec.append(&mut [stone].repeat(count));
        }

        vec.into_boxed_slice()
    }
}

impl Display for StoneMaps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stones = self
            .drain()
            .iter()
            .map(|(stone, count)| {
                // E.g., `"25, 25, 25, "`.
                (stone.to_string() + " ").repeat(*count)
            })
            .collect::<String>();

        assert_eq!(stones.pop(), Some(' '));

        // E.g., `"25, 25, 25, 43, 43"`.
        write!(f, "{stones}")
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum Buf {
    A,
    B,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
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

    pub const fn blink(self) -> (Self, Option<Self>) {
        if self.number == 0 {
            return (Self::new(1), None);
        }

        let len = self.number().ilog10() + 1;

        if len % 2 == 0 {
            // E.g., `1234` -> `12`.
            let left = self.number() / (10 as Integer).pow(len / 2);
            // E.g., `1234` -> `34`.
            let right = self.number() - left * (10 as Integer).pow(len / 2);

            return (Self::new(left), Some(Self::new(right)));
        }

        (Self::new(self.number * 2024), None)
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

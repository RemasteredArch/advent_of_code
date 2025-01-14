use std::fmt::Display;

use crate::Integer;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Filesystem {
    blocks: Vec<Block>,
}

impl Filesystem {
    pub fn parse(input: &str) -> Self {
        enum Next {
            File { id: u8 },
            Empty { id: u8 },
        }

        let mut blocks = vec![];

        let mut next = Next::File { id: 0 };
        for len in input
            .chars()
            .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
        {
            blocks.push(match next {
                Next::File { id } => Block::File { id, len },
                Next::Empty { id: _ } => Block::Empty { len },
            });

            next = match next {
                Next::File { id } => Next::Empty { id: id + 1 },
                Next::Empty { id } => Next::File { id },
            }
        }

        Self { blocks }
    }

    fn last_file(&self) -> Option<File> {
        self.blocks.iter().rev().find_map(|b| b.try_to_file())
    }

    fn push(&mut self, value: Block) {
        let Some(last) = self.blocks.last_mut() else {
            self.blocks.push(value);
            return;
        };

        let len = last.len() + value.len();

        match (*last, value) {
            (
                Block::File {
                    id: last_id,
                    len: _,
                },
                Block::File {
                    id: value_id,
                    len: _,
                },
            ) if last_id == value_id => *last.len_mut() = len,
            (Block::Empty { len: _ }, Block::Empty { len: _ }) => *last.len_mut() = len,
            _ => self.blocks.push(value),
        };
    }

    /// Remove the last file from the filesystem, filling its place with [`Block::Empty`]. Will
    /// join with adjacent [`Block::Empty`].
    fn pop(&mut self) -> Option<File> {
        todo!("implement fs popping")
    }

    pub fn to_compact(&self) -> Self {
        let mut blocks = Self::default();

        for block in self.blocks.iter() {
            match block {
                Block::File { id: _, len: _ } => blocks.push(*block),
                Block::Empty { len } => todo!(),
            }
        }

        todo!("finish compaction")
    }

    pub fn checksum(&self) -> Integer {
        todo!("implement checksumming")
    }
}

impl Display for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.blocks
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct File {
    id: u8,
    len: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Empty {
    len: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Block {
    File { id: u8, len: u8 },
    Empty { len: u8 },
}

impl Block {
    pub const fn len(&self) -> u8 {
        match self {
            Block::File { id: _, len } => *len,
            Block::Empty { len } => *len,
        }
    }

    pub const fn len_mut(&mut self) -> &mut u8 {
        match self {
            Block::File { id: _, len } => len,
            Block::Empty { len } => len,
        }
    }

    pub fn try_to_file(&self) -> Option<File> {
        match *self {
            Block::File { id, len } => Some(File { id, len }),
            Block::Empty { len: _ } => None,
        }
    }

    pub fn try_to_empty(&self) -> Option<Empty> {
        match *self {
            Block::File { id: _, len: _ } => None,
            Block::Empty { len } => Some(Empty { len }),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Block::File { id, len } => id.to_string().repeat(*len as usize),
                Block::Empty { len } => ".".repeat(*len as usize),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::{super::EXAMPLE_INPUT, Filesystem};

    #[test]
    fn parse_and_display() {
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            Filesystem::parse(EXAMPLE_INPUT).to_string()
        );
    }
}

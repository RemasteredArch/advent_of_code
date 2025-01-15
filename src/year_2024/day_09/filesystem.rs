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
                Next::File { id } => Block::File(File { id, len }),
                Next::Empty { id: _ } => Block::Empty(Empty { len }),
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

    fn last_file_block_mut(&mut self) -> Option<(&mut Block, File)> {
        self.blocks
            .iter_mut()
            .rev()
            .find_map(|b| b.try_as_file_block_mut())
    }

    fn push(&mut self, value: Block) {
        let Some(last) = self.blocks.last_mut() else {
            self.blocks.push(value);
            return;
        };

        let len = last.len() + value.len();

        match (*last, value) {
            (Block::File(File { id: last_id, .. }), Block::File(File { id: value_id, .. }))
                if last_id == value_id =>
            {
                *last.len_mut() = len
            }
            (Block::Empty(_), Block::Empty(_)) => *last.len_mut() = len,
            _ => self.blocks.push(value),
        };
    }

    fn fuse_empty(&mut self) {
        todo!();
    }

    /// Remove the last `blocks` of [`File`]s, filling their places with [`Block::Empty`]. Does not
    /// join with adjacent [`Block::Empty`].
    fn pop(&mut self, blocks: usize) -> Vec<File> {
        let mut files = vec![];
        let mut accumulated_blocks = 0;

        while accumulated_blocks < blocks {
            let Some((last, mut as_file)) = self.last_file_block_mut() else {
                return files;
            };

            let remaining = blocks - accumulated_blocks;
            if remaining < last.len() {
                *last.len_mut() -= remaining;
                *as_file.len_mut() = remaining;
            } else {
                *last = Block::Empty(Empty { len: last.len() });
            }

            accumulated_blocks += as_file.len();
            files.push(as_file);
        }

        files
    }

    pub fn to_compact(&self) -> Self {
        let mut blocks = Self::default();

        for block in self.blocks.iter() {
            match block {
                Block::File(_) => blocks.push(*block),
                Block::Empty(_) => todo!(),
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
    len: usize,
}

impl File {
    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn len_mut(&mut self) -> &mut usize {
        &mut self.len
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Empty {
    len: usize,
}

impl Empty {
    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn len_mut(&mut self) -> &mut usize {
        &mut self.len
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Block {
    File(File),
    Empty(Empty),
}

impl Block {
    pub const fn len(&self) -> usize {
        match *self {
            Block::File(File { len, .. }) => len,
            Block::Empty(Empty { len }) => len,
        }
    }

    pub const fn len_mut(&mut self) -> &mut usize {
        match self {
            Block::File(File { len, .. }) => len,
            Block::Empty(Empty { len }) => len,
        }
    }

    pub fn try_to_file(&self) -> Option<File> {
        match *self {
            Block::File(file) => Some(file),
            _ => None,
        }
    }

    pub fn try_as_file_mut(&mut self) -> Option<&mut File> {
        match self {
            Block::File(file) => Some(file),
            _ => None,
        }
    }

    /// Returns a [`Self`] that is guaranteed to be an instance of [`File`], exactly as copied in
    /// the tuple.
    pub fn try_as_file_block_mut(&mut self) -> Option<(&mut Self, File)> {
        match *self {
            Block::File(file) => Some((self, file)),
            _ => None,
        }
    }

    pub fn try_to_empty(&self) -> Option<Empty> {
        match *self {
            Block::Empty(empty) => Some(empty),
            _ => None,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Block::File(File { id, len }) => id.to_string().repeat(*len),
                Block::Empty(Empty { len }) => ".".repeat(*len),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::{super::EXAMPLE_INPUT, File, Filesystem};

    #[test]
    fn parse_and_display() {
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            Filesystem::parse(EXAMPLE_INPUT).to_string()
        );
    }

    #[test]
    fn parse_and_pop() {
        assert_eq!(
            vec![File { id: 9, len: 2 }, File { id: 8, len: 3 }],
            Filesystem::parse(EXAMPLE_INPUT).pop(5),
        );
    }
}

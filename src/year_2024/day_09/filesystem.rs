use std::{fmt::Display, sync::Mutex};

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

    fn last_file_mut(&mut self) -> Option<(&mut Block, File)> {
        self.blocks.iter_mut().rev().find_map(|block| match *block {
            Block::File(file) => Some((block, file)),
            _ => None,
        })
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

    fn append(&mut self, values: Vec<Block>) {
        for block in values {
            self.push(block);
        }
    }

    // TODO: Reimplement to fuse contiguous files.
    fn fuse_empty(&mut self) {
        let mut i = 0;

        while let Some(block) = self.blocks.get(i) {
            if !matches!(block, Block::Empty(_)) {
                i += 1;
                continue;
            }

            while let Some(next) = self.blocks.get(i + 1) {
                if !matches!(next, Block::Empty(_)) {
                    break;
                }

                *self
                    .blocks
                    .get_mut(i)
                    .expect("loop condition proves existence")
                    .len_mut() += next.len();

                self.blocks.remove(i + 1);
            }

            i += 1;
        }
    }

    /// Remove the last `blocks` of [`File`]s, filling their places with [`Block::Empty`]. Does not
    /// join with adjacent [`Block::Empty`].
    fn pop(&mut self, blocks: usize) -> Vec<File> {
        let mut files = vec![];
        let mut accumulated_blocks = 0;

        while accumulated_blocks < blocks {
            let Some((last, mut as_file)) = self.last_file_mut() else {
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
        struct FsIter {
            fs: Mutex<Filesystem>,
            iter_index: usize,
        }

        impl FsIter {
            pub fn next(&mut self) -> Option<Block> {
                let result = self.fs.lock().ok()?.blocks.get(self.iter_index).copied();
                self.iter_index += 1;
                result
            }

            pub fn fs_mut(&self) -> std::sync::MutexGuard<Filesystem> {
                self.fs.lock().unwrap()
            }
        }

        let mut blocks = Self::default();

        let fs = self.clone();
        let mut fs_iter = FsIter {
            fs: Mutex::new(fs),
            iter_index: 0,
        };

        while let Some(block) = fs_iter.next() {
            match block {
                Block::File(_) => blocks.push(block),
                Block::Empty(mut empty) => {
                    let files: Vec<Block> = fs_iter
                        .fs_mut()
                        .pop(empty.len())
                        .iter()
                        .map(|&f| Block::File(f))
                        .collect();

                    let len: usize = files.iter().map(|f| f.len()).sum();

                    blocks.append(files);

                    if len < empty.len() {
                        *empty.len_mut() -= len;
                        blocks.push(Block::Empty(empty));
                    }
                }
            }
        }

        blocks
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
    use super::{super::EXAMPLE_INPUT, Block, Empty, File, Filesystem};

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

    #[test]
    fn fuse_empty() {
        let mut fs = Filesystem {
            blocks: vec![
                Block::File(File { id: 0, len: 2 }),
                Block::Empty(Empty { len: 2 }),
                Block::Empty(Empty { len: 1 }),
                Block::File(File { id: 1, len: 3 }),
                Block::Empty(Empty { len: 2 }),
            ],
        };
        fs.fuse_empty();

        assert_eq!(
            Filesystem {
                blocks: vec![
                    Block::File(File { id: 0, len: 2 }),
                    Block::Empty(Empty { len: 3 }),
                    Block::File(File { id: 1, len: 3 }),
                    Block::Empty(Empty { len: 2 }),
                ]
            },
            fs
        );
    }

    #[test]
    fn to_compact() {
        let fs = Filesystem {
            blocks: vec![
                Block::File(File { id: 0, len: 2 }),
                Block::Empty(Empty { len: 4 }),
                Block::File(File { id: 0, len: 3 }),
                Block::Empty(Empty { len: 2 }),
            ],
        };

        assert_eq!(
            Filesystem {
                blocks: vec![
                    Block::File(File { id: 0, len: 5 }),
                    Block::Empty(Empty { len: 1 }),
                    Block::Empty(Empty { len: 2 }),
                ]
            },
            fs.to_compact()
        );
    }
}

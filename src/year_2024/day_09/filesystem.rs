use std::{fmt::Display, sync::Mutex};

use crate::Integer;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Filesystem {
    spans: Vec<Span>,
}

#[expect(unused, reason = "in-progress")]
impl Filesystem {
    /// Parse from the official format from Advent of Code.
    pub fn parse(input: &str) -> Self {
        enum Next {
            File { id: u8 },
            Empty { id: u8 },
        }

        let mut spans = vec![];

        let mut next = Next::File { id: 0 };
        for len in input
            .chars()
            .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
        {
            if len != 0 {
                spans.push(match next {
                    Next::File { id } => Span::File(File { id, len }),
                    Next::Empty { id: _ } => Span::Empty(Empty { len }),
                });
            }

            next = match next {
                Next::File { id } => Next::Empty { id: id + 1 },
                Next::Empty { id } => Next::File { id },
            }
        }

        Self { spans }
    }

    /// Parse from the [`Display`] format of [`Self`].
    pub fn deserialize(input: &str) -> Self {
        let mut fs = Self::default();

        for char in input.chars() {
            let span = match char
                .to_digit(10)
                .and_then(|c| TryInto::<u8>::try_into(c).ok())
            {
                Some(id) => Span::File(File { id, len: 1 }),
                None if char == '.' => Span::Empty(Empty { len: 1 }),
                None => continue,
            };

            fs.push(span);
        }

        fs
    }

    fn last_file(&self) -> Option<File> {
        self.spans.iter().rev().find_map(Span::try_to_file)
    }

    fn last_file_mut(&mut self) -> Option<(&mut Span, File)> {
        self.spans.iter_mut().rev().find_map(|span| match *span {
            Span::File(file) => Some((span, file)),
            Span::Empty(_) => None,
        })
    }

    fn push(&mut self, value: Span) {
        let Some(last) = self.spans.last_mut() else {
            self.spans.push(value);
            return;
        };

        match (*last, value) {
            (Span::File(last_file), Span::File(value_file)) if last_file.id == value_file.id => {
                *last.len_mut() += value.len();
            }
            (Span::Empty(_), Span::Empty(_)) => *last.len_mut() += value.len(),
            _ => self.spans.push(value),
        };
    }

    fn append(&mut self, spans: Vec<Span>) {
        for span in spans {
            self.push(span);
        }
    }

    // TODO: Reimplement to fuse contiguous files.
    fn fuse_empty(&mut self) {
        let mut i = 0;

        while let Some(span) = self.spans.get(i) {
            if !matches!(span, Span::Empty(_)) {
                i += 1;
                continue;
            }

            while let Some(next) = self.spans.get(i + 1) {
                if !matches!(next, Span::Empty(_)) {
                    break;
                }

                *self
                    .spans
                    .get_mut(i)
                    .expect("loop condition proves existence")
                    .len_mut() += next.len();

                self.spans.remove(i + 1);
            }

            i += 1;
        }
    }

    /// Remove the last `blocks` of [`File`]s, filling their places with [`Span::Empty`]. Does not
    /// join with adjacent [`Span::Empty`].
    ///
    /// Returns the values in reverse order.
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
                *last = Span::Empty(Empty { len: last.len() });
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
            pub fn next(&mut self) -> Option<Span> {
                let result = self.fs.lock().ok()?.spans.get(self.iter_index).copied();
                self.iter_index += 1;
                result
            }

            pub fn fs_mut(&self) -> std::sync::MutexGuard<Filesystem> {
                self.fs.lock().unwrap()
            }
        }

        let mut spans = Self::default();

        let fs = self.clone();
        let mut fs_iter = FsIter {
            fs: Mutex::new(fs),
            iter_index: 0,
        };

        while let Some(span) = fs_iter.next() {
            match span {
                Span::File(_) => spans.push(span),
                Span::Empty(mut empty) => {
                    let files: Vec<Span> = fs_iter
                        .fs_mut()
                        .pop(empty.len())
                        .iter()
                        .map(|&f| Span::File(f))
                        .collect();

                    let len: usize = files.iter().map(Span::len).sum();

                    spans.append(files);

                    if len < empty.len() {
                        *empty.len_mut() -= len;
                        spans.push(Span::Empty(empty));
                    }
                }
            }
        }

        spans
    }

    pub fn checksum(&self) -> Integer {
        // Tracks the actual block-level index in the filesystem.
        let mut block_index = 0;

        self.spans
            .iter()
            // For every file,
            .filter_map(|s| {
                // For every block that file spans,
                s.try_to_file().map(|s| {
                    { 0..s.len() }
                        // Sum the `id` times the block-level index.
                        .map(|_| {
                            let result = block_index * s.id as usize;
                            block_index += 1;
                            result
                        })
                        .sum::<usize>()
                })
            })
            .sum::<usize>()
            .try_into()
            .unwrap()
    }
}

impl Display for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.spans
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct File {
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
pub struct Empty {
    len: usize,
}

impl Empty {
    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn len_mut(&mut self) -> &mut usize {
        &mut self.len
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Span {
    File(File),
    Empty(Empty),
}

impl Span {
    pub const fn len(&self) -> usize {
        match *self {
            Self::Empty(Empty { len }) | Self::File(File { len, .. }) => len,
        }
    }

    pub const fn len_mut(&mut self) -> &mut usize {
        match self {
            Self::Empty(Empty { len }) | Self::File(File { len, .. }) => len,
        }
    }

    pub const fn try_to_file(&self) -> Option<File> {
        match *self {
            Self::File(file) => Some(file),
            Self::Empty(_) => None,
        }
    }

    pub fn try_as_file_mut(&mut self) -> Option<&mut File> {
        match self {
            Self::File(file) => Some(file),
            Self::Empty(_) => None,
        }
    }

    pub const fn try_to_empty(&self) -> Option<Empty> {
        match *self {
            Self::Empty(empty) => Some(empty),
            Self::File(_) => None,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::File(File { id, len }) => id.to_string().repeat(len),
                Self::Empty(Empty { len }) => ".".repeat(len),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::{super::EXAMPLE_INPUT, Empty, File, Filesystem, Span};

    #[test]
    fn parse_display_and_deserialize() {
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        let parsed = Filesystem::parse(EXAMPLE_INPUT);

        assert_eq!(expected, parsed.to_string());
        assert_eq!(parsed, Filesystem::deserialize(expected));
    }

    #[test]
    fn pop() {
        let mut fs = Filesystem::deserialize("00...111...2...333.44.5555.6666.777.888899");

        assert_eq!(
            vec![File { id: 9, len: 2 }, File { id: 8, len: 3 }],
            fs.clone().pop(5),
        );

        assert_eq!(
            vec![
                File { id: 9, len: 2 },
                File { id: 8, len: 4 },
                File { id: 7, len: 3 },
            ],
            fs.pop(9),
        );
    }

    #[test]
    fn fuse_empty() {
        let mut fs = Filesystem {
            spans: vec![
                Span::File(File { id: 0, len: 2 }),
                Span::Empty(Empty { len: 2 }),
                Span::Empty(Empty { len: 1 }),
                Span::File(File { id: 1, len: 3 }),
                Span::Empty(Empty { len: 2 }),
            ],
        };
        fs.fuse_empty();

        assert_eq!(
            Filesystem {
                spans: vec![
                    Span::File(File { id: 0, len: 2 }),
                    Span::Empty(Empty { len: 3 }),
                    Span::File(File { id: 1, len: 3 }),
                    Span::Empty(Empty { len: 2 }),
                ]
            },
            fs
        );
    }

    #[test]
    fn push() {
        let mut fs = Filesystem {
            spans: vec![
                Span::File(File { id: 0, len: 2 }),
                Span::Empty(Empty { len: 4 }),
            ],
        };
        fs.push(Span::Empty(Empty { len: 2 }));

        assert_eq!(
            Filesystem {
                spans: vec![
                    Span::File(File { id: 0, len: 2 }),
                    Span::Empty(Empty { len: 6 }),
                ],
            },
            fs,
        );

        let mut fs = Filesystem {
            spans: vec![
                Span::File(File { id: 0, len: 2 }),
                Span::Empty(Empty { len: 4 }),
            ],
        };
        fs.push(Span::File(File { id: 0, len: 2 }));

        assert_eq!(
            Filesystem {
                spans: vec![
                    Span::File(File { id: 0, len: 2 }),
                    Span::Empty(Empty { len: 4 }),
                    Span::File(File { id: 0, len: 2 }),
                ],
            },
            fs,
        );

        let mut fs = Filesystem {
            spans: vec![
                Span::Empty(Empty { len: 4 }),
                Span::File(File { id: 0, len: 2 }),
            ],
        };
        fs.push(Span::File(File { id: 0, len: 2 }));

        assert_eq!(
            Filesystem {
                spans: vec![
                    Span::Empty(Empty { len: 4 }),
                    Span::File(File { id: 0, len: 4 }),
                ],
            },
            fs,
        );

        let mut fs = Filesystem {
            spans: vec![
                Span::Empty(Empty { len: 4 }),
                Span::File(File { id: 0, len: 2 }),
            ],
        };
        fs.push(Span::File(File { id: 1, len: 2 }));

        assert_eq!(
            Filesystem {
                spans: vec![
                    Span::Empty(Empty { len: 4 }),
                    Span::File(File { id: 0, len: 2 }),
                    Span::File(File { id: 1, len: 2 }),
                ],
            },
            fs,
        );
    }

    #[test]
    fn to_compact() {
        let fs = Filesystem {
            spans: vec![
                Span::File(File { id: 0, len: 2 }),
                Span::Empty(Empty { len: 4 }),
                Span::File(File { id: 0, len: 3 }),
                Span::Empty(Empty { len: 2 }),
            ],
        };
        let compact = fs.to_compact();

        let expected = Filesystem {
            spans: vec![
                Span::File(File { id: 0, len: 5 }),
                Span::Empty(Empty { len: 1 }),
                Span::Empty(Empty { len: 5 }),
            ],
        };

        println!("{fs}\n{compact}\n{expected}");

        assert_eq!(expected, compact);
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Filesystem {
                spans: vec![
                    Span::File(File { id: 0, len: 2 }),
                    Span::File(File { id: 9, len: 2 }),
                    Span::File(File { id: 8, len: 1 }),
                    Span::File(File { id: 1, len: 3 }),
                    Span::File(File { id: 8, len: 3 }),
                    Span::File(File { id: 2, len: 1 }),
                    Span::File(File { id: 7, len: 3 }),
                    Span::File(File { id: 3, len: 3 }),
                    Span::File(File { id: 6, len: 1 }),
                    Span::File(File { id: 4, len: 2 }),
                    Span::File(File { id: 6, len: 1 }),
                    Span::File(File { id: 5, len: 4 }),
                    Span::File(File { id: 6, len: 2 }),
                    Span::Empty(Empty { len: 14 }),
                ],
            },
            Filesystem::deserialize("0099811188827773336446555566.............."),
        );
    }

    #[test]
    fn checksum() {
        let fs = Filesystem {
            // `0099811188827773336446555566..............`
            spans: vec![
                Span::File(File { id: 0, len: 2 }),
                Span::File(File { id: 9, len: 2 }),
                Span::File(File { id: 8, len: 1 }),
                Span::File(File { id: 1, len: 3 }),
                Span::File(File { id: 8, len: 3 }),
                Span::File(File { id: 2, len: 1 }),
                Span::File(File { id: 7, len: 3 }),
                Span::File(File { id: 3, len: 3 }),
                Span::File(File { id: 6, len: 1 }),
                Span::File(File { id: 4, len: 2 }),
                Span::File(File { id: 6, len: 1 }),
                Span::File(File { id: 5, len: 4 }),
                Span::File(File { id: 6, len: 2 }),
                Span::Empty(Empty { len: 14 }),
            ],
        };

        assert_eq!(1928, fs.checksum());
    }
}

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
            Span::Empty(Empty { len: 6 }),
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

const INPUT: &str = include_str!("./data.txt");

// Not finished!
pub fn part_one() -> u32 {
    const NUM_LEN: std::ops::RangeInclusive<usize> = 1..=3;

    #[derive(PartialEq, Debug)]
    enum Pattern {
        Char(char),
        Number,
        NumberOrComma,
        NumberOrCloseParen,
        Comma,
        None,
    }

    impl PartialEq<char> for Pattern {
        fn eq(&self, other: &char) -> bool {
            match self {
                Self::Char(c) => c == other,
                Self::Number => other.is_numeric(),
                Self::NumberOrComma => *other == ',' || other.is_numeric(),
                Self::NumberOrCloseParen => *other == ')' || other.is_numeric(),
                Self::Comma => *other == ',',
                Self::None => false,
            }
        }
    }

    fn current_num_length(stack: &str) -> usize {
        stack.chars().rev().take_while(|c| c.is_numeric()).count()
    }

    fn eval_stack(stack: String) -> u32 {
        dbg!(&stack);
        let [lhs, rhs] = stack
            .split(',')
            .map(|str| {
                str.chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect::<Vec<_>>()[..]
        else {
            panic!("encountered a stack without two numbers separated by a comma")
        };

        lhs * rhs
    }

    let mut stack = String::new();
    let mut total: u32 = 0;

    for char in INPUT.chars() {
        print!("{char}");
        let ideal_next = match stack.chars().last() {
            None => Pattern::Char('m'),
            Some('m') => Pattern::Char('u'),
            Some('u') => Pattern::Char('l'),
            Some('l') => Pattern::Char('('),
            Some('(') => Pattern::Number,
            Some(')') => Pattern::None,
            Some(',') => Pattern::Number,
            Some(c) if c.is_numeric() => {
                let is_lhs = !stack.contains(',');
                eprintln!("{}╰─ {c} is_lhs: {is_lhs}", " ".repeat(19 + " == ".len()));

                if current_num_length(&stack) == *NUM_LEN.end() {
                    if is_lhs {
                        Pattern::Comma
                    } else {
                        Pattern::Char(')')
                    }
                } else if is_lhs {
                    Pattern::NumberOrComma
                } else {
                    Pattern::NumberOrCloseParen
                }
            }
            _ => panic!("bad character in stack"),
        };
        eprintln!(
            "{:>19} == {char} {} ({stack:?})",
            format!("{ideal_next:?}"),
            ideal_next == char
        );

        if ideal_next == Pattern::None {
            total += eval_stack(std::mem::take(&mut stack));
        }

        if ideal_next == char {
            stack.push(char);
        }
    }

    total
}

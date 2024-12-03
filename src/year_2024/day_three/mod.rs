const INPUT: &str = include_str!("./data.txt");
const _INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

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

    impl Pattern {
        pub fn ideal_next(stack: &str) -> Self {
            match stack.chars().last() {
                Option::None => Pattern::Char('m'),
                Some('m') => Pattern::Char('u'),
                Some('u') => Pattern::Char('l'),
                Some('l') => Pattern::Char('('),
                Some('(') => Pattern::Number,
                Some(c) if c.is_numeric() => {
                    let is_lhs = !stack.contains(',');

                    if current_num_length(stack) == *NUM_LEN.end() {
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
                Some(',') => Pattern::Number,
                Some(')') => Pattern::None,
                _ => panic!("bad character in stack"),
            }
        }
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

    // This doesn't need to be heap allocated, it has a fixed maximum length.
    let mut stack = String::with_capacity("mul(123,123)".len());
    let mut total: u32 = 0;

    for char in INPUT.chars() {
        // The expected value of the current character, based on the current top of the stack.
        let mut ideal_next = Pattern::ideal_next(&stack);

        // If no character is expected, that means the stack is finished and can be evaluated.
        if ideal_next == Pattern::None {
            // Drain the stack and evaluate the result.
            total += eval_stack(std::mem::take(&mut stack));

            // Reset the expectation for the newly-emptied stack.
            ideal_next = Pattern::ideal_next(&stack);
        }

        if ideal_next == char {
            // Current stack is still valid, add to it.
            stack.push(char);
        } else {
            // Current stack is invalid, clear it.
            stack.clear();
        }
    }

    total
}

pub trait Stack<P: PartialEq<char>>
where
    Self: std::fmt::Debug,
{
    /// Create a new instance of [`Self`].
    fn new() -> Self;

    /// The expected value of the current character, based on the current top of the stack.
    fn ideal_next(&self) -> P;

    /// Returns true if the stack is a fully filled in, valid form of itself ready to be evaluated.
    fn is_valid(&self) -> bool;

    /// Get [`Self`] as a string slice.
    fn as_str(&self) -> &str;

    /// Push a character onto the top of the stack.
    fn push(&mut self, char: char);

    /// Push a character onto the step if matches [`Self::ideal_next`], otherwise clear the stack.
    fn push_or_clear(&mut self, char: char) {
        if self.ideal_next() == char {
            // Current stack is still valid, add to it.
            self.push(char);
        } else {
            // Current stack is invalid, clear it.
            self.clear();
        }
    }

    /// Get the character currently on top of the stack.
    fn peek(&self) -> Option<char>;

    /// Empty the stack.
    fn clear(&mut self);

    /// If the stack is valid, clear it and return `true`. Otherwise, return `false`.
    fn clear_if_valid(&mut self) -> bool {
        if self.is_valid() {
            self.clear();
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MulStack {
    stack: String,
}

impl MulStack {
    pub fn current_num_length(&self) -> usize {
        self.as_str()
            .chars()
            .rev()
            .take_while(|c| c.is_numeric())
            .count()
    }

    /// If the stack is valid, drain the stack and evaluate the result. Otherwise, do nothing and
    /// return `0`.
    pub fn eval_if_valid(&mut self) -> u32 {
        if self.is_valid() {
            self.eval()
        } else {
            0
        }
    }

    /// Drain the stack and evaluate the result.
    pub fn eval(&mut self) -> u32 {
        let [lhs, rhs] = std::mem::take(&mut self.stack)
            .split(',')
            .map(|str| {
                str.chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .take(2) // There should only ever be two, but it doesn't hurt.
            .collect::<Vec<_>>()[..]
        else {
            panic!("encountered a stack without two numbers separated by a comma")
        };

        lhs * rhs
    }
}

impl Stack<MulPattern> for MulStack {
    fn new() -> Self {
        Self {
            stack: String::with_capacity("mul(123,123)".len()),
        }
    }

    fn ideal_next(&self) -> MulPattern {
        const NUM_LEN: std::ops::RangeInclusive<usize> = 1..=3;

        match self.peek() {
            Option::None => MulPattern::Char('m'),
            Some('m') => MulPattern::Char('u'),
            Some('u') => MulPattern::Char('l'),
            Some('l') => MulPattern::Char('('),
            Some('(') => MulPattern::Number,
            Some(c) if c.is_numeric() => {
                let is_lhs = !self.as_str().contains(',');

                if self.current_num_length() == *NUM_LEN.end() {
                    if is_lhs {
                        MulPattern::Comma
                    } else {
                        MulPattern::Char(')')
                    }
                } else if is_lhs {
                    MulPattern::NumberOrComma
                } else {
                    MulPattern::NumberOrCloseParen
                }
            }
            Some(',') => MulPattern::Number,
            Some(')') => MulPattern::None,
            _ => panic!("bad character in stack"),
        }
    }

    fn is_valid(&self) -> bool {
        self.ideal_next() == MulPattern::None
    }

    fn as_str(&self) -> &str {
        &self.stack
    }

    fn push(&mut self, char: char) {
        self.stack.push(char);
    }

    fn peek(&self) -> Option<char> {
        self.as_str().chars().last()
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

#[derive(PartialEq, Debug)]
pub enum MulPattern {
    Char(char),
    Number,
    NumberOrComma,
    NumberOrCloseParen,
    Comma,
    None,
}

impl PartialEq<char> for MulPattern {
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

#[derive(Debug)]
pub struct DoStack {
    stack: String,
}

impl Stack<DoPattern> for DoStack {
    fn new() -> Self {
        Self {
            stack: String::with_capacity("do()".len()),
        }
    }

    fn ideal_next(&self) -> DoPattern {
        match self.peek() {
            Option::None => DoPattern::D,
            Some('d') => DoPattern::O,
            Some('o') => DoPattern::OpenParen,
            Some('(') => DoPattern::CloseParen,
            Some(')') => DoPattern::None,
            _ => panic!("bad character in stack"),
        }
    }

    fn is_valid(&self) -> bool {
        self.ideal_next() == DoPattern::None
    }

    fn as_str(&self) -> &str {
        &self.stack
    }

    fn push(&mut self, char: char) {
        self.stack.push(char);
    }

    fn peek(&self) -> Option<char> {
        self.stack.chars().last()
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DoPattern {
    D,
    O,
    OpenParen,
    CloseParen,
    None,
}

impl PartialEq<char> for DoPattern {
    fn eq(&self, other: &char) -> bool {
        Into::<Option<char>>::into(*self).is_some_and(|c| c == *other)
    }
}

impl From<DoPattern> for Option<char> {
    fn from(value: DoPattern) -> Self {
        match value {
            DoPattern::D => Some('d'),
            DoPattern::O => Some('o'),
            DoPattern::OpenParen => Some('('),
            DoPattern::CloseParen => Some(')'),
            DoPattern::None => None,
        }
    }
}

#[derive(Debug)]
pub struct DontStack {
    stack: String,
}

impl Stack<DontPattern> for DontStack {
    fn new() -> Self {
        Self {
            stack: String::with_capacity("don't()".len()),
        }
    }

    fn ideal_next(&self) -> DontPattern {
        match self.peek() {
            Option::None => DontPattern::D,
            Some('d') => DontPattern::O,
            Some('o') => DontPattern::N,
            Some('n') => DontPattern::Apostrophe,
            Some('\'') => DontPattern::T,
            Some('t') => DontPattern::OpenParen,
            Some('(') => DontPattern::CloseParen,
            Some(')') => DontPattern::None,
            _ => panic!("bad character in stack"),
        }
    }

    fn is_valid(&self) -> bool {
        self.ideal_next() == DontPattern::None
    }

    fn as_str(&self) -> &str {
        &self.stack
    }

    fn push(&mut self, char: char) {
        self.stack.push(char);
    }

    fn peek(&self) -> Option<char> {
        self.stack.chars().last()
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DontPattern {
    D,
    O,
    N,
    Apostrophe,
    T,
    OpenParen,
    CloseParen,
    None,
}

impl PartialEq<char> for DontPattern {
    fn eq(&self, other: &char) -> bool {
        Into::<Option<char>>::into(*self).is_some_and(|c| c == *other)
    }
}

impl From<DontPattern> for Option<char> {
    fn from(value: DontPattern) -> Self {
        match value {
            DontPattern::D => Some('d'),
            DontPattern::O => Some('o'),
            DontPattern::N => Some('n'),
            DontPattern::Apostrophe => Some('\''),
            DontPattern::T => Some('t'),
            DontPattern::OpenParen => Some('('),
            DontPattern::CloseParen => Some(')'),
            DontPattern::None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{DontStack, Stack};

    #[test]
    fn test_dont_stack() {
        let mut stack = DontStack::new();

        for c in "don't()".chars() {
            assert!(!stack.is_valid());
            stack.push_or_clear(c);
        }

        assert!(stack.is_valid());
    }
}

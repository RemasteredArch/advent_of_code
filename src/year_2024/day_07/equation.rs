use crate::Integer;
use std::{fmt::Display, num::NonZeroUsize};

#[derive(Debug, PartialEq, Eq)]
pub struct Equation {
    expected_value: Integer,
    inputs: Box<[Integer]>,
}

impl Equation {
    pub fn new(expected_value: Integer, inputs: Box<[Integer]>) -> Self {
        Self {
            expected_value,
            inputs,
        }
    }

    pub fn expected_value(&self) -> Integer {
        self.expected_value
    }

    pub fn inputs(&self) -> &[Integer] {
        &self.inputs
    }

    pub fn is_valid(&self) -> bool {
        match self.inputs.len() {
            0 => return false,
            1 => return self.expected_value == *self.inputs.first().expect("`inputs` is length 1"),
            _ => (),
        }

        // All of these values could fit in like... a [`u16`], so all these casts are safe.
        let operations = NonZeroUsize::new(self.inputs.len() - 1).expect("`inputs` is length >1");

        for i in 0..2_usize.pow(operations.get() as u32) {
            // Standardizes bit order: `11 => 1101 0000 0000 0000 ...`.
            //
            // Makes the assumption that the most significant bit is always first, regardless of
            // byte endianness.
            let standard = i.to_le().reverse_bits();

            // Convert to binary string, then truncate to the relevant length, and convert the
            // binary to [`Operation`]s.
            let mut operations = format!(
                "{:0>width$b}",
                standard,
                width = NonZeroUsize::BITS as usize
            )
            .chars()
            .take(operations.get())
            .filter_map(|c| c.try_into().ok())
            .collect::<Vec<Operation>>();

            let mut iter = self.inputs.iter();
            let mut acculumated = *iter.next().expect("`inputs` is length >1");
            print!("{}: {acculumated}", self.expected_value);
            for value in iter {
                let operator = operations
                    .pop()
                    .expect("`operations` is `inputs.len() - 1` in a loop of `inputs.len() - 1`");

                print!(" {operator} {value}");
                acculumated = operator.apply(acculumated, *value);
            }
            print!(" = {acculumated}");

            if acculumated == self.expected_value {
                println!(" (valid)");
                return true;
            }

            println!();
        }

        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
}

impl Operation {
    pub fn apply(&self, lhs: Integer, rhs: Integer) -> Integer {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
        }
    }
}

impl From<Operation> for char {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Add => '+',
            Operation::Multiply => '*',
        }
    }
}

impl From<Operation> for bool {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Add => false,
            Operation::Multiply => true,
        }
    }
}

impl From<bool> for Operation {
    fn from(value: bool) -> Self {
        match value {
            false => Operation::Add,
            true => Operation::Multiply,
        }
    }
}

impl TryFrom<char> for Operation {
    // I do not feel like making an error type to communicate that a character is not in the list
    // of convertible characters. Treat this like an [`Option`].
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operation::Add),
            '0' => Ok(true.into()),
            '*' => Ok(Operation::Multiply),
            '1' => Ok(false.into()),
            _ => Err(()),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<char>::into(*self))
    }
}

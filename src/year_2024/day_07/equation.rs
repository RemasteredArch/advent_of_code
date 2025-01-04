use crate::Integer;
use std::{fmt::Display, num::NonZeroUsize, str::FromStr};

use super::base;

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

    /// Tests all possible combinations of [`Operation::Add`] and [`Operation::Multiply`] on
    /// [`Self::inputs`] to see if any match [`Self::expected_value`]. If any match, return `true`,
    /// else `false`.
    pub fn is_valid_binary(&self) -> bool {
        match self.inputs.len() {
            0 => return false,
            1 => return self.expected_value == *self.inputs.first().expect("`inputs` is length 1"),
            _ => (),
        }

        // All of these values could fit in like... a [`u16`], so all these casts are safe.
        let operations = NonZeroUsize::new(self.inputs.len() - 1).expect("`inputs` is length >1");

        // Represents each operator as bits in a binary word, which means that iterating over the
        // values from zero to the max value of an unsigned integer of length `operations` will hit
        // every possible combination of operators.
        for i in 0..2_usize.pow(operations.get() as u32) {
            let mut operations = base::to_binary_operations(i, operations.into());

            // Applies the `operations` on `self.inputs`.
            let mut iter = self.inputs.iter();
            let mut acculumated = *iter.next().expect("`inputs` is length >1");
            for value in iter {
                acculumated = operations
                    .pop()
                    .expect("`operations` is `inputs.len() - 1` in a loop of `inputs.len() - 1`")
                    .apply(acculumated, *value);
            }

            if acculumated == self.expected_value {
                return true;
            }
        }

        false
    }

    /// Tests all possible combinations of [`Operation`]s on [`Self::inputs`] to see if any match
    /// [`Self::expected_value`]. If any match, return `true`, else `false`.
    pub fn is_valid_ternary(&self) -> bool {
        match self.inputs.len() {
            0 => return false,
            1 => return self.expected_value == *self.inputs.first().expect("`inputs` is length 1"),
            _ => (),
        }

        // All of these values could fit in like... a [`u16`], so all these casts are safe.
        let operations = NonZeroUsize::new(self.inputs.len() - 1).expect("`inputs` is length >1");

        for i in 1..3_usize.pow(operations.get() as u32) {
            let mut operations = base::to_ternary_operations(i, operations.into());

            // Applies the `operations` on `self.inputs`.
            let mut iter = self.inputs.iter();
            let mut acculumated = *iter.next().expect("`inputs` is length >1");
            for value in iter {
                acculumated = operations
                    .pop()
                    .expect("`operations` is `inputs.len() - 1` in a loop of `inputs.len() - 1`")
                    .apply(acculumated, *value);
            }

            if acculumated == self.expected_value {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    pub fn apply(&self, lhs: Integer, rhs: Integer) -> Integer {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
            Self::Concatenate => {
                let mut lhs = lhs.to_string();
                lhs.push_str(&rhs.to_string());
                lhs.parse()
                    .expect("concatenated integers should create an integer")
            }
        }
    }

    pub fn from_ternary(digit: char) -> Option<Self> {
        match digit {
            '0' => Some(Self::Add),
            '1' => Some(Self::Multiply),
            '2' => Some(Self::Concatenate),
            _ => None,
        }
    }
}

impl FromStr for Operation {
    // I do not feel like making an error type to communicate that a character is not in the list
    // of convertible characters. Treat this like an [`Option`].
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" | "0" => Ok(Self::Add),
            "*" | "1" => Ok(Self::Multiply),
            "||" | "2" => Ok(Self::Concatenate),
            _ => Err(()),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Multiply => "*",
                Self::Concatenate => "||",
            }
        )
    }
}

use crate::Integer;
use core::panic;
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
        self.is_valid(2)
    }

    /// Tests all possible combinations of [`Operation`]s on [`Self::inputs`] to see if any match
    /// [`Self::expected_value`]. If any match, return `true`, else `false`.
    pub fn is_valid_ternary(&self) -> bool {
        self.is_valid(3)
    }

    fn is_valid(&self, base: usize) -> bool {
        const MAX_BASE: usize = Operation::base();
        assert!(
            base <= MAX_BASE,
            "invalid base (received {base}, expected <= {MAX_BASE})",
        );

        match self.inputs.len() {
            0 => return false,
            1 => return self.expected_value == *self.inputs.first().expect("`inputs` is length 1"),
            _ => (),
        }

        let operations = self.inputs.len() - 1;

        for i in 0..base.pow(operations as u32) {
            let mut operations = base::to_base_operations(base, i, operations);

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
    /// When representing [`Self`] numerically, what is the base of the counting system? Currently,
    /// there are three members of [`Self`], so it can be represented by a ternary (base-3) value.
    pub const fn base() -> usize {
        3
    }

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

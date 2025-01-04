use crate::Integer;

use super::base::Base;

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

    /// Tests all possible combinations of [`Operation::Add`] and [`Operation::Multiply`] on
    /// [`Self::inputs`] to see if any match [`Self::expected_value`]. If any match, return `true`,
    /// else `false`.
    pub fn is_valid_binary(&self) -> bool {
        self.is_valid(Base::new(2).expect("2 is a valid `Base`"))
    }

    /// Tests all possible combinations of [`Operation`]s on [`Self::inputs`] to see if any match
    /// [`Self::expected_value`]. If any match, return `true`, else `false`.
    pub fn is_valid_ternary(&self) -> bool {
        self.is_valid(Base::new(3).expect("3 is a valid `Base`"))
    }

    fn is_valid(&self, base: Base) -> bool {
        match self.inputs.len() {
            0 => return false,
            1 => return self.expected_value == *self.inputs.first().expect("`inputs` is length 1"),
            _ => (),
        }

        let operations = self.inputs.len() - 1;

        for i in 0..base.get().pow(operations as u32) {
            let mut operations = base.int_to_operations(i, operations);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Operation {
    #[default]
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
            Self::Concatenate => lhs * (10 as Integer).pow(rhs.ilog10() + 1) + rhs,
        }
    }

    pub fn from_digit(digit: usize) -> Option<Self> {
        match digit {
            0 => Some(Self::Add),
            1 => Some(Self::Multiply),
            2 => Some(Self::Concatenate),
            _ => None,
        }
    }
}

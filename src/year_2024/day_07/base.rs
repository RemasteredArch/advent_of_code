use super::equation::Operation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Base {
    base: usize,
}

impl Base {
    pub fn new(base: usize) -> Option<Self> {
        if base > Operation::base() {
            return None;
        }

        Some(Self { base })
    }

    pub fn get(&self) -> usize {
        self.base
    }

    pub fn int_to_operations(&self, mut int: usize, length: usize) -> Vec<Operation> {
        let mut operations = Vec::with_capacity(length);

        while int > 0 {
            let remainder = int % self.base;
            int /= self.base;

            operations.push(
                Operation::from_digit(remainder)
                    .expect("modulo `Base` will always be an accepted value by `Operation`"),
            );
        }

        while operations.len() < length {
            operations.push(Operation::default());
        }

        operations
    }
}

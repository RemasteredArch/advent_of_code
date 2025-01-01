use std::num::NonZeroUsize;

pub struct Equation {
    expected_value: u32,
    inputs: Box<[u32]>,
}

impl Equation {
    pub fn new(expected_value: u32, inputs: Box<[u32]>) -> Self {
        Self {
            expected_value,
            inputs,
        }
    }

    pub fn value(&self) -> u32 {
        self.expected_value
    }

    pub fn numbers(&self) -> &[u32] {
        &self.inputs
    }

    pub fn is_valid(&self) -> bool {
        match self.inputs.len() {
            0 => return false,
            1 => return self.expected_value == *self.inputs.first().expect("`inputs` is length 1"),
            _ => (),
        }

        let operations = NonZeroUsize::new(self.inputs.len() - 1).expect("`inputs` is length >1");

        println!("{operations}");

        for i in 0..operations.get() {
            // Standardizes bit order: `11 => 1101 0000 0000 0000 ...`
            let standard = i.to_le().reverse_bits();
            // Convert to string, iterate over to form operations...
            println!("{i} {:0>64b}", standard);
        }

        todo!()
    }
}

pub enum Operation {
    Add,
    Multiply,
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
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operation::Add),
            '*' => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}

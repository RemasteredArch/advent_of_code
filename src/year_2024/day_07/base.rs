use super::equation::Operation;

/// Convert to binary string, truncate to the relevant length, and convert to [`Operation`]s.
pub fn to_binary_operations(int: usize, length: usize) -> Vec<Operation> {
    ternary_to_operations(
        format!(
            "{:0>width$b}",
            standardize(int),
            width = usize::BITS as usize
        )
        .chars()
        .take(length)
        .collect(),
    )
}

/// Convert to ternary string and truncate to the relevant length.
pub fn to_ternary_operations(int: usize, length: usize) -> Vec<Operation> {
    ternary_to_operations(to_base(3, int, length))
}

/// Convert from a ternary (or binary) string into [`Operation`]s.
fn ternary_to_operations(str: String) -> Vec<Operation> {
    str.chars().filter_map(Operation::from_ternary).collect()
}

/// Standardizes bit order: `11 => 1101 0000 0000 0000 ...`.
///
/// Makes the assumption that the most significant bit is always first, regardless of byte
/// endianness.
fn standardize(int: usize) -> usize {
    int.to_le().reverse_bits()
}

fn to_base(base: usize, int: usize, length: usize) -> String {
    let mut str = "".to_string();
    let mut int = int;

    while int > 0 {
        let remainder = int % base;
        int /= base;

        str.push_str(&remainder.to_string());
    }

    format!("{str:0<width$}", width = length)
}

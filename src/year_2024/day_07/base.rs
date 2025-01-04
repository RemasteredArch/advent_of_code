use super::equation::Operation;

pub fn to_base_operations(base: usize, int: usize, length: usize) -> Vec<Operation> {
    let mut str = "".to_string();
    let mut int = int;

    while int > 0 {
        let remainder = int % base;
        int /= base;

        str.push_str(&remainder.to_string());
    }

    format!("{str:0<width$}", width = length)
        .chars()
        .filter_map(Operation::from_ternary)
        .collect()
}

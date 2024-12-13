mod stack;
use stack::{DoStack, DontStack, MulStack, Stack};

const INPUT: &str = include_str!("./data.txt");
const _INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const __INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

pub fn part_one() -> u32 {
    let mut stack = MulStack::new();
    let mut total: u32 = 0;

    for char in INPUT.chars() {
        total += stack.eval_if_valid();
        stack.push_or_clear(char);
    }

    total
}

pub fn part_two() -> u32 {
    let mut mul_stack = MulStack::new();
    let mut total: u32 = 0;

    let mut do_stack = DoStack::new();
    let mut dont_stack = DontStack::new();
    let mut do_mode = true;

    for char in INPUT.chars() {
        if do_stack.clear_if_valid() {
            do_mode = true;
        }
        if dont_stack.clear_if_valid() {
            do_mode = false;
        }

        total += mul_stack.eval_if_valid();

        if !do_mode {
            do_stack.push_or_clear(char);
            continue;
        }
        mul_stack.push_or_clear(char);
        dont_stack.push_or_clear(char);
    }

    total
}

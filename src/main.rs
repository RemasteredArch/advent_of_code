#![warn(clippy::nursery, clippy::pedantic)]

mod year_2024;

use paste::paste;

type Integer = u64;

fn main() {
    macro_rules! days {
        [ $( $day_str:expr, $day_num:ident; )+ ] => {
            println!("Year 2024");

            $(
                println!(concat!("- Day ", $day_str));
                paste! {
                    println!("  - Part One: {}", year_2024::[<day $day_num>]::part_one());
                    println!("  - Part Two: {}", year_2024::[<day $day_num>]::part_two())
                }
            );+
        };
    }

    days![
        "One", _01;
        "Two", _02;
        "Three", _03;
        "Four", _04;
        "Five", _05;
        // "Six", _06;
        // "Seven", _07;
        "Eight", _08;
        "Nine", _09;
    ];
}

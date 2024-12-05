mod year_2024;

// TODO: macro wizardry to make this less manual

#[allow(unused_variables)]
fn main() {
    let mut args = std::env::args();

    // TODO: Use CLI args to automate selection of target.
    let year = args.nth(1);
    let day = args.nth(2);
    let part = args.nth(3);

    println!("Year 2024");
    println!("- Day One");
    println!("  - Part One: {}", year_2024::day_one::part_one());
    println!("  - Part Two: {}", year_2024::day_one::part_two());
    println!("- Day Two");
    println!("  - Part One: {}", year_2024::day_two::part_one());
    println!("  - Part Two: {}", year_2024::day_two::part_two());
    println!("- Day Three");
    println!("  - Part One: {}", year_2024::day_three::part_one());
    println!("  - Part Two: {}", year_2024::day_three::part_two());
    println!("- Day four");
    println!("  - Part One: {}", year_2024::day_four::part_one());
    // println!("  - Part Two: {}", year_2024::day_four::part_two());
}

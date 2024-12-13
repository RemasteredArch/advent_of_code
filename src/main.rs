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
    println!("  - Part One: {}", year_2024::day_01::part_one());
    println!("  - Part Two: {}", year_2024::day_01::part_two());
    println!("- Day Two");
    println!("  - Part One: {}", year_2024::day_02::part_one());
    println!("  - Part Two: {}", year_2024::day_02::part_two());
    println!("- Day Three");
    println!("  - Part One: {}", year_2024::day_03::part_one());
    println!("  - Part Two: {}", year_2024::day_03::part_two());
    println!("- Day four");
    println!("  - Part One: {}", year_2024::day_04::part_one());
    println!("  - Part Two: {}", year_2024::day_04::part_two());
    println!("- Day five");
    println!("  - Part One: {}", year_2024::day_05::part_one());
    println!("  - Part Two: {}", year_2024::day_05::part_two());
    println!("- Day six");
    println!("  - Part One: {}", year_2024::day_06::part_one());
    // println!("  - Part Two: {}", year_2024::day_06::part_two());
}

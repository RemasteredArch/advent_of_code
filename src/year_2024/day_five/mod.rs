use updates::Updates;

mod updates;

#[allow(unused)]
const INPUT: &str = include_str!("./data.txt");
const _INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

pub fn part_one() -> u32 {
    let updates = Updates::from_str(_INPUT).unwrap();

    updates
        .sorted_updates()
        // Take the sum of all the middle values.
        .filter_map(|update| update.get(update.len() / 2))
        .sum()
}

pub fn part_two() -> u32 {
    let mut updates: Updates<u32> = Updates::from_str(_INPUT).unwrap();

    updates
        .unsorted_updates_mut()
        .map(|update| {
            update.sort();
            update
        })
        // Take the sum of all the middle values.
        .filter_map(|update| update.get(update.len() / 2))
        .sum()
}

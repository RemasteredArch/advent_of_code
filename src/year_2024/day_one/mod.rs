// TODO: Make days into traits.
//
// TODO: Proper error handling!

const INPUT: &str = include_str!("./data.txt");

pub fn part_one() -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = unzip(INPUT);

    // Sort the two vectors.
    left.sort_unstable();
    right.sort_unstable();

    // Pair either side back up, get the difference of each pair, then get the sum of those
    // differences.
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.max(right) - left.min(right))
        .sum()
}

pub fn part_two() -> u32 {
    let (mut left, mut right) = unzip(INPUT);

    // Sort the two vectors.
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .map(|location_id| {
            // Returns the number of times [`location_id`] appears in [`right`] times
            // [`location_id`].
            location_id
                * right.iter().fold(0, |count, next| {
                    if next == location_id {
                        count + 1
                    } else {
                        count
                    }
                })
        })
        .sum()
}

/// Convert `/\d+ +\d+/` (number spaces number) two vectors of [`u32`] for the two sides.
fn unzip(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let sliced = line
                .split(' ')
                .filter(|str| !str.is_empty())
                .map(|str| {
                    str.parse::<u32>()
                        .expect("a number, spaces, and then one more number")
                })
                .collect::<Vec<u32>>();
            let [left, right] = sliced.as_slice() else {
                panic!("Malformed input; expected a number, spaces, and then one more number")
            };
            (*left, *right)
        })
        .unzip()
}

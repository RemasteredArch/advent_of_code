use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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

/// Whether some right hand value should be stored to the right or the left of some left hand
/// value.
enum Ordering {
    /// The given value should be stored to the left of the initial value.
    Left,
    /// The given value should be stored to the right of the initial value.
    Right,
}

/// For a given value of type [`T`], store a set of values that should be sorted to its left or to
/// its right.
///
/// Does not actually store the value of [`T`], as this is design for used in a [`HashMap`], as in
/// [`RuleMap`].
#[derive(Debug, Clone)]
struct Rules<T: Ord + Hash> {
    /// Every value that should be placed to the left of a value.
    left: HashSet<T>,
    /// Every value that should be placed to the right of a value.
    right: HashSet<T>,
}
impl<T: Ord + Hash> Rules<T> {
    #[allow(unused)]
    fn new() -> Self {
        Self {
            left: HashSet::new(),
            right: HashSet::new(),
        }
    }

    fn from_left(left: T) -> Self {
        Self {
            left: HashSet::from([left]),
            right: HashSet::new(),
        }
    }

    fn from_right(right: T) -> Self {
        Self {
            left: HashSet::new(),
            right: HashSet::from([right]),
        }
    }

    /// Returns whether [`rhs`] should be to the left or the right of the value associated with
    /// [`Self`], or [`None`] if there is no rule.
    #[allow(unused)]
    fn cmp(&self, rhs: &T) -> Option<Ordering> {
        if self.left().contains(rhs) {
            return Some(Ordering::Left);
        }

        if self.right().contains(rhs) {
            return Some(Ordering::Right);
        }

        None
    }

    fn left(&self) -> &HashSet<T> {
        &self.left
    }

    fn left_mut(&mut self) -> &mut HashSet<T> {
        &mut self.left
    }

    fn right(&self) -> &HashSet<T> {
        &self.right
    }

    fn right_mut(&mut self) -> &mut HashSet<T> {
        &mut self.right
    }
}

/// For a given value, store a map of other values and whether they are greater or lesser than the
/// given value.
type RuleMap = HashMap<u32, Rules<u32>>;
type Update = Box<[u32]>;

pub fn part_one() -> u32 {
    let (rules, updates) = parse(INPUT).unwrap();

    updates
        .iter()
        // For each update, check that its values are in the correct ordering and return its middle
        // value.
        .filter_map(|update| {
            for (index, value) in update.iter().enumerate() {
                // Treat a value as incorrectly sorted if there's no rules on sorting it.
                let rules = rules.get(value)?;

                // Check that every value to the left of [`value`] should actually be to its left.
                if update
                    .iter()
                    .take(index)
                    .skip_while(|prev| rules.left().contains(prev))
                    .count()
                    != 0
                {
                    return None;
                }

                // Check that every value to the right of [`value`] should actually be to its
                // right.
                if update
                    .iter()
                    .skip(index + 1)
                    .skip_while(|after| rules.right().contains(after))
                    .count()
                    != 0
                {
                    return None;
                }
            }

            // All values are valid, return the middle value.
            update.get(update.len() / 2)
        })
        // Take the sum of all the middle values.
        .sum()
}

// This is, indeed, super evil. But it works!
fn parse(input: &str) -> Option<(RuleMap, Box<[Update]>)> {
    let mut lines = input.lines();

    let mut rules: RuleMap = HashMap::new();

    // Parse rules. Expecting lines of `01|23`.
    for line in lines.by_ref() {
        let Some((lhs, rhs)) = line.split_once('|') else {
            // Rules are ended with a blank line.
            break;
        };

        let lhs: u32 = lhs.parse().ok()?;
        let rhs: u32 = rhs.parse().ok()?;

        match rules.get_mut(&lhs) {
            Some(map) => {
                map.right_mut().insert(rhs);
            }
            None => {
                rules.insert(lhs, Rules::from_right(rhs));
            }
        }

        match rules.get_mut(&rhs) {
            Some(map) => {
                map.left_mut().insert(lhs);
            }
            None => {
                rules.insert(rhs, Rules::from_left(lhs));
            }
        }
    }

    // Parse updates. Expecting lines of `01,23,45,67`.
    let updates = lines
        .map(|line| {
            line.split(',')
                .flat_map(str::parse)
                .collect::<Vec<u32>>()
                .into_boxed_slice()
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    Some((rules, updates))
}

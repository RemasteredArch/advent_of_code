use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

#[derive(Debug)]
pub struct Updates<T: Ord + Hash + FromStr + Copy + Debug> {
    /// For a given value, store a map of other values and whether they are greater or lesser than the
    /// given value.
    rules: RulesMap<T>,
    updates: Box<[Update<T>]>,
}

impl<T: Ord + Hash + FromStr + Copy + Debug> Updates<T> {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        let mut rules = RulesMap::new();

        // Parse rules. Expecting lines of `01|23`.
        for line in lines.by_ref() {
            let Some((lhs, rhs)) = line.split_once('|') else {
                // Rules are ended with a blank line.
                break;
            };

            let lhs: T = lhs.parse().ok()?;
            let rhs: T = rhs.parse().ok()?;

            rules.push_or_insert(lhs, rhs);
        }

        // Parse updates. Expecting lines of `01,23,45,67`.
        let updates = lines
            .map(|line| {
                line.split(',')
                    .flat_map(str::parse)
                    .collect::<Vec<_>>()
                    .into()
            })
            .collect::<Vec<Update<T>>>()
            .into_boxed_slice();

        Some(Self { rules, updates })
    }

    pub fn sorted_updates(&self) -> impl Iterator<Item = &Update<T>> {
        self.updates
            .iter()
            .filter(|update| update.is_sorted(&self.rules))
    }

    #[allow(unused)]
    pub fn unsorted_updates(&self) -> impl Iterator<Item = &Update<T>> {
        self.updates
            .iter()
            .filter(|update| !update.is_sorted(&self.rules))
    }

    pub fn unsorted_updates_mut(&mut self) -> impl Iterator<Item = &mut Update<T>> {
        self.updates
            .iter_mut()
            .filter(|update| !update.is_sorted(&self.rules))
    }
}

/// For a given value of type [`T`], store a set of values that should be sorted to its left or to
/// its right.
///
/// Does not actually store the value of [`T`], as this is design for used in a [`HashMap`], as in
/// [`RuleMap`].
#[derive(Debug, Clone)]
pub struct Rules<T: Ord + Hash + Copy + Debug> {
    /// Every value that should be placed to the left of a value.
    left: HashSet<T>,
    /// Every value that should be placed to the right of a value.
    right: HashSet<T>,
}

impl<T: Ord + Hash + Copy + Debug> Rules<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            left: HashSet::new(),
            right: HashSet::new(),
        }
    }

    pub fn from_left(left: T) -> Self {
        Self {
            left: HashSet::from([left]),
            right: HashSet::new(),
        }
    }

    pub fn from_right(right: T) -> Self {
        Self {
            left: HashSet::new(),
            right: HashSet::from([right]),
        }
    }

    /// Returns whether [`rhs`] should be to the left or the right of the value associated with
    /// [`Self`], or [`None`] if there is no rule.
    #[allow(unused)]
    pub fn cmp(&self, rhs: &T) -> Option<Ordering> {
        if self.left().contains(rhs) {
            return Some(Ordering::Left);
        }

        if self.right().contains(rhs) {
            return Some(Ordering::Right);
        }

        None
    }

    pub fn left(&self) -> &HashSet<T> {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut HashSet<T> {
        &mut self.left
    }

    pub fn right(&self) -> &HashSet<T> {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut HashSet<T> {
        &mut self.right
    }
}

#[derive(Debug, Clone)]
pub struct RulesMap<T: Ord + Hash + Copy + Debug> {
    inner: HashMap<T, Rules<T>>,
}

impl<T: Ord + Hash + Copy + Debug> RulesMap<T> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: &T) -> Option<&Rules<T>> {
        self.inner.get(key)
    }

    pub fn push_or_insert(&mut self, left: T, right: T) {
        match self.inner.get_mut(&left) {
            Some(map) => {
                map.right_mut().insert(right);
            }
            None => {
                self.inner.insert(left, Rules::from_right(right));
            }
        }

        match self.inner.get_mut(&right) {
            Some(map) => {
                map.left_mut().insert(left);
            }
            None => {
                self.inner.insert(right, Rules::from_left(left));
            }
        }
    }
}

#[derive(Debug)]
pub struct Update<T: Ord + Hash + Copy + Debug> {
    inner: Box<[T]>,
}

impl<T: Ord + Hash + Copy + Debug> Update<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_sorted(&self, rules: &RulesMap<T>) -> bool {
        let update = &self.inner;

        for (index, value) in update.iter().enumerate() {
            // Treat a value as incorrectly sorted if there's no rules on sorting it.
            let Some(rules) = rules.get(value) else {
                println!("No rules!");
                return false;
            };

            // Check that every value to the left of [`value`] should actually be to its left.
            if update
                .iter()
                .take(index)
                .skip_while(|prev| rules.left().contains(prev))
                .count()
                != 0
            {
                return false;
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
                return false;
            }
        }

        // All values are valid, return the middle value.
        true
    }

    pub fn sort(&mut self) {
        todo!();
    }
}

impl<T: Ord + Hash + Copy + Debug> From<Vec<T>> for Update<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            inner: value.into_boxed_slice(),
        }
    }
}

/// Whether some right hand value should be stored to the right or the left of some left hand
/// value.
pub enum Ordering {
    /// The given value should be stored to the left of the initial value.
    Left,
    /// The given value should be stored to the right of the initial value.
    Right,
}

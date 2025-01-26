#! /usr/bin/env bash

# `new_day.sh`: sets up all the boilerplate for a new day.
#
# Example usage:
#
# - `./new_day.sh Twelve 12`
# - `./new_day.sh One 01 2023`

set -euo pipefail

cd "$(readlink "$(dirname "$0")" )"

main='src/main.rs'

# E.g., `Twelve`.
day_word="$1"

# E.g., `12`.
day_number="$2"

year="${3:-2024}"

directory="src/year_$year/day_$day_number"

# Enumerate the lines of `$main`, and select the last entry in the `days!` macro.
last_entry="$(nl -pb a -w 1 -s ':' "$main" | grep '^[0-9]\+: *"[^"]\+", _[0-9][0-9];$' | tail -n 1)"

# POSIX search and replace doesn't have backreferences.
#
# shellcheck disable=SC2001
line_number="$(echo "$last_entry" | sed 's/^\([0-9]\+\).*/\1/')"
line_number=$((line_number + 1))

# POSIX search and replace doesn't have backreferences.
#
# shellcheck disable=SC2001
leading_spaces="$(echo "$last_entry" | sed 's/^[0-9]\+:\( \+\)".*$/\1/')"

# Constructs the next entry in the `days!` macro for  `$main`.
#
# E.g., `    "Twelve", _12;`
next_line="$leading_spaces\"$day_word\", _$day_number;"

# Inserts the next entry after the current last entry in `$main`.
sed -i "${line_number}i\\$next_line" "$main"

mkdir "$directory"

touch "$directory/data.txt"

cat > "$directory/mod.rs" << 'EOF'
use crate::Integer;

const INPUT: &str = include_str!("./data.txt");
const EXAMPLE_INPUT: &str = "";

pub fn part_one() -> Integer {
    todo!("implement part one")
}

pub fn part_two() -> Integer {
    todo!("implement part two")
}
EOF

echo "pub mod day_$day_number;" >> "src/year_$year/mod.rs"

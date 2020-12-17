use std::collections::{hash_map::Entry, HashMap};

fn parse_input(s: &str) -> Option<Vec<u32>> {
    s.split(',').map(|num| num.trim().parse().ok()).collect()
}

#[allow(clippy::cast_possible_truncation)]
fn run_n_rounds_with(starting_nums: impl IntoIterator<Item = u32>, end_turn: u32) -> u32 {
    let mut nums = HashMap::with_capacity(1024 * 256);

    let mut cur = 0;
    for (last_turn, num) in starting_nums.into_iter().enumerate() {
        nums.insert(num, last_turn as u32 + 1);
        cur = num;
    }
    // remove the last turn so we can continue from there
    nums.remove(&cur);

    for last_turn in nums.len() as u32 + 1..end_turn {
        match nums.entry(cur) {
            Entry::Occupied(mut occup) => cur = last_turn - occup.insert(last_turn),
            Entry::Vacant(vacant) => {
                vacant.insert(last_turn);
                cur = 0;
            }
        }
    }

    cur
}

fn first_part(starting_nums: impl IntoIterator<Item = u32>) -> u32 {
    run_n_rounds_with(starting_nums, 2020)
}

fn second_part(starting_nums: impl IntoIterator<Item = u32>) -> u32 {
    run_n_rounds_with(starting_nums, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "0,3,6";

    #[test]
    fn first() {
        let nums = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(first_part(nums), 436);
    }

    #[cfg(feature = "tests-slow")]
    #[test]
    fn second() {
        let nums = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(second_part(nums), 175_594);
    }
}

use std::collections::{HashSet, VecDeque};

const PREAMBLE_LEN: usize = 25;

#[derive(Debug, Eq, PartialEq)]
struct Solution {
    first: u64,
    second: u64,
}
impl Solution {
    fn find_first_sequence_break(
        transmission: &[u64],
        preamble_len: usize,
    ) -> Option<(usize, u64)> {
        let mut it = transmission.iter().copied();
        let mut origins = it.by_ref().take(preamble_len).collect::<HashSet<_>>();
        for (i, n) in it.enumerate() {
            if !origins.iter().any(|&o| n > o && origins.contains(&(n - o))) {
                return Some((i, n));
            }
            origins.remove(&transmission[i]);
            origins.insert(n);
        }

        None
    }

    fn find_encryption_weakness(prev_transmission: &[u64], target: u64) -> Option<u64> {
        let mut queue = VecDeque::new();
        let mut sum = 0;
        for &n in prev_transmission {
            if n >= target {
                continue;
            }
            // increase "slice" to the right
            sum += n;
            queue.push_back(n);
            // shrink slice from the left if we've exceeded the target sum
            while sum > target {
                sum -= queue.pop_front().unwrap();
            }
            if sum == target {
                break;
            }
        }

        // too tired to optimize this
        Some(queue.iter().min()? + queue.iter().max()?)
    }

    fn solve_with_preamble(transmission: &[u64], preamble_len: usize) -> Option<Self> {
        let (i, first) = Self::find_first_sequence_break(transmission, preamble_len)?;
        let second = Self::find_encryption_weakness(&transmission[..i], first)?;
        Some(Self { first, second })
    }
}

fn parse_input(s: &str) -> Option<Vec<u64>> {
    s.split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    "#;

    #[test]
    fn solve() {
        let solution = Solution::solve_with_preamble(
            &parse_input(EXAMPLE_INPUT).expect("failed to parse input"),
            5,
        )
        .expect("failed to solve");
        assert_eq!(
            solution,
            Solution {
                first: 127,
                second: 62
            }
        );
    }
}

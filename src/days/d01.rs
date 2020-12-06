use std::{collections::HashSet, num::ParseIntError};

#[derive(Eq, PartialEq)]
struct Solution {
    entries: Vec<u16>,
    result: u32,
}
impl Solution {
    fn from_entries(entries: Vec<u16>) -> Self {
        let result = entries.iter().copied().map(u32::from).product();
        Self { entries, result }
    }

    fn solve_n(report: &HashSet<u16>, target: u16, n: u8) -> Option<Self> {
        fn inner(report: &HashSet<u16>, target: u16, entries: &mut [u16], sum: u16) -> bool {
            match entries.len() {
                0 => false,
                1 => {
                    let missing = match target.checked_sub(sum) {
                        Some(v) => v,
                        None => return false,
                    };
                    entries[0] = missing;
                    report.contains(&missing)
                }
                _ => {
                    for &entry in report {
                        if inner(report, target, &mut entries[1..], sum + entry) {
                            entries[0] = entry;
                            return true;
                        }
                    }
                    false
                }
            }
        }

        let mut entries = vec![0; n as usize];
        if inner(report, target, &mut entries, 0) {
            Some(Self::from_entries(entries))
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Result<HashSet<u16>, ParseIntError> {
    input
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        1721
        979
        366
        299
        675
        1456
    "#;

    fn example_input() -> HashSet<u16> {
        parse_input(EXAMPLE_INPUT).expect("failed to parse input")
    }

    #[test]
    fn first() {
        let sol = Solution::solve_n(&example_input(), 2020, 2).expect("failed to solve");
        assert_eq!(sol.result, 514579);
    }

    #[test]
    fn second() {
        let sol = Solution::solve_n(&example_input(), 2020, 3).expect("failed to solve");
        assert_eq!(sol.result, 241861950);
    }
}

use lazy_static::lazy_static;
use regex::Regex;

struct Policy {
    letter: char,
    min: u16,
    max: u16,
}
impl Policy {
    fn valid_count(&self, password: &str) -> bool {
        let count = password.matches(self.letter).count() as u16;
        (self.min..=self.max).contains(&count)
    }

    fn valid_index(&self, password: &str) -> bool {
        let is_eq = |l| l == self.letter;
        password
            .chars()
            .nth(self.min as usize - 1)
            .map_or(false, is_eq)
            ^ password
                .chars()
                .nth(self.max as usize - 1)
                .map_or(false, is_eq)
    }
}

struct Entry<'a> {
    policy: Policy,
    password: &'a str,
}
impl<'a> Entry<'a> {
    fn from_str(s: &'a str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)$"#)
                    .unwrap();
        }

        let captures = RE.captures(s)?;
        let min = captures.name("min")?.as_str().parse().ok()?;
        let max = captures.name("max")?.as_str().parse().ok()?;
        let letter = captures.name("letter")?.as_str().chars().next().unwrap();
        let password = captures.name("password")?.as_str();

        Some(Self {
            policy: Policy { min, max, letter },
            password,
        })
    }

    fn valid_count(&self) -> bool {
        self.policy.valid_count(self.password)
    }

    fn valid_index(&self) -> bool {
        self.policy.valid_index(self.password)
    }
}

type Entries<'a> = Vec<Entry<'a>>;

fn parse_input(s: &str) -> Option<Entries> {
    s.lines()
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .map(Entry::from_str)
        .collect::<Option<_>>()
}

struct Solution {
    valid_count: usize,
    valid_index: usize,
}
impl Solution {
    fn solve<'a>(entries: impl IntoIterator<Item = Entry<'a>>) -> Self {
        let (valid_count, valid_index) = entries.into_iter().fold((0, 0), |(c, i), entry| {
            (
                c + entry.valid_count() as usize,
                i + entry.valid_index() as usize,
            )
        });
        Self {
            valid_count,
            valid_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "#;

    fn example_input() -> Entries<'static> {
        parse_input(EXAMPLE_INPUT).expect("failed to parse input")
    }

    #[test]
    fn solve() {
        let sol = Solution::solve(example_input());
        assert_eq!(sol.valid_count, 2);
        assert_eq!(sol.valid_index, 1);
    }
}

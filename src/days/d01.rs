use std::collections::HashSet;
use yew::prelude::*;

#[derive(Eq, PartialEq)]
struct Solution {
    entries: Vec<u16>,
    result: u32,
}
impl Solution {
    fn from_entries(entries: Vec<u16>) -> Self {
        let result = entries.iter().copied().map(|e| e as u32).product();
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

struct Day01 {
    link: ComponentLink<Self>,
    expense_report: Vec<usize>,
}
impl Component for Day01 {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            expense_report: Vec::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    const EXAMPLE_INPUT: &str = r#"
        1721
        979
        366
        299
        675
        1456
    "#;

    fn example_input<T>() -> T
    where
        T: FromIterator<u16>,
    {
        EXAMPLE_INPUT
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<T, _>>()
            .expect("failed to parse example input")
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

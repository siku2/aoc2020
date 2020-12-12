use std::{collections::HashSet, num::ParseIntError};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

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

pub enum Msg {
    Solve,
    SetTarget(InputData),
    SetN(InputData),
}

pub struct Page {
    link: ComponentLink<Self>,
    input_area: NodeRef,
    target: u16,
    n: u8,
    solution: Option<Solution>,
}
impl Page {
    fn render_solution(&self) -> Html {
        if let Some(solution) = &self.solution {
            html! { <code>{ &solution.result }</code> }
        } else {
            html! {}
        }
    }
}
impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input_area: NodeRef::default(),
            target: 2020,
            n: 2,
            solution: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Solve => {
                let input_area = self.input_area.cast::<HtmlTextAreaElement>().unwrap();
                let input = parse_input(&input_area.value());
                if let Ok(input) = input {
                    self.solution = Solution::solve_n(&input, self.target, self.n);
                    if self.solution.is_some() {
                        input_area.set_custom_validity("");
                    } else {
                        input_area.set_custom_validity("failed to solve");
                    }
                } else {
                    self.solution = None;
                    input_area.set_custom_validity("failed to parse");
                }

                true
            }
            Msg::SetTarget(data) => {
                if let Ok(value) = data.value.parse() {
                    self.target = value;
                    true
                } else {
                    false
                }
            }
            Msg::SetN(data) => {
                if let Ok(value) = data.value.parse() {
                    self.n = value;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let Self {
            link,
            input_area,
            target,
            n,
            ..
        } = &self;
        html! {
            <>
                <input type="number" min="0" value=target oninput=link.callback(Msg::SetTarget) />
                <input type="number" min="0" max="10" value=n oninput=link.callback(Msg::SetN) />
                <textarea ref=input_area.clone() />
                <button onclick=link.callback(|_| Msg::Solve)>{ "solve" }</button>
                { self.render_solution() }
            </>
        }
    }
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
        assert_eq!(sol.result, 514_579);
    }

    #[test]
    fn second() {
        let sol = Solution::solve_n(&example_input(), 2020, 3).expect("failed to solve");
        assert_eq!(sol.result, 241_861_950);
    }
}

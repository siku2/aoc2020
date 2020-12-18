use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
    ops::{Add, Mul},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operator {
    Add,
    Mul,
    GroupOpen,
    GroupClose,
}
impl Operator {
    fn perform<T, R>(self, a: T, b: T) -> Option<R>
    where
        T: Add<Output = R> + Mul<Output = R>,
    {
        match self {
            Operator::Add => Some(a + b),
            Operator::Mul => Some(a * b),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Op(Operator),
    Digit(u8),
}
impl Token {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Op(Operator::Add),
            '*' => Self::Op(Operator::Mul),
            '(' => Self::Op(Operator::GroupOpen),
            ')' => Self::Op(Operator::GroupClose),
            // PANIC: a single digit will certainly fit into an u8
            _ => Self::Digit(c.to_digit(10).unwrap_or_default().try_into().unwrap()),
        }
    }
}

struct ExprPostfix(Vec<Token>);
impl ExprPostfix {
    fn tokenize<'a>(s: &'a str) -> impl Iterator<Item = Token> + 'a {
        s.chars().filter_map(|c| {
            if c.is_whitespace() {
                None
            } else {
                Some(Token::from_char(c))
            }
        })
    }

    fn from_input(s: &str, precedence: &HashMap<Operator, usize>) -> Option<Self> {
        let mut operators = VecDeque::new();
        let mut output = Vec::new();

        for token in Self::tokenize(s) {
            match token {
                Token::Op(op) => match op {
                    Operator::Add | Operator::Mul => {
                        while !operators.is_empty()
                            && operators.back() != Some(&Operator::GroupOpen)
                            && (precedence.get(operators.back()?) >= precedence.get(&op))
                        {
                            // PANIC: made sure operators isn't empty
                            output.push(Token::Op(operators.pop_back().unwrap()));
                        }
                        operators.push_back(op);
                    }

                    Operator::GroupOpen => operators.push_back(op),
                    Operator::GroupClose => {
                        while let Some(token) = operators.pop_back() {
                            if token == Operator::GroupOpen {
                                break;
                            }
                            output.push(Token::Op(token));
                        }
                    }
                },
                Token::Digit(_) => {
                    output.push(token);
                }
            }
        }

        while let Some(token) = operators.pop_back() {
            output.push(Token::Op(token));
        }

        Some(ExprPostfix(output))
    }

    fn calculate(&self) -> Option<u64> {
        let mut stack = VecDeque::new();
        for &token in &self.0 {
            match token {
                Token::Digit(n) => {
                    stack.push_back(u64::from(n));
                }
                Token::Op(op) => {
                    let a = stack.pop_back()?;
                    let b = stack.pop_back()?;
                    if let Some(res) = op.perform(a, b) {
                        stack.push_back(res);
                    }
                }
            }
        }

        stack.pop_back()
    }
}

fn parse_input(s: &str, precedence: &HashMap<Operator, usize>) -> Option<Vec<ExprPostfix>> {
    s.trim()
        .lines()
        .map(|line| ExprPostfix::from_input(line.trim(), precedence))
        .collect()
}

fn sum_expressions<'a>(exprs: impl IntoIterator<Item = &'a ExprPostfix>) -> u64 {
    exprs
        .into_iter()
        .filter_map(|expr| expr.calculate().map(u64::from))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        const INPUT: &str = r#"
            1 + (2 * 3) + (4 * (5 + 6))
            2 * 3 + (4 * 5)
            5 + (8 * 3 + 9 + 3 * 4 * 3)
            5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
            ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
        "#;
        let precedence = vec![(Operator::Add, 1), (Operator::Mul, 1)]
            .into_iter()
            .collect();
        let exprs = parse_input(INPUT, &precedence).expect("failed to parse input");
        assert_eq!(sum_expressions(&exprs), 51 + 26 + 437 + 12_240 + 13_632);
    }
    #[test]
    fn second() {
        const INPUT: &str = r#"
            1 + (2 * 3) + (4 * (5 + 6))
            2 * 3 + (4 * 5)
            5 + (8 * 3 + 9 + 3 * 4 * 3)
            5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
            ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
        "#;
        let precedence = vec![(Operator::Add, 2), (Operator::Mul, 1)]
            .into_iter()
            .collect();
        let exprs = parse_input(INPUT, &precedence).expect("failed to parse input");
        assert_eq!(sum_expressions(&exprs), 51 + 46 + 1_445 + 669_060 + 23_340);
    }
}

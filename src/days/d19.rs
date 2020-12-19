use std::collections::HashMap;

struct Matcher<'a> {
    rules: &'a RuleSet,
    chars: Vec<char>,
    indices: Vec<usize>,
}
impl<'a> Matcher<'a> {
    fn new_for_string(rules: &'a RuleSet, s: &str) -> Self {
        let chars = s.chars().collect();
        Self {
            rules,
            chars,
            indices: vec![0],
        }
    }

    fn string_matches_rule(rules: &RuleSet, s: &str, rule: u16) -> bool {
        let mut matcher = Matcher::new_for_string(rules, s);
        let is_match = matcher.match_rule_ref(rule);
        is_match && matcher.is_complete()
    }

    fn is_complete(&self) -> bool {
        self.indices.iter().any(|&index| index == self.chars.len())
    }

    fn with_restore_index<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        let temp = self.indices.clone();
        let result = f(self);
        self.indices = temp;
        result
    }

    fn match_char(&mut self, expected: char) -> bool {
        let mut to_remove = Vec::with_capacity(self.indices.len());
        for (i, index) in self.indices.iter_mut().enumerate() {
            let actual = self.chars.get(*index);
            if actual == Some(&expected) {
                *index += 1;
            } else {
                to_remove.push(i);
            }
        }

        for i in to_remove.into_iter().rev() {
            self.indices.remove(i);
        }

        !self.indices.is_empty()
    }

    fn match_check(&mut self, check: Check) -> bool {
        match check {
            Check::Ref(rule) => self.match_rule_ref(rule),
            Check::Char(c) => self.match_char(c),
        }
    }

    fn match_rule(&mut self, rule: &Rule) -> bool {
        match rule {
            Rule::Sequence(seq) => seq.iter().all(|&check| self.match_check(check)),
            Rule::Alternatives(alts) => {
                let new_indices = alts
                    .iter()
                    .flat_map(|alt| {
                        self.with_restore_index(|matcher| {
                            if matcher.match_rule(alt) {
                                Some(matcher.indices.clone())
                            } else {
                                None
                            }
                        })
                    })
                    .flatten()
                    .collect();

                self.indices = new_indices;
                !self.indices.is_empty()
            }
        }
    }

    fn match_rule_ref(&mut self, rule: u16) -> bool {
        self.rules
            .get(rule)
            .map_or(false, |rule| self.match_rule(rule))
    }
}

#[derive(Clone, Copy)]
enum Check {
    Ref(u16),
    Char(char),
}
impl Check {
    fn parse_input(s: &str) -> Option<Self> {
        if s.starts_with('"') {
            s.chars().nth(1).map(Self::Char)
        } else {
            s.parse().ok().map(Self::Ref)
        }
    }
}

enum Rule {
    Sequence(Vec<Check>),
    Alternatives(Vec<Self>),
}
impl Rule {
    fn parse_sequence(s: &str) -> Option<Self> {
        s.split_whitespace()
            .map(Check::parse_input)
            .collect::<Option<_>>()
            .map(Self::Sequence)
    }

    fn parse_input(s: &str) -> Option<Self> {
        let alts = s
            .split('|')
            .map(|alt| Self::parse_sequence(alt))
            .collect::<Option<Vec<_>>>()?;
        if alts.len() > 1 {
            Some(Self::Alternatives(alts))
        } else {
            alts.into_iter().next()
        }
    }
}

struct RuleSet {
    rules: HashMap<u16, Rule>,
}
impl RuleSet {
    fn parse_input_line(line: &str) -> Option<(u16, Rule)> {
        let mut it = line.split(':');
        let index = it.next()?.trim().parse().ok()?;
        let rule = Rule::parse_input(it.next()?.trim())?;
        Some((index, rule))
    }

    fn parse_input(s: &str) -> Option<Self> {
        let rules = s
            .lines()
            .map(|line| Self::parse_input_line(line.trim()))
            .collect::<Option<_>>()?;
        Some(Self { rules })
    }

    fn get(&self, rule: u16) -> Option<&Rule> {
        self.rules.get(&rule)
    }

    fn update_for_part_two(&mut self) {
        self.rules
            .insert(8, Rule::parse_input("42 | 42 8").unwrap());
        self.rules
            .insert(11, Rule::parse_input("42 31 | 42 11 31").unwrap());
    }
}

fn parse_input<'a>(s: &'a str) -> Option<(RuleSet, Vec<&'a str>)> {
    let mut lines = s.trim().lines().map(str::trim);
    let rules = RuleSet::parse_input(
        &lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n"),
    )?;
    let messages = lines.collect();
    Some((rules, messages))
}

fn first_part<'a>(rules: &RuleSet, messages: impl IntoIterator<Item = &'a str>) -> usize {
    messages
        .into_iter()
        .filter(|msg| Matcher::string_matches_rule(rules, msg, 0))
        .count()
}

fn second_part<'a>(rules: &mut RuleSet, messages: impl IntoIterator<Item = &'a str>) -> usize {
    rules.update_for_part_two();
    first_part(rules, messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        const INPUT: &str = r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"

            ababbb
            bababa
            abbbab
            aaabbb
            aaaabbb
        "#;

        let (rules, messages) = parse_input(INPUT).expect("failed to parse input");
        assert_eq!(first_part(&rules, messages), 2);
    }
    #[test]
    fn second() {
        const INPUT: &str = r#"
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        "#;

        let (mut rules, messages) = parse_input(INPUT).expect("failed to parse input");
        assert_eq!(second_part(&mut rules, messages), 12);
    }
}

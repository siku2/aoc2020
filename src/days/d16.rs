use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

fn parse_range(s: &str) -> Option<RangeInclusive<u16>> {
    let mut it = s.split('-');
    let low = it.next()?.parse().ok()?;
    let high = it.next()?.parse().ok()?;
    Some(low..=high)
}

#[derive(Debug)]
struct Rule(RangeInclusive<u16>, RangeInclusive<u16>);
impl Rule {
    fn parse_input(s: &str) -> Option<Self> {
        let mut it = s.split("or").flat_map(|s| parse_range(s.trim()));
        let first = it.next()?;
        let second = it.next()?;
        Some(Self(first, second))
    }

    fn contains(&self, n: u16) -> bool {
        self.0.contains(&n) || self.1.contains(&n)
    }
}

#[derive(Debug)]
struct TicketRules<'a>(HashMap<&'a str, Rule>);
impl<'a> TicketRules<'a> {
    fn parse_single(s: &'a str) -> Option<(&'a str, Rule)> {
        let mut it = s.split(':');
        let name = it.next()?.trim();
        let rule = Rule::parse_input(it.next()?.trim())?;
        Some((name, rule))
    }

    fn parse_input(s: &'a str) -> Option<Self> {
        let mut rules = HashMap::new();
        for line in s.lines().map(str::trim) {
            let (name, rule) = Self::parse_single(line)?;
            rules.insert(name, rule);
        }

        Some(Self(rules))
    }

    fn any_contains(&self, n: u16) -> bool {
        self.0.values().any(|rule| rule.contains(n))
    }

    fn iter_names<'s>(&'s self) -> impl Iterator<Item = &'a str> + 's {
        self.0.keys().copied()
    }

    fn iter_matching_rules<'s>(&'s self, n: u16) -> impl Iterator<Item = &'a str> + 's {
        self.0
            .iter()
            .filter_map(move |(&name, rule)| if rule.contains(n) { Some(name) } else { None })
    }
}

#[derive(Debug)]
struct Ticket(Vec<u16>);
impl Ticket {
    fn from_input(s: &str) -> Option<Self> {
        let values = s
            .split(',')
            .map(|n| n.trim().parse().ok())
            .collect::<Option<Vec<_>>>()?;
        Some(Self(values))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, i: usize) -> Option<u16> {
        self.0.get(i).copied()
    }

    fn iter_values<'a>(&'a self) -> impl Iterator<Item = u16> + 'a {
        self.0.iter().copied()
    }
}

#[derive(Debug)]
struct Input<'a> {
    rules: TicketRules<'a>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}
impl<'a> Input<'a> {
    fn from_input(s: &'a str) -> Option<Self> {
        let mut it = s.split("your ticket:");
        let rules = TicketRules::parse_input(it.next()?.trim())?;
        let mut it = it.next()?.split("nearby tickets:");
        let my_ticket = Ticket::from_input(it.next()?.trim())?;
        let nearby_tickets = it
            .next()?
            .trim()
            .lines()
            .map(|line| Ticket::from_input(line.trim()))
            .collect::<Option<Vec<_>>>()?;
        Some(Self {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }
}

fn first_part(input: &Input) -> u16 {
    let mut err_sum = 0;
    for ticket in &input.nearby_tickets {
        for value in ticket.iter_values() {
            if !input.rules.any_contains(value) {
                err_sum += value;
            }
        }
    }

    err_sum
}

fn second_part(input: &Input) -> Option<u64> {
    let all_rule_names = input.rules.iter_names().collect::<HashSet<_>>();
    let mut rules_for_column: Vec<HashSet<&str>> = vec![all_rule_names; input.my_ticket.len()];

    for ticket in &input.nearby_tickets {
        let mut col = 0;
        for value in ticket.iter_values() {
            let ticket_valid_rules = input
                .rules
                .iter_matching_rules(value)
                .collect::<HashSet<_>>();
            if ticket_valid_rules.is_empty() {
                continue;
            }

            let rules = rules_for_column[col]
                .intersection(&ticket_valid_rules)
                .copied()
                .collect();
            rules_for_column[col] = rules;
            col += 1;
        }
    }

    let mut rules_for_column = rules_for_column.into_iter().enumerate().collect::<Vec<_>>();
    rules_for_column.sort_unstable_by_key(|(_, rules)| rules.len());

    let mut rule_names_taken = HashSet::with_capacity(rules_for_column.len());
    let mut column_to_rule = HashMap::with_capacity(rules_for_column.len());
    for (col, rule) in rules_for_column {
        let name = *rule.difference(&rule_names_taken).next()?;
        rule_names_taken.insert(name);
        column_to_rule.insert(col, name);
    }

    column_to_rule
        .into_iter()
        .filter_map(|(col, name)| {
            if name.starts_with("departure") {
                Some(input.my_ticket.get(col).map(u64::from))
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        const EXAMPLE_INPUT: &str = r#"
            class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50
            
            your ticket:
            7,1,14
            
            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12
        "#;

        let input = Input::from_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(first_part(&input), 71);
    }

    #[test]
    fn second() {
        const EXAMPLE_INPUT: &str = r#"
            class: 0-1 or 4-19
            row: 0-5 or 8-19
            seat: 0-13 or 16-19
            
            your ticket:
            11,12,13
            
            nearby tickets:
            3,9,18
            15,1,5
            5,14,9
        "#;

        let input = Input::from_input(EXAMPLE_INPUT).expect("failed to parse input");
        // no example input given here, let's just make sure that it completes
        assert_eq!(second_part(&input), Some(1));
    }
}

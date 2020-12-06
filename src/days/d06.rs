use std::collections::HashSet;

type Answers = HashSet<char>;

fn parse_answers(s: &str) -> Answers {
    s.chars().collect()
}

type Group = Vec<Answers>;

fn parse_input(s: &str) -> Vec<Group> {
    let mut groups = Vec::default();
    let mut group = Group::new();
    for line in s.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            groups.push(std::mem::take(&mut group));
            continue;
        }

        group.push(parse_answers(line))
    }

    if !group.is_empty() {
        groups.push(group);
    }

    groups
}

fn first_part(groups: Vec<Group>) -> usize {
    groups
        .into_iter()
        .map(|g| g.into_iter().flatten().collect::<HashSet<_>>().len())
        .sum()
}

fn second_part(groups: Vec<Group>) -> usize {
    groups
        .into_iter()
        .map(|g| {
            let mut it = g.into_iter();
            let intr = match it.next() {
                Some(v) => v,
                None => return 0,
            };
            it.fold(intr, |intr, a| intr.intersection(&a).copied().collect())
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b
    "#;

    #[test]
    fn first() {
        assert_eq!(first_part(parse_input(EXAMPLE_INPUT)), 11);
    }

    #[test]
    fn second() {
        assert_eq!(second_part(parse_input(EXAMPLE_INPUT)), 6);
    }
}

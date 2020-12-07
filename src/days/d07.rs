use std::collections::HashMap;

const COLOR_SHINY_GOLD: &str = "shiny gold";

struct BagContents<'a> {
    count: usize,
    color: &'a str,
}
impl<'a> BagContents<'a> {
    fn from_input(s: &'a str) -> Option<Self> {
        let s = s.strip_suffix(" bags").or_else(|| s.strip_suffix(" bag"))?;
        let mut it = s.splitn(2, ' ');
        let count = it.next()?.parse().ok()?;
        let color = it.next()?;

        Some(Self { count, color })
    }
}

fn parse_bag<'a>(s: &'a str) -> Option<(&'a str, Vec<BagContents<'a>>)> {
    let mut it = s.split(" bags contain ");
    let color = it.next()?;
    let raw_contents = it.next()?.strip_suffix('.')?;
    let contents = if raw_contents == "no other bags" {
        Vec::new()
    } else {
        raw_contents
            .split(", ")
            .map(BagContents::from_input)
            .collect::<Option<_>>()?
    };

    Some((color, contents))
}

fn parse_input<'a>(inp: &'a str) -> Option<HashMap<&'a str, Vec<BagContents<'a>>>> {
    inp.trim()
        .lines()
        .map(|line| parse_bag(line.trim()))
        .collect()
}

fn first_part<'a>(bags: &HashMap<&'a str, Vec<BagContents<'a>>>) -> usize {
    fn contains_shiny_gold<'a>(
        bags: &HashMap<&'a str, Vec<BagContents<'a>>>,
        contains_map: &mut HashMap<&'a str, bool>,
        color: &'a str,
    ) -> bool {
        contains_map.get(color).copied().unwrap_or_else(|| {
            let contains = bags.get(color).map_or(false, |contents| {
                contents.iter().any(|content| {
                    content.color == COLOR_SHINY_GOLD
                        || contains_shiny_gold(bags, contains_map, &content.color)
                })
            });

            contains_map.insert(color, contains);
            contains
        })
    }

    let mut contains_map = HashMap::new();
    bags.keys()
        .filter(|color| contains_shiny_gold(bags, &mut contains_map, color))
        .count()
}

fn second_part<'a>(bags: &HashMap<&'a str, Vec<BagContents<'a>>>) -> usize {
    fn count_bags_recursive<'a>(
        bags: &HashMap<&'a str, Vec<BagContents<'a>>>,
        totals: &mut HashMap<&'a str, usize>,
        color: &'a str,
    ) -> usize {
        totals.get(color).copied().unwrap_or_else(|| {
            let total = bags.get(color).map_or(0, |contents| {
                contents
                    .iter()
                    .map(|content| {
                        content.count * (1 + count_bags_recursive(bags, totals, &content.color))
                    })
                    .sum()
            });

            totals.insert(color, total);
            total
        })
    }

    count_bags_recursive(
        bags,
        &mut HashMap::with_capacity(bags.len()),
        COLOR_SHINY_GOLD,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
    "#;

    #[test]
    fn first() {
        assert_eq!(
            first_part(&parse_input(EXAMPLE_INPUT).expect("failed to parse input")),
            4
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            second_part(&parse_input(EXAMPLE_INPUT).expect("failed to parse input")),
            32
        );
    }
}

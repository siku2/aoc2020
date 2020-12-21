use std::collections::{hash_map::Entry, HashMap, HashSet};

type Food<'a> = (HashSet<&'a str>, HashSet<&'a str>);

fn parse_line(s: &str) -> Option<Food> {
    let mut it = s.split("(contains");
    let ingredients = it.next()?.trim().split_whitespace().collect();
    let allergens = it
        .next()?
        .trim()
        .strip_suffix(')')?
        .split(',')
        .map(str::trim)
        .collect();
    Some((ingredients, allergens))
}

fn parse_input(s: &str) -> Option<Vec<Food>> {
    s.trim().lines().map(parse_line).collect()
}

fn determine_ingredients_with_allergens<'a>(
    foods: &[Food<'a>],
) -> Option<HashMap<&'a str, &'a str>> {
    let mut allergen2ingredient: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, allergens) in foods {
        for &allergen in allergens {
            match allergen2ingredient.entry(allergen) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = entry.get().intersection(ingredients).copied().collect();
                }
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
            }
        }
    }

    let mut ingredient2allergen = HashMap::with_capacity(allergen2ingredient.len());
    while !allergen2ingredient.is_empty() {
        let allergens = allergen2ingredient
            .iter()
            .filter_map(|(&allergen, ingredients)| {
                if ingredients.len() <= 1 {
                    Some(allergen)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if allergens.is_empty() {
            return None;
        }

        for allergen in allergens {
            let ingredient = allergen2ingredient.remove(allergen)?.into_iter().next()?;
            allergen2ingredient.values_mut().for_each(|ingredients| {
                ingredients.remove(ingredient);
            });
            ingredient2allergen.insert(ingredient, allergen);
        }
    }

    Some(ingredient2allergen)
}

fn first_part(foods: &[Food]) -> Option<usize> {
    let ingredient_count = foods
        .iter()
        .flat_map(|(ingredients, _)| ingredients.iter())
        .copied()
        .fold(HashMap::<_, usize>::new(), |mut counts, ingredient| {
            *counts.entry(ingredient).or_default() += 1;
            counts
        });

    let ingredient2allergen = determine_ingredients_with_allergens(foods)?;
    let res = ingredient_count
        .into_iter()
        .filter_map(|(ingredient, count)| {
            if ingredient2allergen.contains_key(ingredient) {
                None
            } else {
                Some(count)
            }
        })
        .sum();
    Some(res)
}

fn second_part(foods: &[Food]) -> Option<String> {
    let ingredient2allergen = determine_ingredients_with_allergens(foods)?;
    let mut ingredients = ingredient2allergen.keys().copied().collect::<Vec<_>>();
    ingredients.sort_unstable_by_key(|ingredient| ingredient2allergen[ingredient]);
    Some(ingredients.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    "#;

    #[test]
    fn first() {
        let foods = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(first_part(&foods).expect("failed to solve"), 5);
    }
    #[test]
    fn second() {
        let foods = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(
            second_part(&foods).expect("failed to solve"),
            "mxmxvkd,sqjhc,fvjkl"
        )
    }
}

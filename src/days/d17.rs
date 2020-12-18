use std::{collections::HashSet, convert::TryInto};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Coord(i32, i32, i32, i32);
impl Coord {
    fn new4(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self(x, y, z, w)
    }
    fn new3(x: i32, y: i32, z: i32) -> Self {
        Self::new4(x, y, z, 0)
    }
    fn new2(x: i32, y: i32) -> Self {
        Self::new3(x, y, 0)
    }

    fn iter_neighbors(self, w_enabled: bool) -> impl Iterator<Item = Self> {
        let offsets = (-1..=1)
            .flat_map(|x_off| (-1..=1).map(move |y_off| (x_off, y_off)))
            .flat_map(|(x_off, y_off)| (-1..=1).map(move |z_off| (x_off, y_off, z_off)));

        let offsets: Box<dyn Iterator<Item = (i32, i32, i32, i32)>> = if w_enabled {
            Box::new(offsets.flat_map(|(x_off, y_off, z_off)| {
                (-1..=1).map(move |w_off| (x_off, y_off, z_off, w_off))
            }))
        } else {
            Box::new(offsets.map(|(x_off, y_off, z_off)| (x_off, y_off, z_off, 0)))
        };

        offsets.filter_map(move |(x_off, y_off, z_off, w_off)| {
            if (x_off, y_off, z_off, w_off) == (0, 0, 0, 0) {
                None
            } else {
                Some(Self::new4(
                    self.0 + x_off,
                    self.1 + y_off,
                    self.2 + z_off,
                    self.3 + w_off,
                ))
            }
        })
    }

    fn min_component_wise(self, other: Self) -> Self {
        let Self(x1, y1, z1, w1) = self;
        let Self(x2, y2, z2, w2) = other;
        Self::new4(x1.min(x2), y1.min(y2), z1.min(z2), w1.min(w2))
    }

    fn max_component_wise(self, other: Self) -> Self {
        let Self(x1, y1, z1, w1) = self;
        let Self(x2, y2, z2, w2) = other;
        Self::new4(x1.max(x2), y1.max(y2), z1.max(z2), w1.max(w2))
    }

    fn add_component_wise(self, num: i32, w_enabled: bool) -> Self {
        let Self(x, y, z, mut w) = self;
        if w_enabled {
            w += num;
        }
        Self::new4(x + num, y + num, z + num, w)
    }

    fn iter_up_to_inclusive(self, target: Self) -> impl Iterator<Item = Self> {
        let Self(x1, y1, z1, w1) = self;
        let Self(x2, y2, z2, w2) = target;

        (x1..=x2)
            .flat_map(move |x| (y1..=y2).map(move |y| (x, y)))
            .flat_map(move |(x, y)| (z1..=z2).map(move |z| (x, y, z)))
            .flat_map(move |(x, y, z)| (w1..=w2).map(move |w| Self::new4(x, y, z, w)))
    }
}

fn parse_input(s: &str) -> HashSet<Coord> {
    let mut active_cubes = HashSet::new();
    for (y, line) in s.split_whitespace().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                // SAFETY: it's very unlikely for there to be more than u32::MAX lines / columns
                active_cubes.insert(Coord::new2(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    active_cubes
}

fn cycle(active_cubes: &HashSet<Coord>, w_enabled: bool) -> HashSet<Coord> {
    let mut next_active_cubes = active_cubes.clone();
    let (mut min, mut max) = (Coord::default(), Coord::default());
    for &coord in active_cubes {
        min = min.min_component_wise(coord);
        max = max.max_component_wise(coord);
    }

    min = min.add_component_wise(-1, w_enabled);
    max = max.add_component_wise(1, w_enabled);
    for coord in min.iter_up_to_inclusive(max) {
        let is_active = active_cubes.contains(&coord);
        let neighbor_count = coord
            .iter_neighbors(w_enabled)
            .filter(|coord| active_cubes.contains(&coord))
            .count();

        if is_active && !matches!(neighbor_count, 2 | 3) {
            next_active_cubes.remove(&coord);
        } else if !is_active && neighbor_count == 3 {
            next_active_cubes.insert(coord);
        }
    }

    next_active_cubes
}

fn perform_some_cycles(mut active_cubes: HashSet<Coord>, w_enabled: bool) -> usize {
    for _ in 0..6 {
        active_cubes = cycle(&active_cubes, w_enabled);
    }

    active_cubes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        .#.
        ..#
        ###
    "#;

    #[test]
    fn first() {
        let active_cubes = parse_input(EXAMPLE_INPUT);
        assert_eq!(perform_some_cycles(active_cubes, false), 112);
    }

    #[cfg(feature = "tests-slow")]
    #[test]
    fn second() {
        let active_cubes = parse_input(EXAMPLE_INPUT);
        assert_eq!(perform_some_cycles(active_cubes, true), 848);
    }
}

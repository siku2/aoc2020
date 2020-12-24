use std::{collections::HashSet, str::Chars};

// x, y, z
type Coords = (i16, i16, i16);

fn coords_add(a: Coords, b: Coords) -> Coords {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn coords_iter_neighbours(coords: Coords) -> impl Iterator<Item = Coords> {
    vec![
        Direction::E,
        Direction::SE,
        Direction::SW,
        Direction::W,
        Direction::NW,
        Direction::NE,
    ]
    .into_iter()
    .map(move |dir| coords_add(coords, dir.to_coords()))
}

fn coords_from_directions(directions: impl IntoIterator<Item = Direction>) -> Coords {
    directions
        .into_iter()
        .fold((0, 0, 0), |pos, dir| coords_add(pos, dir.to_coords()))
}

type Directions = Vec<Direction>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}
impl Direction {
    fn read_from_input(chars: &mut Chars) -> Option<Self> {
        let dir = match chars.next()? {
            'e' => Self::E,
            'w' => Self::W,
            's' => match chars.next()? {
                'e' => Self::SE,
                'w' => Self::SW,
                _ => return None,
            },
            'n' => match chars.next()? {
                'e' => Self::NE,
                'w' => Self::NW,
                _ => return None,
            },
            _ => return None,
        };
        Some(dir)
    }

    fn parse_directions(s: &str) -> Option<Directions> {
        let mut chars = s.chars();
        let mut dirs = Vec::with_capacity(s.len());
        while let Some(dir) = Self::read_from_input(&mut chars) {
            dirs.push(dir);
        }

        if chars.next().is_some() {
            None
        } else {
            Some(dirs)
        }
    }

    fn to_coords(self) -> Coords {
        match self {
            // y
            Self::E => (1, 0, -1),
            // z
            Self::SE => (1, -1, 0),
            // x
            Self::SW => (0, -1, 1),
            // -y
            Self::W => (-1, 0, 1),
            // -z
            Self::NW => (-1, 1, 0),
            // -x
            Self::NE => (0, 1, -1),
        }
    }
}

fn parse_input(s: &str) -> Option<Vec<Directions>> {
    s.trim()
        .lines()
        .map(|line| Direction::parse_directions(line.trim()))
        .collect()
}

type BlackTiles = HashSet<Coords>;
fn flip_tile(tiles: &mut BlackTiles, coords: Coords) {
    if !tiles.remove(&coords) {
        tiles.insert(coords);
    }
}

fn build_tiles<'a>(targets: impl IntoIterator<Item = &'a Directions>) -> BlackTiles {
    let mut tiles = BlackTiles::new();
    for target in targets {
        let coords = coords_from_directions(target.iter().copied());
        flip_tile(&mut tiles, coords);
    }
    tiles
}

fn first_part<'a>(targets: impl IntoIterator<Item = &'a Directions>) -> usize {
    build_tiles(targets).len()
}

fn simulate_day(tiles: &BlackTiles) -> BlackTiles {
    let mut to_check = tiles.clone();
    // collect all the coordinates which have any chance of flipping
    for &coords in tiles {
        to_check.extend(coords_iter_neighbours(coords));
    }

    let mut next_tiles = tiles.clone();
    for coords in to_check {
        let black_neighbors = coords_iter_neighbours(coords)
            .filter(|coords| tiles.contains(coords))
            .count();
        if tiles.contains(&coords) {
            if black_neighbors == 0 || black_neighbors > 2 {
                next_tiles.remove(&coords);
            }
        } else if black_neighbors == 2 {
            next_tiles.insert(coords);
        }
    }

    next_tiles
}

fn second_part<'a>(targets: impl IntoIterator<Item = &'a Directions>) -> usize {
    let mut tiles = build_tiles(targets);
    for _ in 0..100 {
        tiles = simulate_day(&tiles);
    }

    tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    "#;

    #[test]
    fn directions_work() {
        let directions = Direction::parse_directions("nwwswee").expect("failed to parse input");
        assert_eq!(
            directions,
            vec![
                Direction::NW,
                Direction::W,
                Direction::SW,
                Direction::E,
                Direction::E,
            ]
        );
        assert_eq!(coords_from_directions(directions), (0, 0, 0));
    }

    #[test]
    fn first() {
        let targets = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(first_part(&targets), 10);
    }

    #[cfg(feature = "tests-slow")]
    #[test]
    fn second() {
        let targets = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(second_part(&targets), 2208);
    }
}

use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    iter::FromIterator,
    mem,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}
impl Direction {
    fn is_horizontal(self) -> bool {
        matches!(self, Self::Right | Self::Left)
    }
}

#[derive(Eq, PartialEq)]
struct Edge(Vec<bool>);
impl Edge {
    fn eq_reverse(&self, other: &Self) -> bool {
        self.iter().eq(other.iter().rev())
    }
}
impl FromIterator<bool> for Edge {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl Deref for Edge {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Edge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
struct Alignment {
    location: Direction,
    rotations_clockwise: u8,
    flip_x: bool,
    flip_y: bool,
}

struct Edges {
    top_lr: Edge,
    right_tb: Edge,
    bottom_rl: Edge,
    left_bt: Edge,
}
impl Edges {
    fn iter_edges_clockwise<'a>(&'a self) -> impl Iterator<Item = &'a Edge> {
        vec![&self.top_lr, &self.right_tb, &self.bottom_rl, &self.left_bt].into_iter()
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn count_roations_clockwise(my_dir: usize, other_dir: usize) -> u8 {
        (my_dir as isize - other_dir as isize + 2).rem_euclid(4) as u8
    }

    fn align(&self, other: &Self) -> Option<Alignment> {
        for (my_dir, my_edge) in self.iter_edges_clockwise().enumerate() {
            let location = match my_dir {
                0 => Direction::Top,
                1 => Direction::Right,
                2 => Direction::Bottom,
                3 => Direction::Left,
                _ => unreachable!(),
            };
            for (other_dir, other_edge) in other.iter_edges_clockwise().enumerate() {
                let rotations_clockwise = Self::count_roations_clockwise(my_dir, other_dir);
                if my_edge == other_edge {
                    // since both edges are clockwise, if they are in equal order we have to flip.
                    let (mut flip_x, mut flip_y) = (false, false);
                    if location.is_horizontal() {
                        flip_y = true;
                    } else {
                        flip_x = true;
                    }
                    return Some(Alignment {
                        location,
                        rotations_clockwise,
                        flip_x,
                        flip_y,
                    });
                } else if my_edge.eq_reverse(other_edge) {
                    return Some(Alignment {
                        location,
                        rotations_clockwise,
                        flip_x: false,
                        flip_y: false,
                    });
                }
            }
        }
        None
    }
}

struct Tile {
    id: usize,
    width: usize,
    height: usize,
    data: HashSet<(usize, usize)>,
}
impl Tile {
    fn parse_input_from_lines<'a>(mut lines: impl Iterator<Item = &'a str>) -> Option<Self> {
        let id = {
            lines
                .next()?
                .strip_prefix("Tile ")?
                .strip_suffix(':')?
                .parse()
                .ok()?
        };
        let (mut width, mut height) = (0, 0);
        let mut data = HashSet::new();
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                width = width.max(x + 1);
                if c == '#' {
                    data.insert((x, y));
                }
            }
            height = y + 1;
        }

        if width > 0 && height > 0 {
            Some(Self {
                id,
                width,
                height,
                data,
            })
        } else {
            None
        }
    }

    fn parse_input(s: &str) -> Option<Self> {
        let lines = s.trim().lines().map(str::trim);
        Self::parse_input_from_lines(lines)
    }

    fn iter_rows_tb<'a>(
        &'a self,
        x: usize,
    ) -> impl Iterator<Item = bool> + DoubleEndedIterator + 'a {
        (0..self.height).map(move |y| self.data.get(&(x, y)).is_some())
    }

    fn iter_cols_lr<'a>(
        &'a self,
        y: usize,
    ) -> impl Iterator<Item = bool> + DoubleEndedIterator + 'a {
        (0..self.width).map(move |x| self.data.get(&(x, y)).is_some())
    }

    fn edges(&self) -> Edges {
        Edges {
            top_lr: self.iter_cols_lr(0).collect(),
            right_tb: self.iter_rows_tb(self.width - 1).collect(),
            bottom_rl: self.iter_cols_lr(self.height - 1).rev().collect(),
            left_bt: self.iter_rows_tb(0).rev().collect(),
        }
    }

    fn rotate_once_clockwise(&mut self) {
        let last_y_index = self.height - 1;
        for (x, y) in mem::take(&mut self.data) {
            self.data.insert((last_y_index - y, x));
        }
        mem::swap(&mut self.height, &mut self.width);
    }

    fn rotate_clockwise(&mut self, n: u8) {
        for _ in 0..(n % 4) {
            self.rotate_once_clockwise();
        }
    }

    fn flip_x(&mut self) {
        let last_index = self.width - 1;
        for (x, y) in mem::take(&mut self.data) {
            self.data.insert((last_index - x, y));
        }
    }

    fn flip_y(&mut self) {
        let last_index = self.height - 1;
        for (x, y) in mem::take(&mut self.data) {
            self.data.insert((x, last_index - y));
        }
    }

    fn is_within_borders(&self, x: usize, y: usize) -> bool {
        (1..self.width - 1).contains(&x) && (1..self.height - 1).contains(&y)
    }

    fn get_inside_borders(&self, mut x: usize, mut y: usize) -> Option<bool> {
        x += 1;
        y += 1;
        if self.is_within_borders(x, y) {
            Some(self.data.contains(&(x, y)))
        } else {
            None
        }
    }
}

fn parse_input(s: &str) -> Option<Vec<Tile>> {
    let mut tiles = Vec::new();
    let mut temp = Vec::new();
    for line in s.trim().lines().map(str::trim) {
        if line.is_empty() {
            tiles.push(Tile::parse_input_from_lines(temp.drain(..))?);
            continue;
        }

        temp.push(line);
    }

    tiles.push(Tile::parse_input_from_lines(temp.drain(..))?);

    Some(tiles)
}

///  +01xx4567xx0123xx6789+
/// 0|                  # |
/// 1|#    ##    ##    ###|
/// 2| #  #  #  #  #  #   |
///  +--------------------+
const SEA_MONSTER: &[(usize, usize)] = &[
    // 1
    (0, 1),
    (1, 2),
    // 2
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2),
    // 3
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2),
    // 4
    (16, 2),
    (17, 1),
    (18, 0),
    (18, 1),
    (19, 1),
];

const SEA_MONSTER_WIDTH: usize = 20;
const SEA_MONSTER_HEIGHT: usize = 3;

struct AlignedImage {
    min: (isize, isize),
    max: (isize, isize),
    tiles: HashMap<(isize, isize), Tile>,
    tile_width: usize,
    tile_height: usize,
}
impl AlignedImage {
    fn from_grid(grid: HashMap<(isize, isize), Tile>) -> Option<Self> {
        // also checks whether grid empty
        let tile = grid.values().next()?;
        let tile_width = tile.width.checked_sub(2)?;
        let tile_height = tile.height.checked_sub(2)?;

        let x_indices = grid.keys().map(|&(x, _)| x);
        let (min_x, max_x) = (x_indices.clone().min().unwrap(), x_indices.max().unwrap());
        let y_indices = grid.keys().map(|&(_, y)| y);
        let (min_y, max_y) = (y_indices.clone().min().unwrap(), y_indices.max().unwrap());

        Some(Self {
            min: (min_x, min_y),
            max: (max_x, max_y),
            tiles: grid,
            tile_width,
            tile_height,
        })
    }

    fn from_tiles(tiles: impl IntoIterator<Item = Tile>) -> Option<Self> {
        let mut backlog = tiles.into_iter().collect::<VecDeque<_>>();
        let mut grid = HashMap::new();
        grid.insert((0, 0), backlog.pop_front()?);

        while !backlog.is_empty() {
            let prev_length = backlog.len();

            for mut tile in mem::take(&mut backlog) {
                let edges = tile.edges();
                let res = grid
                    .iter()
                    .flat_map(|(&(x, y), grid_tile)| {
                        grid_tile.edges().align(&edges).map(|alig| ((x, y), alig))
                    })
                    .next();

                let ((x, y), alig) = if let Some(v) = res {
                    v
                } else {
                    backlog.push_back(tile);
                    continue;
                };

                tile.rotate_clockwise(alig.rotations_clockwise);
                if alig.flip_x {
                    tile.flip_x();
                }
                if alig.flip_y {
                    tile.flip_y();
                }

                match alig.location {
                    Direction::Top => {
                        grid.insert((x, y - 1), tile);
                    }
                    Direction::Right => {
                        grid.insert((x + 1, y), tile);
                    }
                    Direction::Bottom => {
                        grid.insert((x, y + 1), tile);
                    }
                    Direction::Left => {
                        grid.insert((x - 1, y), tile);
                    }
                }
            }

            // made no progress, abort
            if backlog.len() == prev_length {
                return None;
            }
        }

        Self::from_grid(grid)
    }

    fn horizontal_tiles(&self) -> usize {
        (self.max.0 - self.min.0 + 1).try_into().unwrap_or_default()
    }

    fn width(&self) -> usize {
        self.horizontal_tiles() * self.tile_width
    }

    fn vertical_tiles(&self) -> usize {
        (self.max.1 - self.min.1 + 1).try_into().unwrap_or_default()
    }

    fn height(&self) -> usize {
        self.vertical_tiles() * self.tile_height
    }

    fn corners(&self) -> Option<[&Tile; 4]> {
        let (min_x, min_y) = self.min;
        let (max_x, max_y) = self.max;

        let tl = self.tiles.get(&(min_x, min_y))?;
        let tr = self.tiles.get(&(max_x, min_y))?;
        let bl = self.tiles.get(&(min_x, max_y))?;
        let br = self.tiles.get(&(max_x, max_y))?;

        Some([tl, tr, br, bl])
    }

    fn rotate_once_clockwise(&mut self) {
        let (min_x, min_y) = self.min;
        let (max_x, max_y) = self.max;
        for ((x_off, y_off), mut tile) in mem::take(&mut self.tiles) {
            tile.rotate_once_clockwise();
            let x = x_off - min_x;
            self.tiles.insert((max_y - y_off, x), tile);
        }
        std::mem::swap(&mut self.tile_width, &mut self.tile_height);
        self.min = (0, 0);
        self.max = (max_x - min_x + 1, max_y - min_y + 1);
    }

    fn flip_x(&mut self) {
        let last_index = self.max.1;
        for ((x, y), mut tile) in mem::take(&mut self.tiles) {
            tile.flip_x();
            self.tiles.insert((last_index - x, y), tile);
        }
    }

    fn flip_y(&mut self) {
        let last_index = self.max.1;
        for ((x, y), mut tile) in mem::take(&mut self.tiles) {
            tile.flip_y();
            self.tiles.insert((x, last_index - y), tile);
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        let tile_x = (x / self.tile_width) as isize;
        let tile_y = (y / self.tile_height) as isize;
        let (min_x, min_y) = self.min;
        let tile = self.tiles.get(&(tile_x + min_x, tile_y + min_y))?;

        tile.get_inside_borders(x % self.tile_width, y % self.tile_height)
    }

    fn has_sea_monster_at(&self, start_x: usize, start_y: usize) -> bool {
        SEA_MONSTER.iter().all(|&(off_x, off_y)| {
            self.get_pixel(start_x + off_x, start_y + off_y)
                .unwrap_or_default()
        })
    }

    fn count_sea_monsters(&self) -> usize {
        let (width, height) = (self.width(), self.height());
        if SEA_MONSTER_WIDTH > width || SEA_MONSTER_HEIGHT > height {
            return 0;
        }

        let last_index_x = width - SEA_MONSTER_WIDTH;
        let last_index_y = height - SEA_MONSTER_HEIGHT;

        let mut counter = 0;
        for start_x in 0..=last_index_x {
            for start_y in 0..=last_index_y {
                if self.has_sea_monster_at(start_x, start_y) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn count_sea_monsters_with_rotation(&mut self) -> Option<usize> {
        for _ in 0..3 {
            let count = self.count_sea_monsters();
            if count > 0 {
                return Some(count);
            }
            self.rotate_once_clockwise();
        }
        None
    }

    fn total_black_pixels(&self) -> usize {
        (0..self.width())
            .flat_map(|x| (0..self.height()).map(move |y| (x, y)))
            .filter(|&(x, y)| self.get_pixel(x, y).unwrap_or_default())
            .count()
    }

    fn count_black_pixels_without_sea_monsters_any_rotation(&mut self) -> Option<usize> {
        let monsters = self.count_sea_monsters_with_rotation()?;
        Some(self.total_black_pixels() - monsters * SEA_MONSTER.len())
    }
}

fn find_corners<'a>(tiles: &'a [Tile]) -> Vec<&'a Tile> {
    let mut corners = Vec::new();
    for tile_a in tiles {
        let edges_a = tile_a.edges();
        let mut dir_count = 0;
        for tile_b in tiles {
            if tile_a.id == tile_b.id {
                continue;
            }

            if tile_b.edges().align(&edges_a).is_some() {
                dir_count += 1;
            }
        }

        if dir_count == 2 {
            corners.push(tile_a)
        }
    }

    corners
}

fn first_part(image: &AlignedImage) -> Option<usize> {
    Some(image.corners()?.iter().map(|tile| tile.id).product())
}

fn second_part(image: &mut AlignedImage) -> Option<usize> {
    if let Some(c) = image.count_black_pixels_without_sea_monsters_any_rotation() {
        return Some(c);
    }
    image.flip_x();
    if let Some(c) = image.count_black_pixels_without_sea_monsters_any_rotation() {
        return Some(c);
    }
    // flip back and try horizontally flipped
    image.flip_x();
    image.flip_y();
    if let Some(c) = image.count_black_pixels_without_sea_monsters_any_rotation() {
        return Some(c);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###

        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..

        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...

        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.

        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..

        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.

        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#

        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.

        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
    "#;

    #[test]
    fn first() {
        let tiles = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        let aligned = AlignedImage::from_tiles(tiles).expect("failed to align");

        assert_eq!(
            first_part(&aligned).expect("failed to solve"),
            20_899_048_083_289
        );
    }
    #[test]
    fn second() {
        let tiles = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        let mut aligned = AlignedImage::from_tiles(tiles).expect("failed to align");
        assert_eq!(second_part(&mut aligned), Some(273))
    }
}

use std::collections::HashSet;

struct InfiniteGrid {
    trees: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}
impl InfiniteGrid {
    const TREE_CHAR: char = '#';

    fn from_input(inp: &str) -> Self {
        let mut width = 0;
        let mut height = 0;

        let trees = inp
            .split_whitespace()
            .enumerate()
            .flat_map(|(y, line)| {
                width = width.max(line.len());
                height = y + 1;

                line.chars().enumerate().flat_map(move |(x, c)| {
                    if c == Self::TREE_CHAR {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self {
            trees,
            width,
            height,
        }
    }

    fn has_tree(&self, x: usize, y: usize) -> bool {
        self.trees.contains(&(x % self.width, y))
    }

    fn count_trees(&self, (right, down): (usize, usize)) -> usize {
        let mut count = 0;
        let mut x = 0;
        for y in (0..self.height).step_by(down) {
            if self.has_tree(x, y) {
                count += 1;
            }
            x += right;
        }
        count
    }
}

fn first_part(grid: &InfiniteGrid) -> usize {
    grid.count_trees((3, 1))
}

fn second_part(grid: &InfiniteGrid) -> usize {
    grid.count_trees((1, 1))
        * grid.count_trees((1, 2))
        * grid.count_trees((3, 1))
        * grid.count_trees((5, 1))
        * grid.count_trees((7, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
    "#;

    fn example_input() -> InfiniteGrid {
        InfiniteGrid::from_input(EXAMPLE_INPUT)
    }

    #[test]
    fn first() {
        let sol = first_part(&example_input());
        assert_eq!(sol, 7);
    }
    #[test]
    fn second() {
        let sol = second_part(&example_input());
        assert_eq!(sol, 336);
    }
}

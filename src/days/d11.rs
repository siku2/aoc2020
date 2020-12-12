use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Seat {
    Occupied,
    Empty,
    Floor,
}
impl Seat {
    fn parse_input(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Empty),
            '#' => Some(Self::Occupied),
            '.' => Some(Self::Floor),
            _ => None,
        }
    }
}
impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Occupied => f.write_str("#"),
            Self::Empty => f.write_str("L"),
            Self::Floor => f.write_str("."),
        }
    }
}

struct RaycastHit {
    seat: Seat,
    steps: usize,
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Layout {
    seats: Vec<Seat>,
    rows: usize,
    cols: usize,
}
impl Layout {
    fn parse_input(s: &str) -> Option<Self> {
        let lines = s.split_whitespace().collect::<Vec<_>>();
        let rows = lines.len();
        let seats = lines
            .into_iter()
            .flat_map(|line| line.chars().map(Seat::parse_input))
            .collect::<Option<Vec<_>>>()?;
        let cols = seats.len() / rows;

        Some(Self { seats, rows, cols })
    }

    fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if col >= self.cols {
            return None;
        }

        Some(row * self.cols + col)
    }

    fn get(&self, row: usize, col: usize) -> Option<Seat> {
        self.get_index(row, col)
            .and_then(|i| self.seats.get(i))
            .copied()
    }

    fn set(&mut self, row: usize, col: usize, seat: Seat) {
        if let Some(slot) = self.get_index(row, col).and_then(|i| self.seats.get_mut(i)) {
            *slot = seat;
        }
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|seat| matches!(seat, Seat::Occupied))
            .count()
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn raycast_steps<'a>(
        &'a self,
        row: usize,
        col: usize,
        dir_row: isize,
        dir_col: isize,
    ) -> impl Iterator<Item = RaycastHit> + 'a {
        let (irows, icols) = (self.rows as isize, self.cols as isize);
        let mut row = row as isize;
        let mut col = col as isize;
        let mut steps = 0;

        std::iter::from_fn(move || {
            if !(0..irows).contains(&row) || !(0..icols).contains(&col) {
                return None;
            }
            let (unsigned_row, unsigned_col) = (row as usize, col as usize);
            let seat = self.get(unsigned_row, unsigned_col)?;
            let hit = RaycastHit {
                seat,
                steps,
                row: unsigned_row,
                col: unsigned_col,
            };

            row += dir_row;
            col += dir_col;
            steps += 1;

            Some(hit)
        })
        .fuse()
    }

    fn raycast(
        &self,
        row: usize,
        col: usize,
        dir_row: isize,
        dir_col: isize,
    ) -> Option<RaycastHit> {
        self.raycast_steps(row, col, dir_row, dir_col)
            .skip(1)
            .take_while(|hit| matches!(hit.seat, Seat::Floor | Seat::Occupied))
            .find(|hit| matches!(hit.seat, Seat::Occupied))
    }

    fn adjacent_hits<'a>(
        &'a self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = RaycastHit> + 'a {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .flat_map(move |(dir_row, dir_col)| self.raycast(row, col, *dir_row, *dir_col))
    }

    fn adjacents<'a>(
        &'a self,
        row: usize,
        col: usize,
        max_distance: Option<usize>,
    ) -> impl Iterator<Item = Seat> + 'a {
        self.adjacent_hits(row, col).filter_map(move |hit| {
            if let Some(max_distance) = max_distance {
                if hit.steps > max_distance {
                    return None;
                }
            }

            Some(hit.seat)
        })
    }

    fn step_raw(&self, max_occupied: usize, max_distance: Option<usize>) -> Self {
        let mut next = self.clone();
        for r in 0..self.rows {
            for c in 0..self.cols {
                let seat = self.get(r, c).unwrap();
                let adjacents = self.adjacents(r, c, max_distance).count();

                match (seat, adjacents) {
                    (Seat::Empty, 0) => next.set(r, c, Seat::Occupied),
                    (Seat::Occupied, n) if n >= max_occupied => next.set(r, c, Seat::Empty),
                    _ => {}
                }
            }
        }
        next
    }

    fn step1(&self) -> Self {
        self.step_raw(4, Some(1))
    }
    fn step2(&self) -> Self {
        self.step_raw(5, None)
    }
}
impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.get(row, col).unwrap().fmt(f)?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn first_part(mut layout: Layout) -> usize {
    loop {
        let prev_layout = layout;
        layout = prev_layout.step1();
        if layout == prev_layout {
            return layout.count_occupied();
        }
    }
}

fn second_part(mut layout: Layout) -> usize {
    loop {
        let prev_layout = layout;
        layout = prev_layout.step2();
        if layout == prev_layout {
            return layout.count_occupied();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    "#;

    #[test]
    fn first_first_step() {
        let mut layout = Layout::parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        layout = layout.step1();
        assert_eq!(
            layout,
            Layout::parse_input(
                r#"
                #.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##
                "#
            )
            .expect("failed to parse reference layout")
        );

        layout = layout.step1();
        assert_eq!(
            layout,
            Layout::parse_input(
                r#"
                #.LL.L#.##
                #LLLLLL.L#
                L.L.L..L..
                #LLL.LL.L#
                #.LL.LL.LL
                #.LLLL#.##
                ..L.L.....
                #LLLLLLLL#
                #.LLLLLL.L
                #.#LLLL.##
                "#
            )
            .expect("failed to parse reference layout")
        );
    }

    #[test]
    fn first() {
        let sol = first_part(Layout::parse_input(EXAMPLE_INPUT).expect("failed to parse input"));
        assert_eq!(sol, 37);
    }

    #[test]
    fn second_first_steps() {
        let mut layout = Layout::parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        // first step
        layout = layout.step2();
        assert_eq!(
            layout,
            Layout::parse_input(
                r#"
                #.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##
                "#
            )
            .expect("failed to parse reference layout")
        );

        // second step
        layout = layout.step2();
        assert_eq!(
            layout,
            Layout::parse_input(
                r#"
                #.LL.LL.L#
                #LLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLL#
                #.LLLLLL.L
                #.LLLLL.L#
                "#
            )
            .expect("failed to parse reference layout")
        );
    }

    #[test]
    fn second() {
        let sol = second_part(Layout::parse_input(EXAMPLE_INPUT).expect("failed to parse input"));
        assert_eq!(sol, 26);
    }
}

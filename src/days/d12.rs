use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    north: isize,
    east: isize,
}
impl Position {
    #[allow(clippy::cast_sign_loss)]
    fn manhatten_length(self) -> usize {
        (self.north.abs() + self.east.abs()) as usize
    }

    #[allow(clippy::cast_possible_wrap)]
    fn sincos90(n: isize) -> (isize, isize) {
        const SIN90: [isize; 4] = [0, 1, 0, -1];
        const SIN90_N: isize = SIN90.len() as isize;

        (
            unsafe { *SIN90.get_unchecked(n.rem_euclid(SIN90_N) as usize) },
            unsafe { *SIN90.get_unchecked((n + 1).rem_euclid(SIN90_N) as usize) },
        )
    }

    fn rotate_clockwise(self, n: isize) -> Self {
        let (sin, cos) = Self::sincos90(n);
        Self {
            north: self.north * cos - self.east * sin,
            east: self.north * sin + self.east * cos,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            north: self.north + rhs.north,
            east: self.east + rhs.east,
        }
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            north: self.north - rhs.north,
            east: self.east - rhs.east,
        }
    }
}

impl Mul<isize> for Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            north: rhs * self.north,
            east: rhs * self.east,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Heading {
    North,
    South,
    East,
    West,
}
impl Heading {
    const CLOCKWISE: [Self; 4] = [Self::East, Self::South, Self::West, Self::North];

    fn from_input(c: char) -> Option<Self> {
        match c {
            'N' => Some(Self::North),
            'S' => Some(Self::South),
            'E' => Some(Self::East),
            'W' => Some(Self::West),
            _ => None,
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn rotate_clockwise(self, n: isize) -> Self {
        let mut index = Self::CLOCKWISE
            .iter()
            .position(|&head| head == self)
            .unwrap() as isize;
        index += n;

        unsafe {
            *Self::CLOCKWISE
                .get_unchecked(index.rem_euclid(Self::CLOCKWISE.len() as isize) as usize)
        }
    }

    fn dir_pos(self) -> Position {
        match self {
            Self::North => Position { north: 1, east: 0 },
            Self::South => Position { north: -1, east: 0 },
            Self::East => Position { north: 0, east: 1 },
            Self::West => Position { north: 0, east: -1 },
        }
    }
}

#[derive(Debug)]
struct Ship {
    facing: Heading,
    pos: Position,
    waypoint_offset: Position,
}
impl Ship {
    fn apply(&mut self, instr: Instruction) {
        match instr {
            Instruction::Head(facing, amount) => {
                self.pos += facing.dir_pos() * amount;
            }
            Instruction::Left(n) => self.facing = self.facing.rotate_clockwise(-n),
            Instruction::Right(n) => self.facing = self.facing.rotate_clockwise(n),
            Instruction::Forward(steps) => {
                self.pos += self.facing.dir_pos() * steps;
            }
        }
    }

    fn apply_with_waypoint(&mut self, instr: Instruction) {
        match instr {
            Instruction::Head(facing, amount) => {
                self.waypoint_offset += facing.dir_pos() * amount;
            }
            Instruction::Left(n) => {
                self.waypoint_offset = self.waypoint_offset.rotate_clockwise(-n)
            }
            Instruction::Right(n) => {
                self.waypoint_offset = self.waypoint_offset.rotate_clockwise(n)
            }
            Instruction::Forward(steps) => {
                self.pos += self.waypoint_offset * steps;
            }
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            facing: Heading::East,
            pos: Position::default(),
            waypoint_offset: Position { north: 1, east: 10 },
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Head(Heading, isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}
impl Instruction {
    fn from_input(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let i = chars.next()?;
        let n = chars.collect::<String>().parse().ok()?;

        let instr = match i {
            'L' => Self::Left(n / 90),
            'R' => Self::Right(n / 90),
            'F' => Self::Forward(n),
            c => Self::Head(Heading::from_input(c)?, n),
        };
        Some(instr)
    }
}

fn parse_input(s: &str) -> Option<Vec<Instruction>> {
    s.split_whitespace().map(Instruction::from_input).collect()
}

fn first_part(instructions: &[Instruction]) -> usize {
    let mut ship = Ship::default();
    for &instr in instructions {
        ship.apply(instr);
    }

    ship.pos.manhatten_length()
}

fn second_part(instructions: &[Instruction]) -> usize {
    let mut ship = Ship::default();
    for &instr in instructions {
        ship.apply_with_waypoint(instr);
    }

    ship.pos.manhatten_length()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        F10
        N3
        F7
        R90
        F11
    "#;

    #[test]
    fn first() {
        let sol = first_part(&parse_input(EXAMPLE_INPUT).expect("failed to parse input"));
        assert_eq!(sol, 25);
    }

    #[test]
    fn second() {
        let sol = second_part(&parse_input(EXAMPLE_INPUT).expect("failed to parse input"));
        assert_eq!(sol, 286);
    }
}

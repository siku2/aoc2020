type SeatId = u16;

const ROW_BITS: SeatId = 7;
const TOTAL_ROWS: SeatId = 1 << ROW_BITS;

const COL_BITS: SeatId = 3;
const TOTAL_COLS: SeatId = 1 << COL_BITS;

fn parse_seat_id(s: &str) -> SeatId {
    s.chars()
        .map(|c| matches!(c, 'B' | 'R'))
        .fold(0, |i, b| (i << 1) + b as SeatId)
}

fn get_seat_pos(id: SeatId) -> (u8, u8) {
    ((id >> COL_BITS) as u8, (id & (TOTAL_COLS - 1)) as u8)
}

fn parse_input(s: &str) -> Vec<SeatId> {
    s.split_whitespace().map(parse_seat_id).collect()
}

fn first_part(seats: impl IntoIterator<Item = SeatId>) -> Option<SeatId> {
    seats.into_iter().max()
}

fn second_part(mut seats: Vec<SeatId>) -> Option<SeatId> {
    seats.sort_unstable();
    seats.windows(2).find_map(|win| {
        let (prev, next) = (win[0], win[1]);
        if next - prev > 1 {
            Some(prev + 1)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_index() {
        assert_eq!(get_seat_pos(parse_seat_id("BFFFBBFRRR")), (70, 7));
        assert_eq!(get_seat_pos(parse_seat_id("FFFBBBFRRR")), (14, 7));
        assert_eq!(get_seat_pos(parse_seat_id("BBFFBBFRLL")), (102, 4));
    }

    const EXAMPLE_INPUT: &str = r#"
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL
    "#;

    #[test]
    fn first() {
        assert_eq!(first_part(parse_input(EXAMPLE_INPUT)), Some(820));
    }

    #[test]
    fn second() {
        assert_eq!(second_part(parse_input(EXAMPLE_INPUT)), Some(120));
    }
}

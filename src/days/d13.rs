#[derive(Clone, Copy, Debug)]
enum BusTime {
    None,
    Timestamp(usize),
}
impl BusTime {
    fn parse_input(s: &str) -> Option<Self> {
        if s == "x" {
            Some(Self::None)
        } else {
            s.parse().ok().map(Self::Timestamp)
        }
    }

    fn get_id(self) -> Option<usize> {
        self.as_time()
    }

    fn as_time(self) -> Option<usize> {
        match self {
            Self::None => None,
            Self::Timestamp(t) => Some(t),
        }
    }
}

fn ceiling_division(a: usize, b: usize) -> usize {
    a / b + (a % b != 0) as usize
}

struct Timetable(Vec<BusTime>);
impl Timetable {
    fn parse_input(s: &str) -> Option<Self> {
        let times = s
            .split(',')
            .map(BusTime::parse_input)
            .collect::<Option<Vec<_>>>()?;
        Some(Self(times))
    }

    fn find_next_bus(&self, n: usize) -> Option<(BusTime, usize)> {
        self.0
            .iter()
            .filter_map(move |&bus| bus.as_time().map(|t| (bus, t * ceiling_division(n, t))))
            .min_by(|&(_, a), (_, b)| a.cmp(b))
    }

    fn first(&self, n: usize) -> Option<usize> {
        self.find_next_bus(n)
            .map(|(bus, time)| bus.get_id().unwrap_or_default() * (time - n))
    }

    fn second(&self) -> usize {
        let mut time_step = 1;
        let mut matching_time = 0;
        for (offset, time) in self
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, bus)| bus.as_time().map(|time| (i, time)))
        {
            // find the next timestamp which matches the condition for this bus
            let mut c = matching_time;
            matching_time = std::iter::repeat_with(move || {
                let tmp = c;
                c += time_step;
                tmp
            })
            .find(|&c| (c + offset) % time == 0)
            .unwrap();

            // for every `matching_time + k * time_step` the condition will still hold true.
            // no need to use LCM here because times are coprime.
            time_step *= time;
        }
        matching_time
    }
}

fn parse_input(s: &str) -> Option<(Timetable, usize)> {
    let mut it = s.split_whitespace();
    let start_time = it.next()?.parse().ok()?;
    let timetable = it.next().and_then(Timetable::parse_input)?;
    Some((timetable, start_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        939
        7,13,x,x,59,x,31,19
    "#;

    #[test]
    fn ceildiv_works() {
        assert_eq!(ceiling_division(5, 3), 2);
    }

    #[test]
    fn first() {
        let (table, time) = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(table.first(time).expect("failed to solve"), 295);
    }

    #[test]
    fn second() {
        let (table, _) = parse_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(table.second(), 1_068_781);
    }
}

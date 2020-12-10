fn parse_input(s: &str) -> Option<Vec<u16>> {
    s.split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()
}

fn first_part(mut adapters: Vec<u16>) -> u16 {
    adapters.sort_unstable();

    let mut diff1 = 0;
    let mut diff3 = 1;
    let mut prev_joltage = 0;
    for adapter in adapters {
        match adapter - prev_joltage {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => {}
        }
        prev_joltage = adapter;
    }

    diff1 * diff3
}

/// Sequence: 0, 1, 1, 2, 4, 7, 13
fn tribonacci(mut n: u64) -> u64 {
    match n {
        0 => return 0,
        1 | 2 => return 1,
        _ => {
            n -= 2;
        }
    }
    let (mut s0, mut s1, mut s2) = (0, 1, 1);

    for _ in 0..n {
        let tmp = s0 + s1 + s2;
        s0 = s1;
        s1 = s2;
        s2 = tmp;
    }

    s2
}

fn second_part(mut adapters: Vec<u16>) -> u64 {
    const MAX_RANGE: u16 = 3;

    adapters.sort_unstable();

    // each entry represents the amount of adaptors that may follow the previous one
    let mut sequences = vec![1];

    let mut previous = 0;
    for adapter in adapters {
        if adapter - previous >= MAX_RANGE {
            // adapter no longer compatible; start new sequence
            sequences.push(0);
        }

        *sequences.last_mut().unwrap() += 1;
        previous = adapter;
    }

    sequences.iter().copied().map(tribonacci).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3
    "#;

    #[test]
    fn first() {
        let sol = first_part(parse_input(EXAMPLE_INPUT).expect("failed to parse input"));
        assert_eq!(sol, 220);
    }
    #[test]
    fn tribonacci_works() {
        assert_eq!(tribonacci(0), 0);
        assert_eq!(tribonacci(1), 1);
        assert_eq!(tribonacci(5), 7);
    }

    #[test]
    fn second() {
        let sol = second_part(parse_input(EXAMPLE_INPUT).expect("failed to parse input"));
        assert_eq!(sol, 19208);
    }
}

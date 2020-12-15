use std::collections::HashMap;

type Memory = HashMap<u64, u64>;

/// Distribute the bits from `n` over the `1` in `mask`
fn my_pdep(n: u64, mut mask: u64) -> u64 {
    let mut result = 0;
    let mut i = 0;
    while mask != 0 {
        let l = mask & (0_u64.overflowing_sub(mask).0); // extract mask's least significant 1-bit
        mask ^= l; // clear mask's least significant 1-bit
        let (s, _) = 0_u64.overflowing_sub(n & (1 << i)); // spread i-th bit of `n` to more signif. bits
        result |= l & s; // deposit i-th bit of `n` at position of mask's 1-bit
        i += 1;
    }

    result
}

#[derive(Clone, Copy, Default)]
struct Mask {
    /// bitmask: `1` if mask has a `1` at the position
    high: u64,
    /// bitmask: `1` if mask has a `0` at the position
    low: u64,
    /// bitmask: `1` if mask has a `X` at the position
    x: u64,
}
impl Mask {
    const U36_MAX: u64 = (1 << 36) - 1;

    fn from_input(s: &str) -> Option<Self> {
        let (mut high, mut low, mut x) = (0, 0, 0);
        for c in s.chars() {
            high <<= 1;
            low <<= 1;
            x <<= 1;

            match c {
                '1' => {
                    high += 1;
                }
                '0' => {
                    low += 1;
                }
                'X' => {
                    x += 1;
                }
                _ => return None,
            }
        }

        Some(Self { high, low, x })
    }

    fn apply_high(self, n: u64) -> u64 {
        self.high | n
    }

    fn apply_low(self, n: u64) -> u64 {
        n & !self.low
    }

    fn apply(self, n: u64) -> u64 {
        self.apply_low(self.apply_high(n))
    }

    fn iter_floating_permutations(self) -> impl Iterator<Item = u64> {
        let x = self.x;
        let count = 1 << x.count_ones();
        (0..count).map(move |i| my_pdep(i, x))
    }

    fn iter_floating_addresses(self, mut n: u64) -> impl Iterator<Item = u64> {
        n = self.apply_high(n);
        n &= !self.x & Self::U36_MAX;
        self.iter_floating_permutations().map(move |c| n | c)
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    SetMask(Mask),
    Write(u64, u64),
}
impl Instruction {
    fn from_input(s: &str) -> Option<Self> {
        let mut it = s.split('=');
        let instr = it.next()?.trim();
        let value = it.next()?.trim();
        if instr == "mask" {
            Mask::from_input(value).map(Self::SetMask)
        } else {
            let value = value.parse().ok()?;
            let addr = {
                let (start, end) = instr.find('[').zip(instr.find(']'))?;
                instr.get(start + 1..end)?.parse().ok()?
            };

            Some(Self::Write(addr, value))
        }
    }
}

fn parse_input(s: &str) -> Option<Vec<Instruction>> {
    s.trim()
        .lines()
        .map(|line| Instruction::from_input(line.trim()))
        .collect()
}

fn first_part(instrs: impl IntoIterator<Item = Instruction>) -> u64 {
    let mut mask = Mask::default();
    let mut mem = Memory::new();
    for instr in instrs {
        match instr {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::Write(addr, value) => {
                mem.insert(addr, mask.apply(value));
            }
        }
    }

    mem.values().sum()
}

fn second_part(instrs: impl IntoIterator<Item = Instruction>) -> u64 {
    let mut mask = Mask::default();
    let mut mem = Memory::new();
    for instr in instrs {
        match instr {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::Write(addr, value) => {
                for addr in mask.iter_floating_addresses(addr) {
                    mem.insert(addr, value);
                }
            }
        }
    }

    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_pdep_works() {
        assert_eq!(my_pdep(0b0101, 0b1100_1100), 0b0100_0100);
    }

    #[test]
    fn mask_works() {
        let mask =
            Mask::from_input("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").expect("failed to parse mask");
        assert_eq!(mask.apply_high(0b0000_1011), 0b0100_1011);
        assert_eq!(mask.apply_low(0b0000_1011), 0b0000_1001);
        assert_eq!(mask.apply(0b0000_1011), 0b0100_1001);
    }

    #[test]
    fn mask_floating_addresses_works() {
        let mask =
            Mask::from_input("000000000000000000000000000000X1001X").expect("failed to parse mask");
        let mut it = mask.iter_floating_addresses(0b0010_1010);
        assert_eq!(it.next(), Some(0b0001_1010));
        assert_eq!(it.next(), Some(0b0001_1011));
        assert_eq!(it.next(), Some(0b0011_1010));
        assert_eq!(it.next(), Some(0b0011_1011));
    }

    #[test]
    fn mask_floating_permutations_works() {
        let mask =
            Mask::from_input("000000000000000000000000000000X1001X").expect("failed to parse mask");
        let mut it = mask.iter_floating_permutations();
        assert_eq!(it.next(), Some(0b0000_0000));
        assert_eq!(it.next(), Some(0b0000_0001));
        assert_eq!(it.next(), Some(0b0010_0000));
        assert_eq!(it.next(), Some(0b0010_0001));

        let mask =
            Mask::from_input("00000000000000000000000000000000X0XX").expect("failed to parse mask");
        assert_eq!(mask.iter_floating_permutations().count(), 8);
    }

    #[test]
    fn first() {
        const INPUT: &str = r#"
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        "#;

        let instrs = parse_input(INPUT).expect("failed to parse input");
        assert_eq!(first_part(instrs), 165);
    }

    #[test]
    fn second() {
        const INPUT: &str = r#"
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        "#;
        let instrs = parse_input(INPUT).expect("failed to parse input");
        assert_eq!(second_part(instrs), 208);
    }
}

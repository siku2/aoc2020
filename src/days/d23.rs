use std::string::ToString;

struct FakeLinkedList {
    labels: Vec<u32>,
    first: u32,
    last: u32,
}
impl FakeLinkedList {
    unsafe fn from_iterator(it: impl IntoIterator<Item = u32>, max_label: u32) -> Option<Self> {
        let mut it = it.into_iter();
        let first = it.next()?;
        let mut labels = vec![0; max_label as usize + 1];
        labels[first as usize] = first;
        let mut list = Self {
            labels,
            first,
            last: first,
        };
        list.append_unchecked(it);
        Some(list)
    }

    fn from_input(s: &str) -> Option<Self> {
        let digits = s
            .trim()
            .chars()
            .map(|d| d.to_digit(10))
            .collect::<Option<Vec<_>>>()?;
        let max_label = *digits.iter().max()?;
        unsafe { Self::from_iterator(digits, max_label) }
    }

    fn resize(&mut self, max_label: u32) {
        self.labels.resize(max_label as usize + 1, 0);
    }

    unsafe fn append_unchecked(&mut self, it: impl IntoIterator<Item = u32>) {
        let mut prev_label = self.last;
        for label in it {
            self.labels[prev_label as usize] = label;
            prev_label = label;
        }
        self.last = prev_label;
        self.labels[prev_label as usize] = self.first;
    }

    #[allow(clippy::cast_possible_truncation)]
    fn min(&self) -> Option<u32> {
        self.labels
            .iter()
            .enumerate()
            .find_map(|(label, &link)| if link == 0 { None } else { Some(label as u32) })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn max(&self) -> Option<u32> {
        self.labels
            .iter()
            .enumerate()
            .rev()
            .find_map(|(label, &link)| if link == 0 { None } else { Some(label as u32) })
    }

    fn iter_after_label<'a>(&'a self, label: u32) -> impl Iterator<Item = u32> + 'a {
        let mut current_label = label;
        std::iter::from_fn(move || {
            let next_label = self.labels[current_label as usize];
            current_label = next_label;
            Some(next_label)
        })
    }

    fn read_three_after(&self, label: u32) -> [u32; 3] {
        let mut it = self.iter_after_label(label);
        [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
    }

    unsafe fn move_three_unchecked(&mut self, pre: u32, target: u32) {
        let [a, _b, c] = self.read_three_after(pre);
        // current order:
        //  - pre -> a -> b -> c -> <c.next>
        //  - target -> <target.next>
        //
        // `a -> b -> c` should be moved to after target

        // link pre -> <c.next>
        self.labels[pre as usize] = self.labels[c as usize];

        // link c -> <target.next>
        self.labels[c as usize] = self.labels[target as usize];
        // link target -> a
        self.labels[target as usize] = a;
        // a -> b remains true
    }

    fn after(&self, label: u32) -> Option<u32> {
        self.labels.get(label as usize).copied()
    }
}

fn decrement_wrap(n: u32, low: u32, high: u32) -> u32 {
    if n <= low {
        return high;
    }

    n - 1
}

#[must_use]
fn simulate_moves(cups: &mut FakeLinkedList, moves_to_simulate: usize) -> Option<()> {
    let mut current_label = cups.first;
    let min = cups.min()?;
    let max = cups.max()?;
    for _ in 0..moves_to_simulate {
        let pickup = cups.read_three_after(current_label);

        let mut dest_label = decrement_wrap(current_label, min, max);
        while pickup.contains(&dest_label) {
            dest_label = decrement_wrap(dest_label, min, max);
        }

        unsafe { cups.move_three_unchecked(current_label, dest_label) };
        current_label = cups.after(current_label).unwrap();
    }

    Some(())
}

fn first_part(cups: &mut FakeLinkedList) -> Option<String> {
    simulate_moves(cups, 100)?;

    let digits = cups
        .iter_after_label(1)
        .take_while(|&d| d != 1)
        .map(|d| d.to_string())
        .collect();
    Some(digits)
}

fn second_part(cups: &mut FakeLinkedList) -> Option<u64> {
    let max_label = cups.max()?;
    cups.resize(1_000_000);
    unsafe { cups.append_unchecked(max_label + 1..=1_000_000) };

    simulate_moves(cups, 10_000_000)?;

    Some(cups.iter_after_label(1).take(2).map(u64::from).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"389125467"#;

    #[test]
    fn first() {
        let mut cups = FakeLinkedList::from_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(first_part(&mut cups).expect("failed to solve"), "67384529");
    }
    #[cfg(feature = "tests-slow")]
    #[test]
    fn second() {
        let mut cups = FakeLinkedList::from_input(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(
            second_part(&mut cups).expect("failed to solve"),
            149_245_887_792
        );
    }
}

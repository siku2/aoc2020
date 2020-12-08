use std::{collections::HashSet, convert::TryInto};

type Argument = i32;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct State {
    pointer: usize,
    accumulator: Argument,
}

#[derive(Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}
impl Operation {
    fn from_input(s: &str) -> Option<Self> {
        match s {
            "acc" => Some(Self::Acc),
            "jmp" => Some(Self::Jmp),
            "nop" => Some(Self::Nop),
            _ => None,
        }
    }

    fn perform(self, state: &mut State, arg: Argument) -> bool {
        match self {
            Self::Acc => {
                state.accumulator += arg;
                state.pointer += 1;
                true
            }
            Self::Jmp => {
                let new_pointer = state
                    .pointer
                    .try_into()
                    .ok()
                    .map(|p: Argument| p + arg)
                    .and_then(|new_p| new_p.try_into().ok());
                state.pointer = match new_pointer {
                    Some(v) => v,
                    None => return false,
                };
                true
            }
            Self::Nop => {
                state.pointer += 1;
                true
            }
        }
    }

    fn flipped(self) -> Option<Self> {
        match self {
            Self::Acc => None,
            Self::Jmp => Some(Self::Nop),
            Self::Nop => Some(Self::Jmp),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    op: Operation,
    arg: Argument,
}
impl Instruction {
    fn from_input(s: &str) -> Option<Self> {
        let mut it = s.split_whitespace();
        let op = it.next().and_then(Operation::from_input)?;
        let arg = it.next().and_then(|s| s.parse().ok())?;

        Some(Self { op, arg })
    }

    fn execute(&self, state: &mut State) -> bool {
        self.op.perform(state, self.arg)
    }
}

#[derive(Clone)]
struct Machine {
    instructions: Vec<Instruction>,
    state: State,
}
impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            state: State::default(),
        }
    }

    fn from_input(s: &str) -> Option<Self> {
        s.trim()
            .lines()
            .map(|line| Instruction::from_input(line.trim()))
            .collect::<Option<_>>()
            .map(Self::new)
    }

    fn finished(&self) -> bool {
        self.state.pointer == self.instructions.len()
    }

    fn step(&mut self) -> bool {
        match self.instructions.get(self.state.pointer) {
            Some(instr) => instr.execute(&mut self.state),
            None => false,
        }
    }

    fn step_while(&mut self, mut f: impl FnMut(&State) -> bool) -> bool {
        while f(&self.state) {
            if !self.step() {
                return false;
            }
        }
        true
    }

    /// Run the machine until it loops or stops
    ///
    /// Returns `true` if the machine hit an infinite loop and `false` if an error occurred.
    /// Use [`finished`] to check whether the machine finished naturally.
    fn run_until_loop(&mut self) -> bool {
        let mut seen: HashSet<usize> = HashSet::new();
        self.step_while(|state| seen.insert(state.pointer))
    }
}

fn first_part(machine: &mut Machine) -> Option<Argument> {
    if machine.run_until_loop() {
        Some(machine.state.accumulator)
    } else {
        None
    }
}

fn second_part(original_machine: &Machine) -> Option<Argument> {
    let mut machines = Vec::new();
    for (i, instr) in original_machine.instructions.iter().enumerate() {
        if let Some(flipped) = instr.op.flipped() {
            let mut machine = original_machine.clone();
            // since this is a clone we know for sure that this isn't going to panic
            machine.instructions[i].op = flipped;
            machines.push(machine);
        }
    }

    for mut machine in machines {
        if !machine.run_until_loop() && machine.finished() {
            return Some(machine.state.accumulator);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
    "#;

    #[test]
    fn first() {
        assert_eq!(
            first_part(&mut Machine::from_input(EXAMPLE_INPUT).expect("failed to parse input")),
            Some(5)
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            second_part(&Machine::from_input(EXAMPLE_INPUT).expect("failed to parse input")),
            Some(8)
        );
    }
}

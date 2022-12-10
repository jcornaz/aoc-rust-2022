use std::{iter, str::FromStr};

type Output = i32;

pub fn part_1(input: &str) -> Output {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();
    let mut device = Device::new(1, instructions);
    let mut last = 1;
    let mut result = 0;
    for cycle in iter::successors(Some(20), |c| Some(*c + 40)).take_while(|c| *c <= 220) {
        device.advance(cycle - last);
        println!("{} {}", cycle, device.register_value());
        result += cycle as i32 * device.register_value();
        last = cycle;
    }
    result
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

struct Device {
    register: i32,
    queue: Vec<Instruction>,
    cycles: u32,
}

impl Device {
    fn new(initial_reg_value: i32, mut instructions: Vec<Instruction>) -> Self {
        instructions.reverse();
        Self {
            register: initial_reg_value,
            queue: instructions,
            cycles: 0,
        }
    }

    fn advance(&mut self, cycles: u32) {
        self.cycles += cycles;
        while self.cycles > 0 {
            match self.queue.last() {
                Some(Instruction::NoOp) if self.cycles >= 1 => {
                    self.cycles -= 1;
                    let _ = self.queue.pop();
                }
                Some(Instruction::AddX(v)) if self.cycles >= 2 => {
                    self.cycles -= 2;
                    self.register += v;
                    let _ = self.queue.pop();
                }
                _ => return,
            }
        }
    }

    fn register_value(&self) -> i32 {
        self.register
    }
}

enum Instruction {
    NoOp,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::NoOp)
        } else if let Some(v) = s.strip_prefix("addx ") {
            v.parse::<i32>()
                .map_err(|e| e.to_string())
                .map(Instruction::AddX)
        } else {
            Err(format!("Unknown instruction: {s}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction::*;
    use super::*;

    const EXAMPLE: &str = include_str!("day10/example.txt");
    const INPUT: &str = include_str!("day10/input.txt");

    #[rstest]
    #[case::example(EXAMPLE, 13140)]
    #[case::input(INPUT, 14160)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 0)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }

    #[rstest]
    #[case(1, vec![], 0, 1)]
    #[case(1, vec![AddX(3)], 2, 4)]
    #[case(1, vec![NoOp, NoOp], 2, 1)]
    #[case(1, vec![AddX(3)], 1, 1)]
    #[case(1, vec![NoOp, AddX(3)], 1, 1)]
    #[case(1, vec![NoOp, AddX(3)], 2, 1)]
    #[case(1, vec![NoOp, AddX(3)], 3, 4)]
    #[case(1, vec![NoOp, AddX(3), AddX(1)], 3, 4)]
    #[case(1, vec![NoOp, AddX(3), AddX(1)], 4, 4)]
    #[case(1, vec![NoOp, AddX(3), AddX(1)], 5, 5)]
    fn should_advance_simulation_by_the_given_cycles(
        #[case] initial_register_value: i32,
        #[case] instructions: Vec<Instruction>,
        #[case] cycles: u32,
        #[case] exepcted_register_value: i32,
    ) {
        let mut device = Device::new(initial_register_value, instructions);
        device.advance(cycles);
        assert_eq!(device.register_value(), exepcted_register_value);
    }
}

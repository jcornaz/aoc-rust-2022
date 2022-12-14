use std::{collections::HashMap, convert::Infallible, fmt::Debug, str::FromStr};

type Output = usize;
type WorryLevel = u64;

pub fn part_1(input: &str) -> Output {
    solve(input, 3, 20)
}

pub fn part_2(input: &str) -> Output {
    solve(input, 1, 10_000)
}

fn solve(input: &str, worry_level_divisor: WorryLevel, rounds: u32) -> Output {
    let mut monkeys = parse_monkeys(input);
    let mut ids: Vec<_> = monkeys.keys().copied().collect();
    ids.sort();
    let max_worry_level: WorryLevel = monkeys.values().map(|m| m.test_divisor).product();
    for _ in 0..rounds {
        for id in ids.iter().copied() {
            let mut monkey = monkeys.remove(&id).unwrap();
            for (target, item) in monkey.throw_all(worry_level_divisor) {
                monkeys
                    .get_mut(&target)
                    .unwrap()
                    .catch(item % max_worry_level);
            }
            monkeys.insert(id, monkey);
        }
    }
    let mut inspections: Vec<_> = monkeys.values().map(|m| m.inspected).collect();
    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

fn parse_monkeys(input: &str) -> HashMap<usize, Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let (id, declaration) = monkey.split_once('\n').unwrap();
            let id = id
                .strip_prefix("Monkey ")
                .and_then(|m| m.strip_suffix(':'))
                .and_then(|m| m.parse::<usize>().ok())
                .unwrap();
            (id, declaration.trim().parse::<Monkey>().unwrap())
        })
        .collect()
}

struct Monkey {
    items: Vec<WorryLevel>,
    operator: Operator,
    operand: Operand,
    test_divisor: WorryLevel,
    targets: (usize, usize),
    inspected: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

impl Monkey {
    fn catch(&mut self, item: WorryLevel) {
        self.items.push(item);
    }

    fn throw_all(
        &mut self,
        worry_reduction_rate: WorryLevel,
    ) -> impl Iterator<Item = (usize, WorryLevel)> + '_ {
        self.inspected += self.items.len();
        let operator = self.operator;
        let operand = self.operand;
        let test_divisor = self.test_divisor;
        let (target_if_true, target_if_false) = self.targets;
        self.items.drain(..).map(move |i| {
            let operand = match operand {
                Operand::Value(v) => v,
                Operand::Old => i,
            };
            let worry_level = operator.exec(i, operand) / worry_reduction_rate;
            let target_monkey = if worry_level % test_divisor == 0 {
                target_if_true
            } else {
                target_if_false
            };
            (target_monkey, worry_level)
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Multiply,
    Divide,
    Add,
}

#[derive(Debug, Copy, Clone)]
enum Operand {
    Value(WorryLevel),
    Old,
}

impl Operator {
    fn exec(self, left: WorryLevel, right: WorryLevel) -> WorryLevel {
        match self {
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
            Operator::Add => left + right,
        }
    }
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines().map(|l| l.trim());
        let items = parse_starting_items(lines.next().unwrap());
        let (operator, operand) = parse_operation(lines.next().unwrap());
        let test_divisor = parse_test(lines.next().unwrap());
        let targets = parse_targets(lines);
        Ok(Self {
            items,
            operator,
            operand,
            test_divisor,
            targets,
            inspected: 0,
        })
    }
}

fn parse_targets<'a>(mut lines: impl Iterator<Item = &'a str>) -> (usize, usize) {
    let if_true = lines
        .next()
        .unwrap()
        .strip_prefix("If true: throw to monkey ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let if_false = lines
        .next()
        .unwrap()
        .strip_prefix("If false: throw to monkey ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    (if_true, if_false)
}

fn parse_test(declaration: &str) -> WorryLevel {
    declaration
        .strip_prefix("Test: divisible by ")
        .unwrap()
        .parse::<WorryLevel>()
        .unwrap()
}

fn parse_starting_items(declaration: &str) -> Vec<WorryLevel> {
    declaration
        .strip_prefix("Starting items: ")
        .unwrap()
        .split(", ")
        .map(|i| i.parse::<WorryLevel>().unwrap())
        .collect()
}

fn parse_operation(declaration: &str) -> (Operator, Operand) {
    let (operator, operand) = declaration
        .strip_prefix("Operation: new = old ")
        .unwrap()
        .split_once(' ')
        .unwrap();
    let operator = match operator {
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        "+" => Operator::Add,
        _ => panic!("Unexpected operator: {operator}"),
    };
    let operand = if operand == "old" {
        Operand::Old
    } else {
        Operand::Value(operand.parse::<WorryLevel>().unwrap())
    };
    (operator, operand)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    "#;

    const INPUT: &str = include_str!("day11/input.txt");

    #[rstest]
    #[case::example(EXAMPLE, 10605)]
    #[case::input(INPUT, 101436)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    #[rstest]
    #[case::example(EXAMPLE, 2713310158)]
    #[case::input(INPUT, 19754471646)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }

    #[rstest]
    #[case(r#"
    Starting items: 2
    Operation: new = old * 3
    Test: divisible by 2
        If true: throw to monkey 0
        If false: throw to monkey 1
    "#, vec![(0, 2)])]
    #[case(r#"
    Starting items: 2
    Operation: new = old * 6
    Test: divisible by 2
        If true: throw to monkey 0
        If false: throw to monkey 1
    "#, vec![(0, 4)])]
    #[case(r#"
    Starting items: 24
    Operation: new = old / 2
    Test: divisible by 2
        If true: throw to monkey 0
        If false: throw to monkey 1
    "#, vec![(0, 4)])]
    #[case(r#"
    Starting items: 3
    Operation: new = old * old
    Test: divisible by 2
        If true: throw to monkey 0
        If false: throw to monkey 1
    "#, vec![(1, 3)])]
    #[case(r#"
    Starting items: 2
    Operation: new = old * 3
    Test: divisible by 3
        If true: throw to monkey 0
        If false: throw to monkey 1
    "#, vec![(1, 2)])]
    #[case(r#"
    Starting items: 2
    Operation: new = old * 3
    Test: divisible by 3
        If true: throw to monkey 0
        If false: throw to monkey 42
    "#, vec![(42, 2)])]
    #[case(r#"
    Starting items: 2
    Operation: new = old * 3
    Test: divisible by 2
        If true: throw to monkey 12
        If false: throw to monkey 42
    "#, vec![(12, 2)])]
    #[case(r#"
    Starting items: 2, 3
    Operation: new = old * 3
    Test: divisible by 2
        If true: throw to monkey 0
        If false: throw to monkey 1
    "#, vec![(0, 2), (1, 3)])]
    #[case(r#"
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3
    "#, vec![(3,500), (3, 620)])]
    #[case(r#"
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0
    "#, vec![(0,20), (0, 23), (0, 27), (0, 26)])]
    #[case(r#"
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3
    "#, vec![(1,2080), (3, 1200), (3, 3136)])]
    fn should_throw(#[case] mut monkey: Monkey, #[case] expected_throws: Vec<(usize, WorryLevel)>) {
        let actual_throws: Vec<_> = monkey.throw_all(3).collect();
        assert_eq!(actual_throws, expected_throws);
        assert_eq!(monkey.throw_all(3).count(), 0); // <- And there is nothing left to throw
    }
}

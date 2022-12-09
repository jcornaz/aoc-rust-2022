use std::collections::HashSet;
use std::str::FromStr;

type Output = usize;

pub fn part_1(input: &str) -> Output {
    solve(input, 2)
}

pub fn part_2(input: &str) -> Output {
    solve(input, 10)
}

fn solve(input: &str, len: usize) -> Output {
    let mut tracker = Tracker::new(len);
    for (d, n) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        tracker.move_head(d.parse().unwrap(), n.parse().unwrap());
    }
    tracker.tail_visited()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            d => return Err(format!("unknown direction: {d}")),
        })
    }
}

struct Tracker {
    tail_positions: HashSet<(i32, i32)>,
    queue: Vec<(i32, i32)>,
}

impl Tracker {
    fn new(len: usize) -> Self {
        let mut tail_positions = HashSet::new();
        tail_positions.insert((0, 0));
        Self {
            tail_positions,
            queue: vec![(0, 0); len],
        }
    }

    fn move_head(&mut self, d: Direction, n: i32) {
        let (dx, dy) = match d {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };

        for _ in 0..n {
            self.queue[0].0 += dx;
            self.queue[0].1 += dy;
            for i in 1..self.queue.len() {
                if self.distance(i) > 1 {
                    self.queue[i].0 = self.queue[i - 1].0 - dx;
                    self.queue[i].1 = self.queue[i - 1].1 - dy;
                }
            }
            self.tail_positions.insert(*self.queue.last().unwrap());
        }
    }

    fn distance(&self, index: usize) -> i32 {
        let front_knot = self.queue[index - 1];
        let back_knot = self.queue[index];
        (front_knot.0 - back_knot.0)
            .abs()
            .max((front_knot.1 - back_knot.1).abs())
    }

    fn tail_visited(&self) -> Output {
        self.tail_positions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    "#;

    #[rstest]
    #[case("R 4", 4)]
    #[case("R 4\nR 4", 8)]
    #[case("R 4\nL 4", 4)]
    #[case("L 4", 4)]
    #[case("U 4", 4)]
    #[case("U 4\nU 2", 6)]
    #[case("U 4\nD 2", 4)]
    #[case("U 4\nD 2\nR 2", 5)]
    #[case("R 4\nU 4", 7)]
    #[case::example(EXAMPLE, 13)]
    #[case::input(INPUT, 6486)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    const INPUT: &str = include_str!("day09/input.txt");

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 36)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }
}

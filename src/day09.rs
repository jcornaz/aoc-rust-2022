use std::collections::HashSet;
use std::fmt::Debug;
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
                let head = self.queue[i - 1];
                Self::update_tail_knot(&mut self.queue[i], head);
            }
            self.tail_positions.insert(*self.queue.last().unwrap());
        }
    }

    fn tail_visited(&self) -> Output {
        self.tail_positions.len()
    }

    fn update_tail_knot((x, y): &mut (i32, i32), (hx, hy): (i32, i32)) {
        if (hx - *x).abs() > 1 {
            *x += (hx - *x).clamp(-1, 1);
            *y += (hy - *y).clamp(-1, 1);
        } else if hy - *y > 1 {
            *y += 1;
            *x = hx;
        } else if *y - hy > 1 {
            *y -= 1;
            *x = hx;
        }
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

    const INPUT: &str = include_str!("day09/input.txt");

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
    #[case("R 4\nU 4\nL 3", 9)]
    #[case::example(EXAMPLE, 13)]
    #[case::input(INPUT, 6486)]
    fn test_part_1(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_1(input.trim()), expected);
    }

    const LARGER_EXAMPLE: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
    "#;

    #[rstest]
    #[case::example(LARGER_EXAMPLE, 36)]
    #[case::input(INPUT, 2678)]
    fn test_part_2(#[case] input: &str, #[case] expected: Output) {
        assert_eq!(part_2(input.trim()), expected);
    }

    #[rstest]
    #[case(3, "R 2", 1)]
    #[case(3, "R 3", 2)]
    #[case(3, "R 3\nU 1", 2)]
    #[case(3, "R 3\nU 2", 3)]
    fn test_solve(#[case] len: usize, #[case] instructions: &str, #[case] expected: Output) {
        assert_eq!(solve(instructions, len), expected);
    }

    #[rstest]
    #[case((0,0), (0,0))]
    #[case((1,0), (0,0))]
    #[case((2,0), (1,0))]
    #[case((-1,0), (0,0))]
    #[case((-2,0), (-1,0))]
    #[case((0,1), (0,0))]
    #[case((0,2), (0,1))]
    #[case((0,-1), (0,0))]
    #[case((0,-2), (0,-1))]
    #[case((1,1), (0, 0))]
    #[case((2,1), (1, 1))]
    #[case((1,2), (1, 1))]
    #[case((-1,1), (0, 0))]
    #[case((-2,1), (-1, 1))]
    #[case((-1,2), (-1, 1))]
    #[case((-1,-1), (0, 0))]
    #[case((-2,-1), (-1, -1))]
    #[case((-1,-2), (-1, -1))]
    #[case((2,2), (1, 1))]
    #[case((-2,2), (-1, 1))]
    #[case((-2,-2), (-1, -1))]
    #[case((2,-2), (1, -1))]
    fn should_move_tail_towards_head(#[case] head: (i32, i32), #[case] expected: (i32, i32)) {
        let mut tail = (0, 0);
        Tracker::update_tail_knot(&mut tail, head);
        assert_eq!(tail, expected);
    }
}

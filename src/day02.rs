type Output = u64;

pub fn part_1(input: &str) -> Output {
    input.lines().map(part_1_score).sum()
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

fn part_1_score(input: &str) -> Output {
    let (opponent, me) = input.split_once(' ').unwrap();
    let opponent = Shape::from_str(opponent).unwrap();
    let me = match me {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!(),
    };
    me.play_against(opponent).score() + me.score()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn play_against(self, other: Self) -> Result {
        if self == other {
            return Result::Draw;
        }
        match (self, other) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Scissors, Shape::Paper)
            | (Shape::Paper, Shape::Rock) => Result::Win,
            _ => Result::Loose,
        }
    }

    fn score(self) -> Output {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Result {
    Win,
    Loose,
    Draw,
}

impl Result {
    fn score(self) -> Output {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loose => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
A Y
B X
C Z
    "#;

    const INPUT: &str = include_str!("day02/input.txt");

    #[rstest]
    #[case("A Y", 8)]
    #[case("B X", 1)]
    #[case("C Z", 6)]
    #[case::example(EXAMPLE, 15)]
    #[case::input(INPUT, 11841)]
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
    #[case(Shape::Rock, Shape::Scissors)]
    #[case(Shape::Paper, Shape::Rock)]
    #[case(Shape::Scissors, Shape::Paper)]
    fn wins(#[case] a: Shape, #[case] b: Shape) {
        assert_eq!(a.play_against(b), Result::Win);
        assert_eq!(b.play_against(a), Result::Loose);
    }

    #[rstest]
    fn draws(#[values(Shape::Rock, Shape::Paper, Shape::Scissors)] shape: Shape) {
        assert_eq!(shape.play_against(shape), Result::Draw);
    }

    #[rstest]
    #[case(Shape::Rock, 1)]
    #[case(Shape::Paper, 2)]
    #[case(Shape::Scissors, 3)]
    fn shape_score(#[case] shape: Shape, #[case] score: Output) {
        assert_eq!(shape.score(), score);
    }

    #[rstest]
    #[case(Result::Loose, 0)]
    #[case(Result::Draw, 3)]
    #[case(Result::Win, 6)]
    fn result_score(#[case] result: Result, #[case] score: Output) {
        assert_eq!(result.score(), score);
    }
}

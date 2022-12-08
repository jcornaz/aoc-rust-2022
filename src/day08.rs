use std::{convert::Infallible, str::FromStr};

use grid::Grid;

type Output = usize;

pub fn part_1(input: &str) -> Output {
    input.parse::<Forest>().unwrap().count_visible()
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

struct Forest(Grid<u8>);

impl Forest {
    fn count_visible(&self) -> usize {
        let (h, w) = self.0.size();
        (0..w)
            .flat_map(|x| (0..h).map(move |y| (x, y)))
            .filter(|&c| self.is_visible(c))
            .count()
    }

    fn is_visible(&self, (x, y): (usize, usize)) -> bool {
        let value = self.0[y][x];
        (0..y).all(|y| self.0[y][x] < value)
            || ((y + 1)..self.0.rows()).all(|y| self.0[y][x] < value)
            || (0..x).all(|x| self.0[y][x] < value)
            || ((x + 1)..self.0.cols()).all(|x| self.0[y][x] < value)
    }
}

impl FromStr for Forest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        Ok(Forest(Grid::from_vec(
            s.lines()
                .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
                .collect(),
            width,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
30373
25512
65332
33549
35390
    "#;

    const INPUT: &str = include_str!("day08/input.txt");

    #[rstest]
    #[case::example(EXAMPLE, 21)]
    #[case::input(INPUT, 1719)]
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
    #[case("1", (0, 0), true)]
    #[case("12\n34", (0, 0), true)]
    #[case("12\n34", (1, 1), true)]
    #[case("123\n115\n678", (1, 1), false)]
    #[case("123\n435\n678", (1, 1), true)]
    #[case("153\n335\n618", (1, 1), true)]
    #[case("153\n235\n648", (1, 1), true)]
    #[case("153\n432\n648", (1, 1), true)]
    fn should_reckognize_if_tree_is_visible(
        #[case] forest: Forest,
        #[case] coord: (usize, usize),
        #[case] expected: bool,
    ) {
        assert_eq!(forest.is_visible(coord), expected);
    }
}

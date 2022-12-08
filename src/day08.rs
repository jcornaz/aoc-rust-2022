use std::{convert::Infallible, iter, str::FromStr};

use grid::Grid;

type Output = usize;

pub fn part_1(input: &str) -> Output {
    input.parse::<Forest>().unwrap().count_visible()
}

pub fn part_2(input: &str) -> Output {
    input.parse::<Forest>().unwrap().max_score()
}

struct Forest(Grid<u8>);

impl Forest {
    fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (h, w) = self.0.size();
        (0..w).flat_map(move |x| (0..h).map(move |y| (x, y)))
    }

    fn count_visible(&self) -> usize {
        self.coords().filter(|&c| self.is_visible(c)).count()
    }

    fn max_score(&self) -> usize {
        self.coords()
            .map(|p| self.score(p))
            .max()
            .unwrap_or_default()
    }

    fn is_visible(&self, (x, y): (usize, usize)) -> bool {
        let value = self.0[y][x];
        (0..y).all(|y| self.0[y][x] < value)
            || ((y + 1)..self.0.rows()).all(|y| self.0[y][x] < value)
            || (0..x).all(|x| self.0[y][x] < value)
            || ((x + 1)..self.0.cols()).all(|x| self.0[y][x] < value)
    }

    fn score(&self, coord: (usize, usize)) -> usize {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|step| self.visible_trees(coord, step))
            .product()
    }

    fn visible_trees(&self, (x, y): (usize, usize), (dx, dy): (i32, i32)) -> usize {
        let value = self.0[y][x];
        iter::successors(Some((x as i32, y as i32)), |(x, y)| {
            Some((*x + dx, *y + dy))
        })
        .skip(1)
        .take_while(|(x, y)| (*y as usize) < self.0.rows() && (*x as usize) < self.0.cols())
        .map(|(x, y)| self.0[y as usize][x as usize])
        .enumerate()
        .find(|(_, t)| *t >= value)
        .map(|(i, _)| i + 1)
        .unwrap_or_else(|| {
            if dy < 0 {
                y
            } else if dy > 0 {
                self.0.rows() - y - 1
            } else if dx > 0 {
                self.0.cols() - x - 1
            } else {
                x
            }
        })
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
    #[case::example(EXAMPLE, 8)]
    #[case::input(INPUT, 590824)]
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

    #[rstest]
    #[case("1", (0, 0), 0)]
    #[case("123\n456\n789", (1, 1), 1)]
    #[case(EXAMPLE.trim().parse::<Forest>().unwrap(), (2, 3), 8)]
    fn should_compute_score(
        #[case] forest: Forest,
        #[case] coord: (usize, usize),
        #[case] expected: usize,
    ) {
        assert_eq!(forest.score(coord), expected);
    }

    #[rstest]
    #[case("1", (0, 0), 0)]
    #[case("1\n2", (0, 0), 0)]
    #[case("1\n2", (0, 1), 1)]
    #[case("1\n2\n3", (0, 1), 1)]
    #[case("1\n2\n3", (0, 2), 2)]
    #[case("1\n3\n3", (0, 2), 1)]
    fn should_find_number_of_tree_up(
        #[case] forest: Forest,
        #[case] coord: (usize, usize),
        #[case] expected: usize,
    ) {
        assert_eq!(forest.visible_trees(coord, (0, -1)), expected);
    }

    #[rstest]
    #[case("1", (0, 0), 0)]
    #[case("1\n2", (0, 0), 1)]
    #[case("1\n2", (0, 1), 0)]
    #[case("1\n2\n3", (0, 1), 1)]
    #[case("3\n2\n3", (0, 0), 2)]
    #[case("3\n3\n3", (0, 0), 1)]
    #[case("3\n3\n2", (0, 1), 1)]
    fn should_find_number_of_tree_down(
        #[case] forest: Forest,
        #[case] coord: (usize, usize),
        #[case] expected: usize,
    ) {
        assert_eq!(forest.visible_trees(coord, (0, 1)), expected);
    }

    #[rstest]
    #[case("1", (0, 0), 0)]
    #[case("3213", (0, 0), 3)]
    #[case("3211", (0, 0), 3)]
    #[case("3211", (1, 0), 2)]
    fn should_find_number_of_tree_right(
        #[case] forest: Forest,
        #[case] coord: (usize, usize),
        #[case] expected: usize,
    ) {
        assert_eq!(forest.visible_trees(coord, (1, 0)), expected);
    }

    #[rstest]
    #[case("1", (0, 0), 0)]
    #[case("3213", (0, 0), 0)]
    #[case("3213", (2, 0), 1)]
    #[case("1121", (2, 0), 2)]
    fn should_find_number_of_tree_left(
        #[case] forest: Forest,
        #[case] coord: (usize, usize),
        #[case] expected: usize,
    ) {
        assert_eq!(forest.visible_trees(coord, (-1, 0)), expected);
    }
}

use std::{convert::Infallible, str::FromStr};

type Output = u64;

pub fn part_1(input: &str) -> Output {
    input.parse().unwrap()
}

pub fn part_2(input: &str) -> Output {
    input.parse().unwrap()
}

fn shortest_path_length(map: &str, start_level: char, end_level: char) -> u32 {
    let mut map = Map::parse(map);
    let mut current_pos = map.find_and_replace('S', start_level);
    let target_pos = map.find_and_replace('E', end_level);
    let mut steps = 0;
    while current_pos != target_pos {
        current_pos = map
            .directions_from(current_pos)
            .map(|p| (p, map.get(p)))
            .max_by_key(|(pos, v)| if *pos == target_pos { char::MAX } else { *v })
            .unwrap()
            .0;
        steps += 1;
    }
    steps
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<char>>,
}

impl Map {
    fn parse(map: &str) -> Self {
        Self {
            cells: map.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> char {
        self.cells[y][x]
    }

    fn find_and_replace(&mut self, search: char, replace: char) -> (usize, usize) {
        let (x, y) = self
            .cells
            .iter()
            .enumerate()
            .find_map(|(y, row)| Some((row.iter().enumerate().find(|(_, c)| **c == search)?.0, y)))
            .unwrap();
        self.cells[y][x] = replace;
        (x, y)
    }

    fn directions_from(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let width = self.cells[0].len();
        let height = self.cells.len();
        let value = self.cells[y][x] as u16;
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
            .filter(move |(x, y)| *x < width && *y < height)
            .filter(move |(x, y)| self.cells[*y][*x] as u16 - 1 <= value)
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map::parse(s))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const EXAMPLE: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
    "#;

    const INPUT: &str = include_str!("day12/input.txt");

    #[rstest]
    #[ignore = "not implemented"]
    #[case::example(EXAMPLE, 31)]
    #[ignore = "not implemented"]
    #[case::input(INPUT, 0)]
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
    #[case("SE", 'a', 'a', 1)]
    #[case("SaE", 'a', 'a', 2)]
    #[case("aSE", 'a', 'a', 1)]
    #[case("ES", 'a', 'a', 1)]
    #[case("SE\nbc", 'a', 'd', 3)]
    // #[case("SbE\nbbb", 'a', 'c', 2)]
    fn find_shortest_path(
        #[case] map: &str,
        #[case] start_level: char,
        #[case] end_level: char,
        #[case] expected: u32,
    ) {
        assert_eq!(shortest_path_length(map, start_level, end_level), expected);
    }

    #[rstest]
    #[case("ab", (0, 0), &[(1, 0)])]
    #[case("ab", (1, 0), &[(0, 0)])]
    #[case("abc", (1, 0), &[(0, 0), (2, 0)])]
    #[case("ab\nab", (0, 0), &[(1, 0), (0, 1)])]
    #[case("ab\nab", (1, 1), &[(0, 1), (1, 0)])]
    #[case("ad\nbc", (0, 0), &[(0, 1)])]
    #[case("ad\nbc", (0, 1), &[(0, 0), (1, 1)])]
    #[case("ad\nbc", (1, 1), &[(0, 1), (1, 0)])]
    fn find_options(
        #[case] map: Map,
        #[case] from_pos: (usize, usize),
        #[case] expected: &[(usize, usize)],
    ) {
        let expected: HashSet<_> = expected.iter().copied().collect();
        let actual: HashSet<_> = map.directions_from(from_pos).collect();
        assert_eq!(actual, expected);
    }
}

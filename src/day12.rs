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
        println!("{:?}", current_pos);

        current_pos = [map.right_of(current_pos), map.left_of(current_pos)]
            .into_iter()
            .flatten()
            .max_by_key(|(pos, v)| if *pos == target_pos { char::MAX } else { *v })
            .unwrap()
            .0;
        steps += 1;
        assert!(steps < 100);
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

    fn right_of(&self, (mut x, y): (usize, usize)) -> Option<((usize, usize), char)> {
        x += 1;
        self.cells
            .get(y)
            .and_then(|row| row.get(x))
            .map(|&c| ((x, y), c))
    }

    fn left_of(&self, (mut x, y): (usize, usize)) -> Option<((usize, usize), char)> {
        if x == 0 {
            return None;
        }
        x -= 1;
        self.cells
            .get(y)
            .and_then(|row| row.get(x))
            .map(|&c| ((x, y), c))
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
}

#[cfg(test)]
mod tests {
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
    // #[case("SE\nbc", 'a', 'd', 3)]
    fn find_shortest_path(
        #[case] map: &str,
        #[case] start_level: char,
        #[case] end_level: char,
        #[case] expected: u32,
    ) {
        assert_eq!(shortest_path_length(map, start_level, end_level), expected);
    }
}

use itertools::iproduct;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
struct Puzzle {
    guard: Guard,
    rows: usize,
    cols: usize,
    objects: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Guard {
    pt: (usize, usize),
    dir: Direction,
}

impl Guard {
    fn walk(&self) -> Option<(usize, usize)> {
        let (x, y) = self.pt;
        match &self.dir {
            Direction::Up => x.checked_sub(1).map(|nx| (nx, y)),
            Direction::Right => Some((x, y + 1)),
            Direction::Down => Some((x + 1, y)),
            Direction::Left => y.checked_sub(1).map(|ny| (x, ny)),
        }
    }
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let (_, map) = parse_map(input).unwrap();
        let mut guard: Option<Guard> = None;
        let mut objects = Vec::new();

        let guards = vec!['^', 'v', '<', '>'];
        for (x, y) in iproduct!(0..map.len(), 0..map[0].len()) {
            let cell = map[x][y];
            let pt = (x, y);

            if cell == '#' {
                objects.push((x, y));
            }

            if guards.contains(&cell) {
                let dir = match cell {
                    '^' => Direction::Up,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    _ => panic!("guard must be pointing one direction"),
                };
                guard = Some(Guard { pt, dir });
            }
        }

        let guard = guard.unwrap();

        Self {
            guard,
            objects,
            rows: map.len(),
            cols: map[0].len(),
        }
    }

    fn move_until_out(&mut self) -> usize {
        let mut pos = Vec::new();

        loop {
            if let Some((nx, ny)) = self.guard.walk() {
                if nx >= self.rows || ny >= self.cols {
                    break;
                }
                if self.objects.contains(&(nx, ny)) {
                    self.guard.dir = self.guard.dir.rotate();
                } else {
                    pos.push(self.guard.pt);
                    self.guard.pt = (nx, ny);
                }
            } else {
                // walked less than 0
                break;
            }
        }

        pos.sort();
        pos.dedup();
        pos.iter().count() + 1
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(one_of(".#^><v")))(input)
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let distinct_pos = Puzzle::new(input).move_until_out();

    println!("result 1: {}", distinct_pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_input() {
        let given = ".^.
#..";

        let expected = Puzzle {
            guard: Guard {
                pt: (0, 1),
                dir: Direction::Up,
            },
            rows: 2,
            cols: 3,
            objects: vec![(1, 0)],
        };
        let actual = Puzzle::new(given);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_move_until_out() {
        let given = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let expected = 41;
        let actual = Puzzle::new(given).move_until_out();
        assert_eq!(expected, actual);
    }
}

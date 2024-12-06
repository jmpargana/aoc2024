use std::collections::HashSet;

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
    objects: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let mut objects = HashSet::new();

        let guards = vec!['^', 'v', '<', '>'];
        for (x, y) in iproduct!(0..map.len(), 0..map[0].len()) {
            let cell = map[x][y];
            let pt = (x, y);

            if cell == '#' {
                objects.insert((x, y));
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

    fn move_until_out(&self) -> usize {
        let (pos, _) = self.has_cycle();
        pos.iter().count() + 1
    }

    fn count_cycles(&self) -> usize {
        let (pos, _) = self.has_cycle();
        let mut cycles = HashSet::new();

        for p in pos {
            let mut objects = self.objects.clone();
            objects.insert(p);
            let np = Puzzle {
                objects,
                ..self.clone()
            };
            let (cycle, is_cycle) = np.has_cycle();
            if is_cycle {
                cycles.insert(cycle);
            }
        }
        cycles.iter().count()
    }

    fn has_cycle(&self) -> (Vec<(usize, usize)>, bool) {
        let mut puzzle = self.clone();
        let mut pos = HashSet::new();
        loop {
            if let Some((nx, ny)) = puzzle.guard.walk() {
                if pos.len() >= puzzle.rows * puzzle.cols {
                    return (set_to_vec(pos), true);
                }

                if nx >= puzzle.rows || ny >= puzzle.cols {
                    break;
                }
                if puzzle.objects.contains(&(nx, ny)) {
                    puzzle.guard.dir = puzzle.guard.dir.rotate();
                } else {
                    let np = (
                        puzzle.guard.pt.0,
                        puzzle.guard.pt.1,
                        puzzle.guard.dir.clone(),
                    );
                    if !pos.insert(np) {
                        return (set_to_vec(pos), true);
                    }
                    puzzle.guard.pt = (nx, ny);
                }
            } else {
                break;
            }
        }
        let mut pos = set_to_vec(pos);
        pos.sort();
        pos.dedup();
        (pos, false)
    }
}

fn set_to_vec(v: HashSet<(usize, usize, Direction)>) -> Vec<(usize, usize)> {
    v.into_iter().map(|(x, y, _)| (x, y)).collect()
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(one_of(".#^><v")))(input)
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let puzzle = Puzzle::new(input);
    let distinct_pos = puzzle.move_until_out();
    let cycles = puzzle.count_cycles();

    println!("result 1: {}", distinct_pos);
    println!("result 2: {}", cycles);
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn should_have_cycle() {
        let given = "....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#...";

        let (_, actual) = Puzzle::new(given).has_cycle();
        assert!(actual);
    }

    #[test]
    fn should_not_have_cycle() {
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

        let (_, actual) = Puzzle::new(given).has_cycle();
        assert!(!actual);
    }

    #[test]
    fn should_have_n_cycles() {
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

        let expected = 6;
        let actual = Puzzle::new(given).count_cycles();
        assert_eq!(expected, actual);
    }
}

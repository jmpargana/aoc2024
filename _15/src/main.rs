use std::cell::RefCell;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Kind {
    Box,
    Wall,
    Empty,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Dir {
    N,
    W,
    E,
    S,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Map {
    m: Vec<Vec<Kind>>,
    robot: (usize, usize),
}

impl Map {
    fn next(&mut self, dir: Dir) {
        let (dr, dc) = match dir {
            Dir::N => (-1, 0),
            Dir::W => (0, -1),
            Dir::E => (0, 1),
            Dir::S => (1, 0),
        };

        let (r, c) = self.robot;
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr < 0 || nr >= self.m.len() as isize || nc < 0 || nc >= self.m[0].len() as isize {
            return;
        }

        let (nr, nc) = (nr as usize, nc as usize);

        match self.m[nr][nc] {
            Kind::Wall => {
                return;
            }
            Kind::Empty => {
                self.robot = (nr, nc);
            }
            Kind::Box => {
                let mut boxes_positions = vec![(nr, nc)];

                let mut cur_r = nr;
                let mut cur_c = nc;
                loop {
                    let next_r = cur_r as isize + dr;
                    let next_c = cur_c as isize + dc;

                    if next_r < 0
                        || next_r >= self.m.len() as isize
                        || next_c < 0
                        || next_c >= self.m[0].len() as isize
                    {
                        return;
                    }

                    let (next_r_u, next_c_u) = (next_r as usize, next_c as usize);
                    match self.m[next_r_u][next_c_u] {
                        Kind::Wall => {
                            return;
                        }
                        Kind::Box => {
                            boxes_positions.push((next_r_u, next_c_u));
                            cur_r = next_r_u;
                            cur_c = next_c_u;
                        }
                        Kind::Empty => {
                            self.m[next_r_u][next_c_u] = Kind::Box;
                            for i in (1..boxes_positions.len()).rev() {
                                let (prev_r, prev_c) = boxes_positions[i - 1];
                                let (cur_r, cur_c) = boxes_positions[i];
                                self.m[cur_r][cur_c] = Kind::Box;
                                self.m[prev_r][prev_c] = Kind::Empty;
                            }

                            let (first_box_r, first_box_c) = boxes_positions[0];
                            self.m[first_box_r][first_box_c] = Kind::Empty;
                            self.robot = (nr, nc);

                            return;
                        }
                    }
                }
            }
        }
    }

    fn count_gps(&self) -> usize {
        self.m
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, &k)| if k == Kind::Box { 100 * i + j } else { 0 })
            })
            .sum()
    }
}

fn sum_gps(m: &mut Map, dirs: &Vec<Dir>) -> usize {
    for &d in dirs {
        m.next(d);
    }
    m.count_gps()
}

fn parse_map(input: &str) -> Map {
    let robot = RefCell::new((0, 0));
    let m = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Kind::Empty,
                    '#' => Kind::Wall,
                    'O' => Kind::Box,
                    '@' => {
                        *robot.borrow_mut() = (i, j);
                        Kind::Empty
                    }
                    _ => unreachable!("parsing"),
                })
                .collect()
        })
        .collect();
    let map = Map {
        m,
        robot: *robot.borrow(),
    };
    map
}

fn parse_dirs(input: &str) -> Vec<Dir> {
    input
        .chars()
        .map(|c| match c {
            '^' => Some(Dir::N),
            '<' => Some(Dir::W),
            '>' => Some(Dir::E),
            'v' => Some(Dir::S),
            _ => None,
        })
        .filter_map(|x| x)
        .collect()
}

fn parse(input: &str) -> (Map, Vec<Dir>) {
    let mut parts = input.splitn(2, "\n\n");
    let map_part = parts.next().unwrap();
    let dirs_part = parts.next().unwrap();

    let map = parse_map(map_part);
    let dirs = parse_dirs(dirs_part.trim());
    (map, dirs)
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let (mut map, dirs) = parse(input);
    let sum = sum_gps(&mut map, &dirs);
    println!("result 1: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_map() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let map = parse_map(input);
        assert_eq!(map.m[1][3], Kind::Box);
        assert_eq!(map.m[1][0], Kind::Wall);
        assert_eq!(map.m[1][1], Kind::Empty);
        assert_eq!(map.m[4][4], Kind::Empty);
        assert_eq!(map.robot, (4, 4));
    }

    #[test]
    fn should_move_up() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let mut map = parse_map(input);
        map.next(Dir::N);
        assert_eq!(map.robot, (3, 4));
    }

    #[test]
    fn should_move_box() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let mut map = parse_map(input);
        assert_eq!(map.m[4][2], Kind::Empty);
        map.next(Dir::W);
        assert_eq!(map.robot, (4, 3));
        assert_eq!(map.m[4][2], Kind::Box);
    }

    #[test]
    fn should_move_two_boxes() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#.OO@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let mut map = parse_map(input);
        assert_eq!(map.m[4][1], Kind::Empty);
        map.next(Dir::W);
        assert_eq!(map.robot, (4, 3));
        assert_eq!(map.m[4][2], Kind::Box);
        assert_eq!(map.m[4][1], Kind::Box);
    }

    #[test]
    fn should_not_move_wall() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..#@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let mut map = parse_map(input);
        map.next(Dir::W);
        assert_eq!(map.robot, (4, 4));
    }

    #[test]
    fn should_not_move_boxes_with_wall() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
##OO@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let mut map = parse_map(input);
        assert_eq!(map.m[4][3], Kind::Box);
        assert_eq!(map.m[4][2], Kind::Box);
        map.next(Dir::W);
        assert_eq!(map.robot, (4, 4));
        assert_eq!(map.m[4][3], Kind::Box);
        assert_eq!(map.m[4][2], Kind::Box);
    }

    #[test]
    fn should_example1() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let (mut map, dirs) = parse(input);
        let actual = sum_gps(&mut map, &dirs);
        let expected = 10092;
        assert_eq!(actual, expected);
    }
}

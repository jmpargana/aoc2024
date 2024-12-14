// Each point is a node (x,y)
// Edges are represented by move

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u32},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Game {
    a: (usize, usize),
    b: (usize, usize),
    goal: (usize, usize),
}

fn find_min_tokens(g: &Game) -> Option<usize> {
    let start = (0, 0, 0, 0);
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((0, start)));

    let mut visited = HashSet::new();

    while let Some(Reverse((curr_cost, (x, y, a_count, b_count)))) = min_heap.pop() {
        if (x, y) == g.goal {
            return Some(curr_cost);
        }
        if visited.contains(&(x, y, a_count, b_count)) {
            continue;
        }
        visited.insert((x, y, a_count, b_count));
        if a_count < 100 {
            let a = (x + g.a.0, y + g.a.1, a_count + 1, b_count);
            if a.0 <= g.goal.0 && a.1 <= g.goal.1 {
                min_heap.push(Reverse((curr_cost + 3, a)));
            }
        }

        if b_count < 100 {
            let b = (x + g.b.0, y + g.b.1, a_count, b_count + 1);
            if b.0 <= g.goal.0 && b.1 <= g.goal.1 {
                min_heap.push(Reverse((curr_cost + 1, b)));
            }
        }
    }

    None
}

fn total_fewest_tokens(games: &Vec<Game>) -> usize {
    games
        .iter()
        .map(|g| find_min_tokens(g))
        .filter_map(|x| x)
        .sum()
}

fn parse_button(input: &str) -> IResult<&str, (u32, u32)> {
    preceded(
        multispace0,
        tuple((preceded(tag("X+"), u32), preceded(tag(", Y+"), u32))),
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, (u32, u32)> {
    preceded(
        multispace0,
        tuple((preceded(tag("X="), u32), preceded(tag(", Y="), u32))),
    )(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((
            preceded(tag("Button A:"), parse_button),
            preceded(multispace1, preceded(tag("Button B:"), parse_button)),
            preceded(multispace1, preceded(tag("Prize:"), parse_prize)),
        )),
        |((ax, ay), (bx, by), (gx, gy))| Game {
            a: (ax as usize, ay as usize),
            b: (bx as usize, by as usize),
            goal: (gx as usize, gy as usize),
        },
    )(input)
}

fn parse(input: &str) -> Vec<Game> {
    let (_, games) = separated_list0(multispace1, parse_game)(input).unwrap();
    games
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let games = parse(&input);
    let total = total_fewest_tokens(&games);

    println!("result 1: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_smallest() {
        let given = Game {
            a: (94, 34),
            b: (22, 67),
            goal: (8400, 5400),
        };
        let expected = 280;
        let actual = find_min_tokens(&given).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_find_nothing() {
        let given = Game {
            a: (26, 66),
            b: (67, 21),
            goal: (12748, 12176),
        };
        let expected = None;
        let actual = find_min_tokens(&given);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_find_some() {
        let given = Game {
            a: (17, 86),
            b: (84, 37),
            goal: (7870, 6450),
        };
        let expected = Some(200);
        let actual = find_min_tokens(&given);
        assert_eq!(expected, actual);
    }
}

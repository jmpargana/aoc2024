use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, multispace0},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Quadrant {
    NW,
    NE,
    SE,
    SW,
}

impl Robot {
    fn next(&mut self, limits: (isize, isize)) {
        self.p = (
            (self.p.0 + self.v.0).rem_euclid(limits.0),
            (self.p.1 + self.v.1).rem_euclid(limits.1),
        );
    }

    fn iter(&mut self, n: usize, limits: (isize, isize)) {
        for _ in 0..n {
            self.next(limits);
        }
    }

    fn to_quadrant(&self, limits: (isize, isize)) -> Option<Quadrant> {
        let mid = (limits.0 / 2, limits.1 / 2);
        if self.p.0 == mid.0 || self.p.1 == mid.1 {
            return None;
        }
        match (self.p.0 < mid.0, self.p.1 < mid.1) {
            (true, true) => Some(Quadrant::NW),
            (false, true) => Some(Quadrant::NE),
            (true, false) => Some(Quadrant::SW),
            (false, false) => Some(Quadrant::SE),
        }
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let parse_signed_number = recognize(tuple((opt(alt((char('+'), char('-')))), digit1)));
    map_res(parse_signed_number, |s: &str| s.parse::<isize>())(input)
}

fn parse_pair(input: &str) -> IResult<&str, (isize, isize)> {
    tuple((parse_isize, preceded(char(','), parse_isize)))(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        tuple((
            preceded(tag("p="), parse_pair),
            preceded(multispace0, preceded(tag("v="), parse_pair)),
        )),
        |(p, v)| Robot { p, v },
    )(input)
}

fn parse(input: &str) -> Vec<Robot> {
    let (_, v) = separated_list1(line_ending, parse_robot)(input).unwrap();
    v
}

fn product_robots(robots: &mut Vec<Robot>, limits: (isize, isize)) -> usize {
    robots.iter_mut().for_each(|r| r.iter(100, limits));
    robots
        .iter()
        .filter_map(|r| r.to_quadrant(limits))
        .fold(HashMap::new(), |mut acc, q| {
            *acc.entry(q).or_insert(0) += 1;
            acc
        })
        .values()
        .product()
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let mut robots = parse(input);
    let limits = (101, 103);
    let safety_factor = product_robots(&mut robots, limits);

    println!("result 1: {safety_factor}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_next1() {
        let limits = (11, 7);
        let mut given = Robot {
            p: (2, 4),
            v: (2, -3),
        };
        let expected = (4, 1);
        given.next(limits);
        assert_eq!(given.p, expected)
    }

    #[test]
    fn should_get_next5() {
        let limits = (11, 7);
        let mut given = Robot {
            p: (2, 4),
            v: (2, -3),
        };
        let expected = (1, 3);
        given.iter(5, limits);
        assert_eq!(given.p, expected)
    }

    #[test]
    fn should_input1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let mut given = parse(input);
        let limits = (11, 7);
        let actual = product_robots(&mut given, limits);
        let expected = 12;
        assert_eq!(actual, expected)
    }
}

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u32},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    goal: (i64, i64),
}

fn find_min_tokens(g: &Game) -> Option<i64> {
    let (x1, x2) = g.a;
    let (y1, y2) = g.b;
    let (z1, z2) = g.goal;
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return None;
    }
    Some(a * 3 + b)
}

fn total_fewest_tokens(games: &Vec<Game>) -> i64 {
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
            a: (ax as i64, ay as i64),
            b: (bx as i64, by as i64),
            goal: (gx as i64, gy as i64),
        },
    )(input)
}

fn parse(input: &str) -> Vec<Game> {
    let (_, games) = separated_list0(multispace1, parse_game)(input).unwrap();
    games
}

fn add_million(games: &Vec<Game>) -> Vec<Game> {
    games
        .iter()
        .map(|g| Game {
            a: g.a,
            b: g.b,
            goal: (g.goal.0 + 10_000_000_000_000, g.goal.1 + 10_000_000_000_000),
        })
        .collect()
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let games = parse(&input);
    let total = total_fewest_tokens(&games);
    let total_big = total_fewest_tokens(&add_million(&games));

    println!("result 1: {total}");
    println!("result 2: {total_big}");
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

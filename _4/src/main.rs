use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("puzzle.txt")?;

    let buf = BufReader::new(f);

    let mut levels: Vec<Vec<char>> = Vec::new();

    for line in buf.lines() {
        let line = line?;
        levels.push(line.chars().collect());
    }

    let total_xmas = xmas_in_mat(levels);
    // let sum_of_enabled_multiples = sum_mult_parsed_enabled(&buf);

    println!("result 1: {}", total_xmas);
    // println!("result 2: {}", sum_of_enabled_multiples);

    Ok(())
}

fn xmas_in_mat(given: Vec<Vec<char>>) -> u32 {
    let word = vec!['X', 'M', 'A', 'S'];
    let mut counter = 0;

    let directions = vec![
        (0, 1),   // east
        (0, -1),  // west
        (-1, 0),  // north
        (1, 0),   // south
        (-1, 1),  // northeast
        (-1, -1), // northwest
        (1, -1),  // southwest
        (1, 1),   // northeast
    ];

    for i in 0..given.len() {
        for j in 0..given[0].len() {
            if given[i][j] == 'X' {
                for (ro, co) in &directions {
                    // row and col offset
                    let mut match_found = true;

                    for z in 1..word.len() {
                        // new row and new col
                        let nr = i as isize + z as isize * ro;
                        let nc = j as isize + z as isize * co;

                        // boundary check
                        if nr < 0
                            || nc < 0
                            || nr >= given.len() as isize
                            || nc >= given[0].len() as isize
                        {
                            match_found = false;
                            break;
                        }

                        if given[nr as usize][nc as usize] != word[z] {
                            match_found = false;
                            break;
                        }
                    }

                    if match_found {
                        counter += 1;
                    }
                }
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_mat(s: &str) -> Vec<Vec<char>> {
        s.lines().map(|a| a.chars().collect()).collect()
    }

    #[test]
    fn should_match_all() {
        let xmas = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let given = str_to_mat(xmas);
        let expected = 4;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_east() {
        let xmas = "..X...
.XMAS.
.X....";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_west() {
        let xmas = "..X...
.SAMX.
.X....";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_west_and_east() {
        let xmas = "SAMXMAS
.SA.X..
.X.....";
        let given = str_to_mat(xmas);
        let expected = 2;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_north() {
        let xmas = "SS.X.AS
.AA.X..
.MA.X..
.X.....";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_south() {
        let xmas = "SX.X.AS
.MA.X..
.AA.X..
.S.....";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_southwest() {
        let xmas = "SX.X.AS
..M.X..
.AA.X..
S......";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_southeast() {
        let xmas = "S..X.AS
.MA.M..
.AA.XA.
.S....S";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_northwest() {
        let xmas = "SX.X.AS
.AA.X..
..M.X..
...X...";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_northeast() {
        let xmas = "SX.S.AS
.MA.X..
.MA.X..
XS.....";
        let given = str_to_mat(xmas);
        let expected = 1;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_input() {
        let xmas = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let given = str_to_mat(xmas);
        let expected = 18;
        let actual = xmas_in_mat(given);

        assert_eq!(actual, expected);
    }
}


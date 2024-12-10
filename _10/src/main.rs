use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Map {
    pub rows: usize,
    pub cols: usize,
    pub values: HashMap<(usize, usize), usize>,
    pub trailhead: Vec<(usize, usize)>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mat = parse(input);
        let mut values = HashMap::new();
        let mut trailhead = Vec::new();

        for (i, row) in mat.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                values.insert((i, j), cell as usize);
                if cell == 0 {
                    trailhead.push((i, j));
                }
            }
        }

        Self {
            values,
            rows: mat.len(),
            cols: mat[0].len(),
            trailhead,
        }
    }

    fn score(&self) -> usize {
        self.trailhead
            .iter()
            .map(|&th| self.trailhead_score(th))
            .sum()
    }

    fn score_ratings(&self) -> usize {
        self.trailhead
            .iter()
            .map(|&th| self.trailhead_score_all_routes(th))
            .sum()
    }

    // wfs
    fn trailhead_score_all_routes(&self, th: (usize, usize)) -> usize {
        let mut stack = VecDeque::new();
        let mut counter = 0;

        stack.push_front((th, vec![th]));

        while let Some(((x, y), path)) = stack.pop_front() {
            let curr = *self.values.get(&(x, y)).unwrap();

            if curr == 9 {
                counter += 1;
                continue;
            }

            let surroundings = valid_surroundings(x, y, self.rows, self.cols);
            for neighbor in surroundings {
                if path.contains(&neighbor) {
                    continue; // to avoid cycles
                }
                if let Some(&n) = self.values.get(&neighbor) {
                    if n == curr + 1 {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        stack.push_front((neighbor, new_path));
                    }
                }
            }
        }

        counter
    }

    // dgs
    fn trailhead_score(&self, th: (usize, usize)) -> usize {
        let mut stack = VecDeque::new();
        let mut visited = HashMap::new();
        let mut counter = 0;

        stack.push_front(th);

        while let Some((x, y)) = stack.pop_front() {
            if visited.get(&(x, y)).is_some() {
                continue;
            }
            visited.insert((x, y), true);

            let current_value = *self.values.get(&(x, y)).unwrap();

            if current_value == 9 {
                counter += 1;
                continue;
            }

            let surroundings = valid_surroundings(x, y, self.rows, self.cols);
            for (nx, ny) in surroundings {
                if let Some(&neighbor_value) = self.values.get(&(nx, ny)) {
                    if neighbor_value == current_value + 1 {
                        stack.push_front((nx, ny)); // Add to stack for further exploration
                    }
                }
            }
        }

        counter
    }
}

fn valid_surroundings(x: usize, y: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    vec![
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
    .into_iter()
    .filter(|&(nx, ny)| nx < rows && ny < cols) // Ensure valid bounds
    .collect()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|a| a.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let m = Map::new(input);
    let score = m.score();
    let score_ratings = m.score_ratings();

    println!("result 1: {}", score);
    println!("result 2: {}", score_ratings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_multiple_score_for_each_trailhead() {
        let input = "3330333
3331333
3332333
6543456
7333337
8333338
9333339";

        let expected = 2;
        let m = Map::new(input);
        let actual = m.trailhead_score((0, 3));
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_sum_all_trailheads() {
        let input = "2290229
2221298
2222227
6543456
7652987
8762222
9872222";

        let expected = 4;
        let m = Map::new(input);
        let actual = m.score();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_example_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let expected = 36;
        let m = Map::new(input);
        let actual = m.score();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_find_trailheads() {
        let input = "3330333
3331333
3332333
6543456
7333337
8333338
9333339";

        let expected = vec![(0, 3)];
        let actual = Map::new(input);
        assert_eq!(expected, actual.trailhead);
    }

    #[test]
    fn should_count_rating() {
        let input = "1190819
1191698
1172817
6543456
7651987
8761111
9871111";

        let expected = 13;
        let m = Map::new(input);
        let actual = m.trailhead_score_all_routes((0, 3));
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_count_rating2() {
        let input = "8888808
8843218
8858828
8865438
2272242
2287652
2292228";

        let expected = 3;
        let m = Map::new(input);
        let actual = m.trailhead_score_all_routes((0, 5));
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_count_rating3() {
        let input = "012345
123456
234567
345678
416789
567891";

        let expected = 227;
        let m = Map::new(input);
        let actual = m.trailhead_score_all_routes((0, 0));
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_example_part2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let expected = 81;
        let m = Map::new(input);
        let actual = m.score_ratings();
        assert_eq!(expected, actual);
    }
}

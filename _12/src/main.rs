use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    targets: HashMap<char, usize>,
}

impl Map {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        // count occurrences (area)
        let mut targets = HashMap::new();
        for line in input.lines() {
            for c in line.chars() {
                *targets.entry(c).or_insert(0) += 1;
            }
        }

        Self {
            grid: grid.clone(),
            rows: grid.len(),
            cols: grid[0].len(),
            targets,
        }
    }

    fn find_regions(&self, target: char) -> Vec<Vec<(usize, usize)>> {
        // 0-1 grid
        let mut visited = vec![vec![false; self.cols]; self.rows];
        let mut clusters = Vec::new();

        for r in 0..self.rows {
            for c in 0..self.cols {
                if !visited[r][c] && self.grid[r][c] == target {
                    let mut cluster = Vec::new();
                    let mut queue = VecDeque::new();
                    visited[r][c] = true;
                    queue.push_back((r, c));

                    while let Some((cr, cc)) = queue.pop_front() {
                        cluster.push((cr, cc));

                        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                        for (dr, dc) in directions.iter() {
                            let nr = cr as isize + dr;
                            let nc = cc as isize + dc;

                            if nr >= 0
                                && nr < self.rows as isize
                                && nc >= 0
                                && nc < self.cols as isize
                            {
                                let (nr, nc) = (nr as usize, nc as usize);
                                if !visited[nr][nc] && self.grid[nr][nc] == target {
                                    visited[nr][nc] = true;
                                    queue.push_back((nr, nc));
                                }
                            }
                        }
                    }

                    clusters.push(cluster);
                }
            }
        }

        clusters
    }

    fn fencing(&self, target: char) -> usize {
        self.find_regions(target)
            .into_iter()
            .map(|cl| cl.len() * self.perimeter(&cl))
            .sum()
    }

    fn perimeter(&self, cl: &Vec<(usize, usize)>) -> usize {
        let mut perimeter = 0;
        for &(r, c) in cl.iter() {
            let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                    if !cl.contains(&(nr as usize, nc as usize)) {
                        perimeter += 1;
                    }
                } else {
                    // oob
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn fencing_price(&self) -> usize {
        println!("regions: {:?}", self.targets);
        self.targets
            .iter()
            .map(|(&target, _)| self.fencing(target))
            .sum()
    }
}

fn total_perimeters(input: &str) -> usize {
    let m = Map::new(input);
    m.fencing_price()
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let part1 = total_perimeters(input);
    println!("result 1: {part1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_regions() {
        let given = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let m = Map::new(given);
        let actual = m.find_regions('X');
        let expected = vec![vec![(1, 1)], vec![(1, 3)], vec![(3, 1)], vec![(3, 3)]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_sum_fencing() {
        let given = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let actual = total_perimeters(given);
        let expected = 1930;
        assert_eq!(actual, expected);
    }
}

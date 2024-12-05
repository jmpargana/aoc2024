use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("puzzle.txt")?;

    let buf = BufReader::new(f);

    let mut rules = Vec::new();
    let mut arrays = Vec::new();
    let mut is_rules_section = true;

    for line in buf.lines() {
        let line = line?;
        let content = line.trim();
        if line.is_empty() {
            is_rules_section = false;
            continue;
        }

        if is_rules_section {
            if let Some((a, b)) = content.split_once('|') {
                if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                    rules.push((a, b));
                }
            }
        } else {
            let array = content
                .split(',')
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect::<Vec<i32>>();
            arrays.push(array);
        }
    }

    let total_mid_valid_lines = sum_correct_page_ordering(&rules, &arrays);
    let total_mid_invalid_lines = sum_incorrect_page_ordering(&rules, &arrays);

    println!("result 1: {}", total_mid_valid_lines);
    println!("result 2: {}", total_mid_invalid_lines);

    Ok(())
}

fn is_match(rules: &[(i32, i32)], given: &[i32]) -> bool {
    let index_map: HashMap<i32, usize> = given.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    rules.iter().all(|&(k, v)| {
        if let (Some(&k_idx), Some(&v_idx)) = (index_map.get(&k), index_map.get(&v)) {
            k_idx < v_idx
        } else {
            true
        }
    })
}

fn sort_incorrect(rules: &[(i32, i32)], given: &[i32]) -> Vec<i32> {
    let mut dg = HashMap::new();
    for (k, v) in rules.iter() {
        dg.entry(*k).or_insert(Vec::new()).push(*v);
    }
    let dg = extracted_relevant_rules(dg, given);
    topological_sort(&dg, given)
}

fn extracted_relevant_rules(graph: HashMap<i32, Vec<i32>>, nums: &[i32]) -> HashMap<i32, Vec<i32>> {
    let num_set: std::collections::HashSet<i32> = nums.iter().cloned().collect();
    let mut filtered_graph = HashMap::new();

    for (&node, neighbors) in graph.iter() {
        if num_set.contains(&node) {
            let filtered_neighbors: Vec<i32> = neighbors
                .iter()
                .cloned()
                .filter(|&n| num_set.contains(&n))
                .collect();
            filtered_graph.insert(node, filtered_neighbors);
        }
    }

    filtered_graph
}

fn topological_sort(graph: &HashMap<i32, Vec<i32>>, arr: &[i32]) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut adj_list = HashMap::new();

    for &it in arr {
        in_degree.insert(it, 0);
        adj_list.insert(it, Vec::new());
    }

    for (&u, vvs) in graph {
        for &v in vvs {
            adj_list.entry(u).or_default().push(v);
            *in_degree.entry(v).or_default() += 1;
        }
    }

    let mut q: VecDeque<i32> = VecDeque::new();

    for (&n, &degree) in &in_degree {
        if degree == 0 {
            q.push_back(n);
        }
    }

    let mut sorted_order = Vec::new();

    while let Some(n) = q.pop_front() {
        sorted_order.push(n);

        if let Some(vvs) = adj_list.get(&n) {
            for &v in vvs.iter() {
                if let Some(in_deg) = in_degree.get_mut(&v) {
                    *in_deg -= 1;
                    if *in_deg == 0 {
                        q.push_back(v)
                    }
                }
            }
        }
    }

    sorted_order
}

fn sum_correct_page_ordering(rules: &[(i32, i32)], given: &[Vec<i32>]) -> i32 {
    given
        .iter()
        .filter(|arr| is_match(&rules, &arr))
        .map(|arr| {
            let mid = arr.len() / 2;
            arr[mid]
        })
        .sum()
}

fn sum_incorrect_page_ordering(rules: &[(i32, i32)], given: &[Vec<i32>]) -> i32 {
    given
        .iter()
        .filter(|arr| !is_match(&rules, &arr))
        .map(|arr| sort_incorrect(&rules, &arr))
        .map(|arr| {
            let mid = arr.len() / 2;
            arr[mid]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match() {
        let rules = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let given = [75, 47, 61, 53, 29];
        let actual = is_match(&rules, &given);
        assert!(actual);
    }

    #[test]
    fn should_not_match() {
        let rules = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let given = [97, 13, 75, 29, 47];
        let actual = is_match(&rules, &given);
        assert!(!actual);
    }

    #[test]
    fn should_sum_valid() {
        let rules = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let given = [
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let actual = sum_correct_page_ordering(&rules, &given);
        let expected = 143;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_sort_arr_correctly() {
        let rules = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let given = [97, 13, 75, 29, 47];
        let expected = [97, 75, 47, 29, 13];
        let actual = sort_incorrect(&rules, &given);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_sum_invalid() {
        let rules = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let given = [
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let actual = sum_incorrect_page_ordering(&rules, &given);
        let expected = 123;
        assert_eq!(actual, expected);
    }
}

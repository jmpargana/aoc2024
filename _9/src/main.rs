#[derive(Debug, PartialEq, Eq, Clone)]
enum BlockType {
    Free,
    File,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Block {
    kind: BlockType,
    size: u8,
}

fn build_array(input: &str) -> Vec<Block> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, it)| {
            let kind = if i % 2 == 0 {
                BlockType::File
            } else {
                BlockType::Free
            };
            let size = it.to_digit(10).unwrap() as u8;
            Block { kind, size }
        })
        .collect()
}

// Option -> either '.' or file id
fn blocks_to_memory(blocks: Vec<Block>) -> Vec<Option<u128>> {
    let mut index = 0;
    blocks
        .iter()
        .flat_map(|it| {
            let v = if it.kind == BlockType::File {
                Some(index)
            } else {
                None
            };
            if it.kind == BlockType::File {
                index += 1;
            }
            vec![v; it.size.into()]
        })
        .collect()
}

fn sort_blocks(blocks: Vec<Option<u128>>) -> Vec<u128> {
    let mut result = Vec::new();
    let mut left = 0;
    let mut right = blocks.len().saturating_sub(1);

    while left <= right {
        if let Some(v) = blocks[left] {
            result.push(v);
            left += 1;
        } else {
            while right > left && blocks[right].is_none() {
                right = right.saturating_sub(1);
            }
            if let Some(v) = blocks[right] {
                result.push(v);
                right = right.saturating_sub(1);
            }
            left += 1;
        }
    }

    result
}

fn checksum(input: &str) -> u128 {
    let blocks = build_array(input);
    let memory = blocks_to_memory(blocks);
    let memory = sort_blocks(memory);
    memory
        .iter()
        .enumerate()
        .map(|(i, it)| i as u128 * *it as u128)
        .sum::<u128>()
}

// This removes all abtractions and is a close representation of memory
fn parse_part2(input: &str) -> Vec<(usize, isize)> {
    let mut fileid = 0;
    input
        .trim()
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            (
                (b - b'0') as usize,
                if i % 2 == 0 {
                    fileid += 1;
                    fileid - 1
                } else {
                    -1
                },
            )
        })
        .collect()
}

fn efficient_checksum(mut files: Vec<(usize, isize)>) -> usize {
    let mut i = files.len() - 1;
    while i > 0 {
        let (size, id) = files[i];
        if id == -1 {
            i -= 1;
            continue;
        }
        if let Some(j) = files[0..i]
            .iter()
            .position(|&(s, id)| id == -1 && size <= s)
        {
            let s = files[j].0;
            files[j] = (size, id);
            files[i] = (size, -1);
            if size < s {
                files.insert(j + 1, (s - size, -1));
            }
        }
        i -= 1;
    }
    files
        .iter()
        .flat_map(|&(s, id)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
        .sum()
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let checksum = checksum(input);
    let disk = parse_part2(input);
    println!("result 2: {}", efficient_checksum(disk));
    println!("result 1: {}", checksum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_file_in_array() {
        let expected = 1928;
        let given = "2333133121414131402";
        let actual = checksum(given);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_move_only_available_blocks() {
        let expected = 2858;
        let given = "2333133121414131402";
        let given = parse_part2(given);
        let actual = efficient_checksum(given);
        assert_eq!(expected, actual);
    }
}

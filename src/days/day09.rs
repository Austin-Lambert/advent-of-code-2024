use std::fs::File;
use std::io::{self, Read};
use num_bigint::BigInt;

pub fn solve(input: File) {
    let formatted = format_input(input);

    let part1 = solve_part1(&formatted);
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&formatted);
    println!("The answer for part 2 is: {}", part2);
}

fn solve_part1(input: &DiskMap) -> BigInt {
    let mut map = input.clone();
    map.sort_blocks();
    map.checksum()
}

fn solve_part2(input: &DiskMap) -> BigInt {
    let mut map = input.clone();
    map.sort_files();
    map.checksum()
}

fn format_input(input: File) -> DiskMap {
    println!("FORMATTING");
    let mut buf = String::new();
    let mut reader = io::BufReader::new(input);
    reader.read_to_string(&mut buf).unwrap();
    DiskMap::new(&buf.trim().to_string())
}



#[derive(Clone, PartialEq, Eq, Debug)]
struct DiskMap {
    raw: String,
    map: Vec<BlockType>
}

impl DiskMap {
    fn new(raw: &String) -> Self {
        println!("MAPPING");
        let mut map = Vec::new();
        let mut id = 0;
        for i in 0..raw.len() {
            let c = raw.chars().nth(i).unwrap();
            if i % 2 != 0{
                map.extend(BlockType::Free.repeat(c.to_string().parse::<usize>().unwrap()));
            } else {
                map.extend(BlockType::File(id).repeat(c.to_string().parse::<usize>().unwrap()));
                id += 1;
            }
        }
        Self { raw: raw.to_string(), map: map }
    }

    fn sort_blocks(&mut self) {
        println!("SORTING");
        let mut blocks = self.map.clone();
        let mut free = Vec::new();
        while blocks.contains(&BlockType::Free) {
            while blocks.last().unwrap() == &BlockType::Free {
                blocks.pop();
                free.push(BlockType::Free);
            }
            let index = blocks.iter().position(|&b| b == BlockType::Free).unwrap();
            blocks.swap_remove(index);
            free.push(BlockType::Free);
        }
        blocks.extend(free);
        self.map = blocks;
    }

    fn checksum(&self) -> BigInt {
        println!("CHECKSUM");
        let mut checksum = BigInt::from(0);
        for (i, block) in self.map.iter().enumerate() {
            if let BlockType::File(id) = block {
                checksum += BigInt::from(*id) * BigInt::from(i);
            }
        }
        checksum
    }

    fn group_blocks_by_id(&self) -> Vec<AFile> {
        println!("GROUPING");
        let mut files = Vec::new();
        let mut current_file: Option<AFile> = None;
        
        for (i, block) in self.map.iter().enumerate() {
            match (&current_file, block) {
                (None, _) => {
                    current_file = Some(AFile {
                        block_type: *block,
                        blocks: 1,
                        index_of_first_block: i,
                    });
                },
                (Some(file), block) if file.block_type == *block => {
                    current_file = Some(AFile {
                        block_type: file.block_type,
                        blocks: file.blocks + 1,
                        index_of_first_block: file.index_of_first_block,
                    });
                },
                (Some(file), _) => {
                    files.push(*file);
                    current_file = Some(AFile {
                        block_type: *block,
                        blocks: 1,
                        index_of_first_block: i,
                    });
                }
            }
        }
        if let Some(file) = current_file {
            files.push(file);
        }
        
        files
    }

    fn group_files_by_type(&self, files: &Vec<AFile>) -> (Vec<AFile>, Vec<AFile>) {
        let files = files.clone();
        let mut occupied = Vec::new();
        let mut free = Vec::new();
        for file in files {
            if file.block_type == BlockType::Free {
                free.push(file);
            } else {
                occupied.push(file);
            }
        }
        (occupied, free)
    }

    fn sort_files(&mut self) {
        println!("SORTING");
        let (mut occupied, mut free) = self.group_files_by_type(&self.group_blocks_by_id());
        occupied.reverse();
        for i in 0..occupied.len() {
            for j in 0..free.len() {
                if self.try_swap_file_with_free(&occupied[i], &free[j]) {
                    break;
                }
            }
            (_, free) = self.group_files_by_type(&self.group_blocks_by_id());
        }
    }

    fn try_swap_file_with_free(&mut self, file: &AFile, free: &AFile) -> bool {
        if free.blocks < file.blocks || file.index_of_first_block < free.index_of_first_block {
            return false;
        }
        let mut map = self.map.clone();
        for i in 0..file.blocks {
            map.swap(file.index_of_first_block + i, free.index_of_first_block + i);
        }
        self.map = map;
        true
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
enum BlockType {
    File(i32),
    Free,
}

impl BlockType {
    fn repeat(&self, n: usize) -> Vec<BlockType> {
        let mut result = Vec::new();
        for _i in 0..n {
            result.push(self.clone());
        }
        result
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
struct AFile {
    block_type: BlockType,
    blocks: usize,
    index_of_first_block: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn it_will_solve_part1() {
        let input = create_file_input();
        let formatted = format_input(input);
        let part1 = solve_part1(&formatted);
        assert_eq!(part1, BigInt::from(1928));
    }

    #[test]
    fn it_will_solve_part2() {
        let input = create_file_input();
        let formatted = format_input(input);
        let part2 = solve_part2(&formatted);
        assert_eq!(part2, BigInt::from(2858));
    }

    #[test]
    fn it_will_format_the_input() {
        let input = create_file_input();
        let formatted = format_input(input);
        assert_eq!(formatted.raw, "2333133121414131402");
    }

    #[test]
    fn it_will_group_blocks_by_id() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "12345").unwrap();
        let formatted = format_input(temp_file.reopen().unwrap());
        let grouped = formatted.group_blocks_by_id();
        assert_eq!(grouped, vec![
            AFile { block_type: BlockType::File(0), blocks: 1, index_of_first_block: 0 },
            AFile { block_type: BlockType::Free, blocks: 2, index_of_first_block: 1 },
            AFile { block_type: BlockType::File(1), blocks: 3, index_of_first_block: 3 },
            AFile { block_type: BlockType::Free, blocks: 4, index_of_first_block: 6 },
            AFile { block_type: BlockType::File(2), blocks: 5, index_of_first_block: 10 },
        ]);
    }

    #[test]
    fn it_will_sort_files() {
        let files = vec![
            AFile { block_type: BlockType::File(0), blocks: 1, index_of_first_block: 0 },
            AFile { block_type: BlockType::Free, blocks: 2, index_of_first_block: 1 },
            AFile { block_type: BlockType::File(1), blocks: 3, index_of_first_block: 3 },
            AFile { block_type: BlockType::File(2), blocks: 2, index_of_first_block: 6 },
        ];
        let mut formatted = DiskMap { raw: "".to_string(), map: vec![] };
        formatted.ungroup_blocks_into_map(&files);
        formatted.sort_files();
        assert_eq!(formatted.map, vec![BlockType::File(0), BlockType::File(2), BlockType::File(2), BlockType::File(1), BlockType::File(1), BlockType::File(1), BlockType::Free, BlockType::Free]);
    }

    #[test]
    fn it_will_ungroup_blocks_into_map() {
        let files = vec![
            AFile { block_type: BlockType::File(0), blocks: 1, index_of_first_block: 0 },
            AFile { block_type: BlockType::Free, blocks: 2, index_of_first_block: 1 },
            AFile { block_type: BlockType::File(1), blocks: 3, index_of_first_block: 3 },
        ];
        let mut formatted = DiskMap { raw: "".to_string(), map: vec![] };
        formatted.ungroup_blocks_into_map(&files);
        assert_eq!(formatted.map, vec![BlockType::File(0), BlockType::Free, BlockType::Free, BlockType::File(1), BlockType::File(1), BlockType::File(1)]);
    }

    fn create_file_input() -> File {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "2333133121414131402").unwrap();
        temp_file.reopen().unwrap()
    }
}
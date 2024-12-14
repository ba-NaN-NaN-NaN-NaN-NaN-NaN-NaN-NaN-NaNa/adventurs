use lazy_static::lazy_static;
use std::str::FromStr;
use regex::Regex;

use std::{collections::HashSet, hash::Hash};
use std::collections::{HashMap, VecDeque};
use crate::{grids::basic2d::{self, Grid2d, GridDirs8}, input};

#[allow(dead_code)]
pub fn blockdevice_into_repr(blockdevice: &Vec<Option<usize>>) -> String {
    let chars: Vec<String> = blockdevice.iter().map(|b| {
        match b {
            Some(block_id) => {format!("{}", block_id)}
            None => ".".to_string()
        }
    }).collect();
    let formatted: String = chars.join("");
    // println!("blockdevice_into_repr({:?}) -> '{}'", blockdevice, formatted);
    formatted
}



/*
#[allow(dead_code)]
pub fn checksum_XXX(blockdevice: &Vec<Option<usize>>) -> usize {
    let mut first_encounters: HashMap<usize, usize> = HashMap::new();

    for idx in 0..blockdevice.len() {
        match blockdevice[idx] {
            None => {},
            Some(block_id) => {
                if !first_encounters.contains_key(&block_id) {
                    first_encounters.insert(block_id, idx);
                }
            }
        }
    }

    let mut checksum: usize = 0;

    

    for (block_id, idx) in first_encounters.iter() {
        let partial = block_id * idx;
        println!("block_id * idx = partial_checksum -> {} * {} = {}", block_id, idx, partial);
        checksum += partial;
    }
    println!("first_encounters is {:?}. Results in checksum {}.", first_encounters, checksum);
    checksum
}
 */
#[allow(dead_code)]
pub fn defrag(blockdevice: &Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut toreturn = blockdevice.clone();

    let mut idx_left: usize = 0;
    let mut idx_right: usize = toreturn.len()-1;

    while idx_right > idx_left {
        // Place indexes.
        while toreturn[idx_right] == None {
            idx_right -= 1;
        }

        while idx_left < idx_right && toreturn[idx_left] != None {
            idx_left += 1;
        }

        if idx_left < idx_right && 
            toreturn[idx_left] == None &&
            toreturn[idx_right] != None {

            toreturn[idx_left] = toreturn[idx_right];
            toreturn[idx_right] = None;
        }
    }
    toreturn

}

#[allow(dead_code)]
pub fn move_attempt_single_file(blockdevice: &mut Vec<Option<usize>>, src: usize, dst: usize) -> bool {
    if dst > 0 && blockdevice[dst-1] == None {
        panic!("Why am I not moving to start of empty sector? Found more space to left at position {}", dst)
    }

    if blockdevice[dst] != None {
        panic!("fgjdh")
    }

    if blockdevice[src] == None {
        panic!("fgjdh")
    }

    let mut space_available : usize = 0;
    while blockdevice[dst + space_available] == None {
        space_available += 1
    }

    let mut src_first_idx: usize = src;
    let mut src_last_idx: usize = src;
    while src_last_idx < blockdevice.len()-1 && blockdevice[src_last_idx+1] == blockdevice[src] {
        src_last_idx += 1
    }

    while src_first_idx > 0 && blockdevice[src_first_idx-1] == blockdevice[src] {
        src_first_idx -= 1
    }

    let src_len = src_last_idx-src_first_idx+1;
    println!("Determined that file with id {} has len {}. {}-{}", blockdevice[src].unwrap(), src_len, src_last_idx, src_first_idx);

    let file_id = blockdevice[src].unwrap();
    if space_available >= src_len {
        for n in 0..src_len {
            blockdevice[src_first_idx + n] = None;
            blockdevice[dst+n] = Some(file_id);
        }

        println!("Could move '{}' from position {} to {}, block device is now {}", file_id, src, dst, blockdevice_into_repr(&blockdevice));
        return true
    } else {
        println!("Could NOT move '{}' from position {} to {}. Block device unchanged as {}", file_id, src, dst, blockdevice_into_repr(&blockdevice));
        return false
    }

    // panic!("fjkldhg")

}




#[allow(dead_code)]
pub fn sparse_into_block(sparse_rep: &String) -> Vec<Option<usize>> {
    let mut toreturn: Vec<Option<usize>> = Vec::new();

    let mut current_block_id: usize = 0;
    let mut rep_as_ints: VecDeque<usize> = VecDeque::new();
    for ch in sparse_rep.chars() {
        let single_formatted: String = format!("{}", ch);
        rep_as_ints.push_back(usize::from_str(&single_formatted).unwrap());
    }

    // println!("rep_as_ints parse gave {} -> {:?}", sparse_rep, rep_as_ints);
    let initial_block_size = rep_as_ints.pop_front().unwrap();
    for _ in 0..initial_block_size {
        toreturn.push(Some(current_block_id));
    }

    while rep_as_ints.len() > 0 {
        let blocks_to_skip = rep_as_ints.pop_front().unwrap();
        let blocklen_next_block = rep_as_ints.pop_front().unwrap();

        for _ in 0..blocks_to_skip {
            toreturn.push(None);
        }
        current_block_id += 1;

        for _ in 0..blocklen_next_block {
            toreturn.push(Some(current_block_id));
        }
    }

    toreturn
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    start_idx: usize,
    len: usize,
}

#[allow(dead_code)]
pub fn sparse_into_files(sparse_rep: &String) -> Vec<File> {
    let mut toreturn: Vec<File> = Vec::new();

    let mut current_block_id: usize = 0;
    let mut current_position: usize = 0;
    let mut rep_as_ints: VecDeque<usize> = VecDeque::new();
    for ch in sparse_rep.chars() {
        let single_formatted: String = format!("{}", ch);
        rep_as_ints.push_back(usize::from_str(&single_formatted).unwrap());
    }

    // println!("rep_as_ints parse gave {} -> {:?}", sparse_rep, rep_as_ints);
    let initial_block_size = rep_as_ints.pop_front().unwrap();
    for _ in 0..initial_block_size {
        toreturn.push(
            File { 
                id: current_block_id, 
                start_idx:0,
                len: initial_block_size 
            }
        );
    }
    current_position += initial_block_size;

    while rep_as_ints.len() > 0 {
        // Sparse representation, number of blocks to skip.
        let empty_space_size = rep_as_ints.pop_front().unwrap();
        current_position += empty_space_size;

        let blocklen_next_block = rep_as_ints.pop_front().unwrap();
        current_block_id += 1;
        toreturn.push(
            File { 
                id: current_block_id, 
                start_idx:current_position,
                len: blocklen_next_block
            }
        );
        current_position += blocklen_next_block;
    }

    toreturn
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct Day9 {
    disk_blocks: Vec<Option<usize>>,
    sparse_files: Vec<File>
}

impl Day9 {
    pub fn from_lines(content: &str) -> Day9 {
        let sparse_rep = content.trim().to_string();
        let initial_state = sparse_into_block(&sparse_rep);
        let sparse_files = sparse_into_files(&sparse_rep);

        Day9 {
            disk_blocks: initial_state,
            sparse_files,
        }
    }

    pub fn part1(&mut self) -> usize {        
        self.disk_blocks = defrag(&self.disk_blocks);
        let p1 = self.checksum();
        p1
    }

    pub fn part2(&mut self) -> i64 {
        // self.defrag_full_files();

        println!("Doing part 2 on disk with sparse files: {:?}", self.sparse_files);
        self.defrag_full_files();
        let p2 = self.checksum();
        // println!("Defragged disk {:?} gave checksum {}", defragged, p1);
        p2 as i64
    }

    #[allow(dead_code)]
    pub fn defrag_full_files(&mut self) {
        let mut worklist = self.sparse_files.clone();
        while worklist.len() > 0 {
            let file = worklist.pop().unwrap();
            println!("defrag_full_files() will attempt to move file {:?}", file);
            let possible_dst = self.first_space_of_size(file.len);
            
            if let Some(dst) = possible_dst {
                if dst < file.start_idx {
                    for offset in 0..file.len {
                        self.disk_blocks[dst+offset] = Some(file.id);
                        self.disk_blocks[file.start_idx+offset] = None;
                    }
                }
            }
        }

        println!("After defrag_full_files, block device looks like: {}", blockdevice_into_repr(&self.disk_blocks));
    }

    #[allow(dead_code)]
    pub fn checksum(&self) -> usize {
        let mut toreturn = 0;
        for idx in 0..self.disk_blocks.len() {
            match self.disk_blocks[idx] {
                None => {},
                Some(block_id) => {
                    toreturn += block_id*idx
                }
            }
        }
        toreturn
    }


    #[allow(dead_code)]
    pub fn space_is_available(&self, idx :usize, len :usize) -> bool {
        for offset in 0..len {
            if idx+offset >= self.disk_blocks.len() {
                return false
            }
    
            if self.disk_blocks[idx+offset] != None {
                return false
            }
        }
        return true
    }
    
    #[allow(dead_code)]
    pub fn first_space_of_size(&self, len: usize) -> Option<usize> {
        let mut n: usize = 0;
        while n < self.disk_blocks.len() {
            while self.disk_blocks[n] != None { 
                n+= 1
            }

            if self.space_is_available(n, len) {
                return Some(n)
            } else {
                n += 1
            }
        }
    
    
        return None
    }
    
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut d9 = Day9::from_lines(input);    
    let toreturn = d9.part1();
    // d9.show();
    // println!("{:?}", d9.found);
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut d9 = Day9::from_lines(input);    
    let toreturn = d9.part2();
    // d9.show();
    // println!("{:?}", d9.found);
    toreturn
}

#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2024::d9::tests --nocapture

    use super::*;
    use crate::input;

    
    #[test]
    fn test_parse() {
        assert_eq!(3*9, sparse_into_block(&"90909".to_owned()).len());
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", blockdevice_into_repr(&sparse_into_block(&"2333133121414131402".to_string())));
    }

    #[test]
    fn test_defrag() {
        let initial_state = sparse_into_block(&"2333133121414131402".to_string());
        let defragged = defrag(&initial_state);
        let defragged_formatted = blockdevice_into_repr(&defragged);
        let expected = "0099811188827773336446555566..............";
        assert_eq!(expected, defragged_formatted);
    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2024::d9::tests --nocapture
        if false {
            let pbuf = input::get_input("2024_d9_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 1928); // p1 sample
        }
        if false {
            let pbuf = input::get_input("2024_d9.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 6461289671426); // p1 skarp
        }
        {
            let pbuf = input::get_input("2024_d9_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 2858); // p2 sample
        }
        {
            let pbuf = input::get_input("2024_d9.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 6488291456470); // p2 skarp
        }
    }
}

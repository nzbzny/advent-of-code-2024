use std::collections::VecDeque;

use crate::utils;

#[derive(Debug)]
struct Block {
    idx: usize,
    len: usize,
}

fn build_disk_map(line: &str) -> (Vec<i64>, VecDeque<Block>, VecDeque<Block>) {
    let mut id = 0;
    let mut disk_map: Vec<i64> = vec![];
    let mut free_space: VecDeque<Block> = VecDeque::new();
    let mut file_blocks: VecDeque<Block> = VecDeque::new();

    for (idx, c) in line.chars().enumerate() {
        let length = c.to_digit(10).unwrap();
        let val: i64;
        if idx % 2 == 0 {
            val = id;

            id += 1;
            file_blocks.push_back(Block { idx: disk_map.len(), len: length as usize });
        } else {
            val = -1;

            free_space.push_back(Block { idx: disk_map.len(), len: length as usize });
        }

        for _ in 0..length {
            disk_map.push(val);
        }
    }

    (disk_map, free_space, file_blocks)
}

fn move_to_free_space_part_1(disk_map: &mut [i64]) {
    let mut start = disk_map.iter().enumerate().find(|(_, val)| **val == -1).unwrap().0;
    let mut end = disk_map.iter().enumerate().rfind(|(_, val)| **val != -1).unwrap().0;

    while start <= end {
        disk_map[start] = disk_map[end];
        disk_map[end] = -1;

        while start < disk_map.len() && disk_map[start] != -1 {
            start += 1;
        }
        while end > 0 && disk_map[end] == -1 {
            end -= 1;
        }
    }
}

fn move_to_free_space_part_2(disk_map: &mut [i64], mut free_space: VecDeque<Block>, mut file_blocks: VecDeque<Block>) {
    loop {
        let block = match file_blocks.pop_back() {
            None => {
                println!("no more file blocks");
                return;
            }
            Some(b) => b
        };

        if let Some(space) = free_space.iter_mut().find(|space| space.len >= block.len) {
            if space.idx >= block.idx {
                return;
            }

            for i in 0..block.len {
                disk_map[i + space.idx] = disk_map[i + block.idx];
                disk_map[i + block.idx] = -1;
            }

            space.len -= block.len;
            space.idx += block.len;
        } 

        if let Some(space) = free_space.front() {
            if space.len == 0 {
                free_space.pop_front();
            }
        }
    }
}

fn calculate_checksum(disk_map: &[i64]) -> i64 {
    disk_map.iter().enumerate().fold(0, |acc, (idx, val)| {
        if *val == -1 {
            acc
        } else {
            acc + (idx as i64 * val)
        }
    })
}

pub fn run() {
    // let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_9");

    // part 1
    let (mut disk_map_part_1, _, _) = build_disk_map(&lines[0]);

    move_to_free_space_part_1(&mut disk_map_part_1);

    let checksum_part_1 = calculate_checksum(&disk_map_part_1);

    println!("Part 1 checksum is: {checksum_part_1}");

    // part 2
    let (mut disk_map_part_2, free_space, file_blocks) = build_disk_map(&lines[0]);

    // println!("{:?}", disk_map_part_2);
    // println!("{:?}", free_space);
    // println!("{:?}", file_blocks);
    move_to_free_space_part_2(&mut disk_map_part_2, free_space, file_blocks);
    // println!("{:?}", disk_map_part_2);

    let checksum_part_2 = calculate_checksum(&disk_map_part_2);

    println!("Part 2 checksum is: {checksum_part_2}");
}

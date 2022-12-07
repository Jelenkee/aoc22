use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(7);
    let mut current_folder: Vec<String> = vec![];
    let mut size_map = HashMap::new();
    let mut ls = false;
    for line in lines {
        if line.starts_with('$') {
            let command = line.trim_start_matches("$ ").trim();
            let args = command.split_whitespace().collect::<Vec<&str>>();
            match args[0] {
                "cd" => {
                    ls = false;
                    match args[1] {
                        "/" => {
                            current_folder.clear();
                        }
                        ".." => {
                            current_folder.pop();
                        }
                        file => {
                            current_folder.push(file.to_string());
                        }
                    }
                }
                "ls" => {
                    ls = true;
                }
                _ => {
                    panic!("invalid command")
                }
            }
        } else {
            assert!(ls);
            if !line.starts_with("dir") {
                let file_size = line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let mut folder = current_folder.clone();
                size_map
                    .entry(folder.clone())
                    .and_modify(|v| *v += file_size)
                    .or_insert(file_size);
                while !folder.is_empty() {
                    folder.pop();
                    size_map
                        .entry(folder.clone())
                        .and_modify(|v| *v += file_size)
                        .or_insert(file_size);
                }
            }
        }
    }
    let free_space = 70_000_000u64 - *size_map.get(&vec![]).unwrap();
    let missing_space = 30_000_000u64 - free_space;

    let mut valid_values = size_map
        .values()
        .filter(|&&v| v >= missing_space)
        .copied()
        .collect::<Vec<u64>>();
    valid_values.sort();

    let sol1: u64 = size_map.values().filter(|&&v| v <= 100000u64).sum();
    let sol2: u64 = *valid_values.first().unwrap();

    (Solution::U64(sol1), Solution::U64(sol2))
}

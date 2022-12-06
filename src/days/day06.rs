use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use std::collections::{HashSet, LinkedList};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(6);
    let buffer = lines.first().unwrap();
    let index = decode(buffer, 4);
    let index2 = decode(buffer, 14);
    let sol1: u64 = index;
    let sol2: u64 = index2;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn decode(buffer: &str, window_size: usize) -> u64 {
    let mut index = 0u64;
    let mut window = LinkedList::new();
    for (i, char) in buffer.chars().enumerate() {
        window.push_back(char);
        if window.len() == window_size {
            if window.iter().collect::<HashSet<&char>>().len() == window.len() {
                index = 1 + i as u64;
                break;
            }
            window.pop_front();
        }
    }
    index
}

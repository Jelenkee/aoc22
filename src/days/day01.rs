use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = get_file_lines(1);
    let mut groups: Vec<u64> = vec![];
    let mut current_group: u64 = 0;
    for line in lines {
        if line.is_empty() {
            groups.push(current_group);
            current_group = 0;
        } else {
            current_group += line.parse::<u64>().unwrap();
        }
    }
    groups.sort();
    groups.reverse();
    let sol1: u64 = *groups.first().unwrap();
    let sol2: u64 = groups.iter().take(3).sum();

    (Solution::U64(sol1), Solution::U64(sol2))
}

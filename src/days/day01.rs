use crate::days::getFileLines;
use crate::{Solution, SolutionPair};
use std::cmp::max;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = getFileLines(1);
    let mut groups: Vec<u64> = vec![];
    let mut currentGroup: u64 = 0;
    for line in lines {
        if line.is_empty() {
            groups.push(currentGroup);
            currentGroup = 0;
        } else {
            currentGroup += line.parse::<u64>().unwrap();
        }
    }
    groups.sort();
    groups.reverse();
    let sol1: u64 = *groups.first().unwrap();
    let sol2: u64 = groups.iter().take(3).sum();

    (Solution::U64(sol1), Solution::U64(sol2))
}

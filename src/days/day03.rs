use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = get_file_lines(3);
    let mut sum1 = 0u64;
    let mut sum2 = 0u64;
    for line in &lines {
        sum1 += get_prio_for_char(get_common_letter(line));
    }
    for chunk in lines.chunks(3) {
        sum2 += get_prio_for_char(get_common_letter2(vec![&chunk[0], &chunk[1], &chunk[2]]))
    }
    let sol1: u64 = sum1;
    let sol2: u64 = sum2;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn get_common_letter(ruck: &str) -> char {
    let c1 = ruck.get(0..(ruck.len() / 2)).unwrap();
    let c2 = ruck.get((ruck.len() / 2)..ruck.len()).unwrap();
    let mut set1 = c1.chars().collect::<HashSet<char>>();
    let set2 = c2.chars().collect::<HashSet<char>>();
    for ch in set2 {
        if !set1.insert(ch) {
            return ch;
        }
    }
    panic!("no common letter");
}

fn get_common_letter2(rucks: Vec<&str>) -> char {
    let all_chars = rucks
        .iter()
        .flat_map(|r| r.chars())
        .collect::<HashSet<char>>();

    for ch in all_chars {
        if rucks.iter().all(|r| r.contains(ch)) {
            return ch;
        }
    }

    panic!("no common letter");
}

fn get_prio_for_char(ch: char) -> u64 {
    if ch.is_ascii_lowercase() {
        (ch as u64) - 96
    } else {
        (ch as u64) - 38
    }
}

use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(9);
    let mut head = (0, 0);
    let mut tail = head;
    let mut tail_positions = HashSet::new();
    tail_positions.insert(format!("{:?}", tail));
    for line in lines {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap().chars().next().unwrap();
        let steps = split.next().unwrap().parse::<u32>().unwrap();
        for _ in 0..steps {
            match direction {
                'U' => {
                    head.1 += 1;
                    if !is_attached(head, tail) {
                        tail = (head.0, head.1 - 1);
                        tail_positions.insert(format!("{:?}", tail));
                    }
                }
                'R' => {
                    head.0 += 1;
                    if !is_attached(head, tail) {
                        tail = (head.0 - 1, head.1);
                        tail_positions.insert(format!("{:?}", tail));
                    }
                }
                'D' => {
                    head.1 -= 1;
                    if !is_attached(head, tail) {
                        tail = (head.0, head.1 + 1);
                        tail_positions.insert(format!("{:?}", tail));
                    }
                }
                'L' => {
                    head.0 -= 1;
                    if !is_attached(head, tail) {
                        tail = (head.0 + 1, head.1);
                        tail_positions.insert(format!("{:?}", tail));
                    }
                }
                _ => {
                    panic!("invalid direction")
                }
            }
        }
    }
    let sol1: i32 = tail_positions.len() as i32;
    let sol2: u64 = 0;

    (Solution::I32(sol1), Solution::U64(sol2))
}

fn is_attached(p1: (i32, i32), p2: (i32, i32)) -> bool {
    distance_sq(p1, p2) <= 2
}

fn distance_sq(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)
}

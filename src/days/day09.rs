use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////
const SNAKE_LENGTH: usize = 10;

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(9);
    let mut knots = vec![];
    for _ in 0..SNAKE_LENGTH {
        knots.push((0, 0));
    }
    assert_eq!(SNAKE_LENGTH, knots.len());
    let mut tail_positions = HashSet::new();
    tail_positions.insert(knots.first().copied().unwrap());
    for line in lines {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap().chars().next().unwrap();
        let steps = split.next().unwrap().parse::<u32>().unwrap();
        for _ in 0..steps {
            let knot_len = knots.len();
            for i in 0..knot_len {
                let previous_knot_o = if i == 0 {
                    None
                } else {
                    knots.get(i - 1).copied()
                };
                let mut current_knot = knots.get_mut(i).unwrap();
                if previous_knot_o.is_none() {
                    match direction {
                        'U' => (*current_knot).1 += 1,
                        'R' => (*current_knot).0 += 1,
                        'D' => (*current_knot).1 -= 1,
                        'L' => (*current_knot).0 -= 1,
                        _ => panic!("invalid direction"),
                    }
                } else {
                    let previous_knot = previous_knot_o.unwrap();
                    if !is_attached(*current_knot, previous_knot) {
                        if current_knot.0 != previous_knot.0 && current_knot.1 != previous_knot.1 {
                            let x = (current_knot.0 - previous_knot.0).abs() == 1;
                            if x {
                                (*current_knot).0 = previous_knot.0;
                            } else {
                                (*current_knot).1 = previous_knot.1;
                            }
                        }
                        let same_x = current_knot.0 == previous_knot.0;
                        if same_x {
                            if current_knot.1 > previous_knot.1 {
                                current_knot.1 = previous_knot.1 + 1;
                            } else {
                                current_knot.1 = previous_knot.1 - 1;
                            }
                        } else {
                            if current_knot.0 > previous_knot.0 {
                                current_knot.0 = previous_knot.0 + 1;
                            } else {
                                current_knot.0 = previous_knot.0 - 1;
                            }
                        }
                        assert!(is_attached(*current_knot, previous_knot));
                    }
                }
            }
            tail_positions.insert(knots.last().copied().unwrap());
        }
    }
    let sol1: i32 = tail_positions.len() as i32;
    let sol2: u64 = 0; // 2326 < x < 2341
                       // 2327 was the correct solution and I don't know why

    (Solution::I32(sol1), Solution::U64(sol2))
}

fn is_attached(p1: (i32, i32), p2: (i32, i32)) -> bool {
    distance_sq(p1, p2) <= 2
}

fn distance_sq(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)
}

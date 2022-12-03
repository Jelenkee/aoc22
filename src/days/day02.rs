use crate::{Solution, SolutionPair};

use crate::days::get_file_lines;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = get_file_lines(2);
    let mut score1 = 0u64;
    let mut score2 = 0u64;
    for line in lines {
        let pair = line.trim().split(' ').collect::<Vec<&str>>();
        score1 += get_score1(pair[1], pair[0]);
        score2 += get_score2(pair[1], pair[0]);
    }
    // Your solution here...
    let sol1: u64 = score1;
    let sol2: u64 = score2;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn get_score1(my: &str, op: &str) -> u64 {
    let mut r = 0u64;
    if (my == "X" && op == "C") || (my == "Y" && op == "A") || (my == "Z" && op == "B") {
        r += 6;
    } else if (my == "X" && op == "A") || (my == "Y" && op == "B") || (my == "Z" && op == "C") {
        r += 3;
    }

    r + match my {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("invalid input"),
    }
}
fn get_score2(win: &str, op: &str) -> u64 {
    let my;
    if (win == "Z" && op == "A") || (win == "Y" && op == "B") || (win == "X" && op == "C") {
        my = "Y";
    } else if (win == "X" && op == "A") || (win == "Y" && op == "C") || (win == "Z" && op == "B") {
        my = "Z";
    } else {
        my = "X";
    }
    get_score1(my, op)
}

use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut lines = get_file_lines(5);
    let mut stack_lines = vec![];
    let mut move_lines = vec![];
    let mut stack_numbers = String::new();
    for line in &lines {
        stack_lines.push(line.to_string());
        if line.is_empty() {
            stack_lines.pop();
            stack_numbers = stack_lines.pop().unwrap();
            break;
        }
    }
    for line in &lines {
        if line.starts_with("move ") {
            move_lines.push(line.to_string());
        }
    }

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..stack_numbers.chars().filter(|c| c.is_numeric()).count() {
        stacks.push(vec![]);
    }
    for stack_line in stack_lines.iter().rev() {
        for (i, num_char) in stack_numbers.chars().enumerate() {
            if let Some(num) = num_char.to_digit(10) {
                if let Some(car) = stack_line.chars().nth(i) {
                    if !car.is_ascii_whitespace() {
                        let stack = stacks.get_mut((num - 1) as usize).unwrap();
                        stack.push(car);
                    }
                }
            }
        }
    }
    let mut stacks2 = stacks.clone();
    move_items(&mut stacks, &move_lines);
    move_items2(&mut stacks2, &move_lines);

    let sol1: String = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    let sol2: String = stacks2
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();

    (Solution::Str(sol1), Solution::Str(sol2))
}

fn move_items(stacks: &mut Vec<Vec<char>>, move_lines: &Vec<String>) {
    for line in move_lines {
        let (num, from, to) = parse_move(line);
        for _ in 0..num {
            let popped_item = stacks.get_mut(from - 1).unwrap().pop().unwrap();
            let to_stack = stacks.get_mut(to - 1).unwrap();
            to_stack.push(popped_item);
        }
    }
}
fn move_items2(stacks: &mut Vec<Vec<char>>, move_lines: &Vec<String>) {
    for line in move_lines {
        let (num, from, to) = parse_move(line);
        let mut popped_items = vec![];
        for _ in 0..num {
            let from_stack = stacks.get_mut(from - 1).unwrap();
            popped_items.push(from_stack.pop().unwrap());
        }
        popped_items.reverse();
        for item in popped_items {
            let to_stack = stacks.get_mut(to - 1).unwrap();
            to_stack.push(item);
        }
    }
}
fn parse_move(line: &str) -> (usize, usize, usize) {
    let vec = line
        .split_whitespace()
        .map(|c| c.parse::<usize>())
        .filter(|o| o.is_ok())
        .map(|o| o.unwrap() as usize)
        .collect::<Vec<usize>>();
    assert_eq!(3, vec.len());
    (vec[0], vec[1], vec[2])
}

use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, VecDeque};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(11);
    let monkey_templates = lines
        .split(|s| s.is_empty())
        .map(|s| s.iter().map(|ss| ss.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut monkey_map = monkey_templates
        .iter()
        .map(|t| Monkey::new(t))
        .map(|m| (m.id, RefCell::new(m)))
        .collect::<BTreeMap<u8, RefCell<Monkey>>>();
    let mut inspect_map = HashMap::new();
    let r#mod = monkey_map
        .values()
        .map(|m| m.borrow().modulo)
        .product::<u64>();
    dbg!(&r#mod);
    for i in 0..10_000 {
        for monkey_cell in monkey_map.values() {
            let mut monkey = monkey_cell.borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                inspect_map
                    .entry(monkey.id)
                    .and_modify(|v| *v += 1)
                    .or_insert(1u64);
                let item = ((monkey.update)(item)) % r#mod;
                //let item = ((item as f64) / 3.0).floor() as u32;
                let item = item.clone();
                let new_monkey_id = if item % monkey.modulo == 0 {
                    monkey.yes
                } else {
                    monkey.no
                };
                let mut next_monkey = monkey_map.get(&new_monkey_id).unwrap().borrow_mut();
                next_monkey.items.push_back(item);
            }
        }
    }
    let mut sums = inspect_map.values().map(|v| *v).collect::<Vec<u64>>();
    sums.sort();
    sums.reverse();
    let sol1: u64 = sums.iter().take(2).product();
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

struct Monkey {
    id: u8,
    items: VecDeque<u64>,
    update: Box<dyn Fn(u64) -> u64>,
    modulo: u64,
    yes: u8,
    no: u8,
}

impl Monkey {
    fn new(input: &Vec<String>) -> Self {
        assert_eq!(6, input.len());
        let id = input
            .get(0)
            .unwrap()
            .replace("Monkey", "")
            .replace(":", "")
            .trim()
            .parse::<u8>()
            .unwrap();
        let items = input
            .get(1)
            .unwrap()
            .replace("Starting items:", "")
            .split(",")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        let op_parts = input
            .get(2)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let op_op = op_parts
            .get(op_parts.len() - 2)
            .map(|s| s.to_string())
            .unwrap();
        let op_num = op_parts
            .get(op_parts.len() - 1)
            .map(|s| s.to_string())
            .unwrap();
        let update = move |old: u64| {
            let num = op_num.parse::<u64>().unwrap_or(old);
            match op_op.as_str() {
                "*" => old * num,
                "+" => old + num,
                _ => {
                    panic!("invalid operator")
                }
            }
        };
        let modulo = input
            .get(3)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let yes = input
            .get(4)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let no = input
            .get(5)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        Monkey {
            id,
            items,
            update: Box::new(update),
            modulo,
            yes,
            no,
        }
    }
}

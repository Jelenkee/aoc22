use crate::days::day13::Entity::{List, Num};
use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use regex::Regex;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(13);
    let pairs = lines
        .split(|s| s.is_empty())
        .map(|s| s.iter().map(|ss| ss.to_string()).collect::<Vec<String>>())
        .map(|v| {
            (
                Entity::from(v.get(0).unwrap()),
                Entity::from(v.get(1).unwrap()),
            )
        })
        .collect::<Vec<(Entity, Entity)>>();
    let mut sum = 0u64;
    for (i, pair) in pairs.iter().enumerate() {
        if matches!(compare(&pair.0, &pair.1), Less) {
            sum += (i + 1) as u64;
        }
    }
    let mut product = 1u64;
    let mut packets = vec![];
    for pair in pairs {
        packets.push(pair.0);
        packets.push(pair.1);
    }
    packets.push(Entity::from("[[2]]"));
    packets.push(Entity::from("[[6]]"));
    packets.sort();
    for (i, p) in packets.iter().enumerate() {
        match p {
            Num(_) => {}
            List(v) => {
                if v.len() == 1 {
                    let ee = v.get(0).unwrap();
                    if matches!(*ee, List(_)) {
                        let vv = ee.get_list();
                        if vv.len() == 1 {
                            let eee = vv.get(0).unwrap();
                            if matches!(*eee, Num(_)) {
                                if eee.get_num() == 2 || eee.get_num() == 6 {
                                    product *= (i + 1) as u64;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let sol1: u64 = sum;
    let sol2: u64 = product;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn compare(a: &Entity, b: &Entity) -> Ordering {
    if matches!(*a, Num(_)) && matches!(*b, Num(_)) {
        a.get_num().cmp(&b.get_num())
    } else if matches!(*a, List(_)) && matches!(*b, List(_)) {
        let mut i = 0;
        let l1 = a.get_list();
        let l2 = b.get_list();
        loop {
            let e1 = l1.get(i);
            let e2 = l2.get(i);
            i += 1;
            if e1.is_some() && e2.is_some() {
                match compare(e1.unwrap(), e2.unwrap()) {
                    Less => return Less,
                    Equal => {}
                    Greater => return Greater,
                }
            } else if e1.is_none() && e2.is_none() {
                return Equal;
            } else if e2.is_some() {
                return Less;
            } else {
                return Greater;
            }
        }
    } else if matches!(*a, List(_)) && matches!(*b, Num(_)) {
        compare(a, &List(vec![Num(b.get_num())]))
    } else if matches!(*a, Num(_)) && matches!(*b, List(_)) {
        compare(&List(vec![Num(a.get_num())]), b)
    } else {
        panic!("invalid state")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Entity {
    Num(u8),
    List(Vec<Entity>),
}

impl Entity {
    fn from(input: &str) -> Self {
        let num = input.parse::<u8>();
        if num.is_ok() {
            return Num(num.unwrap());
        }
        assert!(input.starts_with('['));
        assert!(input.ends_with(']'));
        let mut deep = 0;

        if input.matches('[').count() == 1 && input.matches(']').count() == 1 {
            let sub = &input[1..input.len() - 1];
            let reg = Regex::new("^(\\d|,)*$").unwrap();
            if sub.is_empty() {
                return List(vec![]);
            }
            if reg.is_match(sub) {
                return List(
                    sub.split(',')
                        .map(|s| s.parse::<u8>().unwrap())
                        .map(|n| Num(n))
                        .collect(),
                );
            }
        }
        let mut comma_indices = vec![0];
        for (i, c) in input.chars().enumerate() {
            if c == '[' {
                deep += 1;
            } else if c == ']' {
                deep -= 1;
            } else {
                if deep == 1 && c == ',' {
                    comma_indices.push(i);
                }
            }
        }
        let mut v = vec![];
        let mut end = input.len() - 1;
        for ci in comma_indices.iter().copied().rev() {
            v.push(&input[(ci + 1)..end]);
            end = ci;
        }

        List(
            v.iter()
                .rev()
                .map(|s| Entity::from(s))
                .collect::<Vec<Entity>>(),
        )
    }

    fn get_num(&self) -> u8 {
        match *self {
            Num(v) => v,
            List(_) => {
                panic!()
            }
        }
    }

    fn get_list(&self) -> &Vec<Entity> {
        match self {
            Num(_) => {
                panic!()
            }
            List(v) => v,
        }
    }
}

impl PartialOrd<Self> for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(&self, other)
    }
}

use crate::days::{get_file_lines, Grid};
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet, VecDeque};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(12);
    let width = lines.first().unwrap().len();
    let height = lines.len();
    let mut grid: Grid<char> = Grid::new(width, height);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, mut c) in line.chars().enumerate() {
            if c == 'S' {
                c = 'a';
                start = (x, y);
            } else if c == 'E' {
                c = 'z';
                end = (x, y);
            }
            grid.set(x, y, c);
        }
    }

    let all_nodes = {
        let mut tmp = vec![];
        for x in 0..grid.width {
            for y in 0..grid.height {
                tmp.push((x, y));
            }
        }
        tmp
    };

    let all_edges = {
        let mut tmp = vec![];
        for n in &all_nodes {
            for nn in valid_neighbors(&grid, n) {
                tmp.push((*n, nn));
            }
        }
        tmp
    };

    dbg!(all_nodes.len());
    dbg!(all_edges.len());

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut d = HashMap::new();
    visited.insert(start);
    q.push_back(start);
    d.insert(start, 0);
    let mut min_d = usize::MAX - (u8::MAX as usize);
    while let Some(node) = q.pop_front() {
        let dist = *d.get(&node).unwrap();
        if node == end {
            min_d = dist;
            break;
        }

        for nei in valid_neighbors(&grid, &node) {
            if !visited.contains(&nei) {
                visited.insert(nei);
                q.push_back(nei);
                d.insert(nei, dist + 1);
            }
        }
    }

    dbg!(min_d);

    let mut path = VecDeque::new();
    path.push_back(start.clone());

    let mut distance_map = HashMap::new();
    for node in &all_nodes {
        distance_map.insert(node.clone(), usize::MAX);
    }
    distance_map.insert(start, 0);

    /*let mut ancestor_map = HashMap::new();
    let mut unvisited_nodes = all_nodes.iter().collect::<HashSet<&(usize, usize)>>();
    while !unvisited_nodes.is_empty() {
        let u = unvisited_nodes
            .iter()
            .min_by(|a, b| {
                distance_map
                    .get(a)
                    .unwrap()
                    .cmp(distance_map.get(b).unwrap())
            })
            .copied()
            .copied()
            .unwrap();
        unvisited_nodes.remove(&u);
        for v in valid_neighbors(&grid, &u) {
            let bool = v == end;
            if bool {
                dbg!(&u);
                dbg!(grid.get((&u).0,(&u).1));
            }
            if unvisited_nodes.contains(&v) {
                //dbg!(distance_map.get(&u).unwrap());
                //dbg!(&u);
                let d = distance_map.get(&u).unwrap();
                let dist = if usize::MAX == *d{ *d} else {*d +1};
                if bool {
                    dbg!(&dist);
                    dbg!(distance_map.get(&v).unwrap());
                }
                if dist < *distance_map.get(&v).unwrap() {
                    distance_map.insert(v, dist);
                    ancestor_map.insert(v, u);
                    //dbg!(u);
                }
            }
        }
    }
    dbg!(&ancestor_map.len());
    let mut path = VecDeque::new();
    path.push_back(&end);
    let mut u = &end;
    while let Some(anc) = ancestor_map.get(&u) {
        u = anc;
        path.push_front(u);
    }*/

    let mut previous = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut dist = -1;
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some(p) = queue.pop_front() {
        if p.0 == end {
            dist = p.1;
            break;
        }

        for n in valid_neighbors(&grid, &p.0) {
            if !visited.contains(&n) {
                previous.insert(n, p.0);
                queue.push_back((n, p.1 + 1));
                visited.insert(n);
            }
        }
    }

    dbg!(dist);

    //let sol1: u64 = (path.len()-1) as u64; //4112 > x > 122
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn valid_neighbors(grid: &Grid<char>, node: &(usize, usize)) -> Vec<(usize, usize)> {
    let o = grid
        .get_neighbors(node)
        .iter()
        .filter(|n| {
            let v = grid.get(node.0, node.1);
            let v2 = grid.get(n.0, n.1);
            (v as u8).abs_diff(v2 as u8) <= 1
            //&& (v as u8) <= (v2 as u8)
        })
        .copied()
        .collect::<Vec<(usize, usize)>>();
    //dbg!(grid.get(node.0,node.1));
    //println!("{:?}",o.iter().map(|u|grid.get((*u).0,(*u).1)).collect::<HashSet<char>>());
    o
}

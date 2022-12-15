use crate::days::{get_file_lines, Grid};
use crate::{Solution, SolutionPair};
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet};

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
        let mut tmp = HashSet::new();
        for x in 0..grid.width {
            for y in 0..grid.height {
                tmp.insert((x, y));
            }
        }
        tmp
    };

    let mut graph = UnGraph::<char, ()>::new_undirected();
    let mut neigh_map = HashMap::new();
    let mut node_map = HashMap::new();
    let mut nodeindex_map = HashMap::new();
    let mut start_node = Default::default();
    let mut end_node = Default::default();
    for n in &all_nodes {
        let c = if &start == n {
            'S'
        } else if &end == n {
            'E'
        } else {
            grid.get(n.0, n.1)
        };
        let pn = graph.add_node(c);
        if &start == n {
            start_node = pn;
        } else if &end == n {
            end_node = pn;
        }
        neigh_map.insert(*n, valid_neighbors(&grid, n));
        //neigh_map.insert(*n, grid.get_neighbors(n));
        node_map.insert(*n, pn);
        nodeindex_map.insert(pn, n);
    }
    let mut edges: Vec<(NodeIndex, NodeIndex)> = vec![];
    for n in node_map.keys() {
        let pn = node_map.get(n).unwrap();
        let npns = neigh_map
            .get(n)
            .unwrap()
            .iter()
            .map(|nn| node_map.get(nn).unwrap())
            .collect::<Vec<&NodeIndex>>();
        for nn in npns {
            edges.push((*pn, *nn))
        }
    }
    graph.extend_with_edges(edges);
    let mut max = 0;
    for i in &all_nodes {
        let c = grid.get(i.0, i.1);
        if c == 'a' {
            let mut sum = {
                let s = node_map.get(i).unwrap();
                let result = dijkstra(&graph, *s, Some(end_node), |_| 1);
                let mut vals = result.iter().collect::<Vec<(&NodeIndex, &i32)>>();
                vals.sort_by(|(_, i), (_, a)| i.cmp(a));
                *vals.last().unwrap().1
            };
            let mut sum2 = {
                let s = node_map.get(i).unwrap();
                let result = dijkstra(&graph, end_node, Some(*s), |_| 1);
                let mut vals = result.iter().collect::<Vec<(&NodeIndex, &i32)>>();
                vals.sort_by(|(_, i), (_, a)| i.cmp(a));
                *vals.last().unwrap().1
            };
            //dbg!(sum,sum2,sum+sum2);
            max = max.max(sum + sum2);
        }
    }
    dbg!(max);
    let result = dijkstra(&graph, start_node, Some(end_node), |_| 1);
    let mut vals = result.iter().collect::<Vec<(&NodeIndex, &i32)>>();
    vals.sort_by(|(_, i), (_, a)| i.cmp(a));
    let co = nodeindex_map.get(vals.last().unwrap().0).unwrap();
    dbg!(co, grid.get(co.0, co.1));
    dbg!(vals.last());
    let result = dijkstra(&graph, end_node, Some(start_node), |_| 1);
    let mut vals = result.iter().collect::<Vec<(&NodeIndex, &i32)>>();
    vals.sort_by(|(_, i), (_, a)| i.cmp(a));
    let co = nodeindex_map.get(vals.last().unwrap().0).unwrap();
    dbg!(co, grid.get(co.0, co.1));
    dbg!(vals.last());
    //dbg!(grid.get(146,18));
    //dbg!(valid_neighbors(&grid,&(145,18)));
    //dbg!(valid_neighbors(&grid,&(146,18)));
    //dbg!(valid_neighbors(&grid,&(147,18)));

    /*let all_edges = {
        let mut tmp = vec![];
        for n in &all_nodes {
            for nn in valid_neighbors(&grid, n) {
                tmp.push((*n, nn));
            }
        }
        tmp
    };*/

    /*let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut d = HashMap::new();
    visited.insert(start);
    q.push_back(start);
    d.insert(start, 0);
    let mut min_d = 999999999usize;
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

    dbg!(visited.len());
    dbg!(all_nodes.len());
    dbg!(min_d);*/

    /*let mut path = VecDeque::new();
    path.push_back(start.clone());

    let mut distance_map = HashMap::new();
    for node in &all_nodes {
        distance_map.insert(node.clone(), usize::MAX);
    }
    distance_map.insert(start, 0);

    let mut ancestor_map = HashMap::new();
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

    /*let mut previous = HashMap::new();
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

    dbg!(dist);*/

    //let sol1: u64 = (path.len()-1) as u64; //4112 > x > 122
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn valid_neighbors(grid: &Grid<char>, node: &(usize, usize)) -> Vec<(usize, usize)> {
    let v = grid.get(node.0, node.1);
    let o = grid
        .get_neighbors(node)
        .iter()
        .filter(|n| {
            let v2 = grid.get(n.0, n.1);
            (v as u8).abs_diff(v2 as u8) <= 1
        })
        .copied()
        .collect::<Vec<(usize, usize)>>();
    o
}

use crate::days::{get_file_lines, Grid};
use crate::{Solution, SolutionPair};
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};
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

    let mut graph = DiGraph::<char, ()>::new();
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
    let result = dijkstra(&graph, start_node, Some(end_node), |_| 1);
    let mut min = i32::MAX;
    for i in &all_nodes {
        let c = grid.get(i.0, i.1);
        if c == 'a' {
            let s = node_map.get(i).unwrap();
            let result = dijkstra(&graph, *s, Some(end_node), |_| 1);
            if let Some(t) = result.get(&end_node).copied() {
                min = min.min(t);
            }
        }
    }
    dbg!(min);

    let sol1: i32 = result.get(&end_node).copied().unwrap();
    let sol2: i32 = min;

    (Solution::I32(sol1), Solution::I32(sol2))
}

fn valid_neighbors(grid: &Grid<char>, node: &(usize, usize)) -> Vec<(usize, usize)> {
    let v = grid.get(node.0, node.1);
    let o = grid
        .get_neighbors(node)
        .iter()
        .filter(|n| {
            let v2 = grid.get(n.0, n.1);
            (v2 as i32) - (v as i32) <= 1
        })
        .copied()
        .collect::<Vec<(usize, usize)>>();
    o
}

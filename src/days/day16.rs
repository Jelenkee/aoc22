use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use petgraph::algo::{dijkstra};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Graph;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(111);
    let mut valve_map = HashMap::new();
    let mut first = String::new();
    for line in lines {
        let v = parse(&line);
        if first.is_empty() {
            first = v.name.clone();
        }
        valve_map.insert(v.name.clone(), v);
    }
    let mut graph = DiGraph::<String, ()>::new();
    let mut index_map = HashMap::new();
    let mut index_map2 = HashMap::new();
    let mut edges: Vec<(NodeIndex, NodeIndex)> = vec![];

    for (name, valve) in &valve_map {
        let index = *index_map
            .entry(name.clone())
            .or_insert_with(|| graph.add_node(name.clone()));
        for nei in &valve.valves {
            let index2 = index_map
                .entry(nei.clone())
                .or_insert_with(|| graph.add_node(nei.clone()));
            edges.push((index, *index2));
        }
    }

    for (k, v) in &index_map {
        index_map2.insert(v, k);
    }

    graph.extend_with_edges(edges);

    let mut total_rate = 0u16;
    let mut released_pressure = 0u16;
    let mut current_index = index_map.get(&first).copied().unwrap();
    let mut target_index = None;
    let mut distance_map = HashMap::new();
    for index in index_map.values() {
        distance_map.insert(index, dijkstra(&graph, *index, None, |_| 1));
    }

    let mut open_valves = HashSet::new();
    for i in 1..=30 {
        released_pressure += total_rate;

        let indexes = distance_map
            .get(&current_index)
            .unwrap()
            .iter()
            .collect::<Vec<(&NodeIndex, &i32)>>();
        let remaining_minutes = 30 - i;
        //indexes.sort_by(|a,b,|a.1.cmp(b.1).reverse());
        if target_index.is_none() {
            let mut valves = indexes
                .iter()
                .map(|i| (i.0, get_valve(i.0, &valve_map, &graph)))
                .filter(|p| !open_valves.contains(p.1))
                .filter(|p| p.1.rate > 0)
                .collect::<Vec<(&NodeIndex, &Valve)>>();
            valves.sort_by(|a, b| {
                (a.1.rate as i32
                    * (remaining_minutes
                        - *distance_map.get(&current_index).unwrap().get(a.0).unwrap()))
                .cmp(
                    &(b.1.rate as i32
                        * (remaining_minutes
                            - *distance_map.get(&current_index).unwrap().get(b.0).unwrap())),
                )
                .reverse()
            });
            dbg!(&valves);
            target_index = valves.first().map(|v| v.0);
        }

        if let Some(target) = target_index {
            if target == &current_index {
                let valve1 = get_valve(target, &valve_map, &graph);
                total_rate += valve1.rate;
                open_valves.insert(valve1);
                target_index = None;
            } else {
                let neighbors = graph.neighbors(current_index);
                let mut ins = neighbors.collect::<Vec<NodeIndex>>();
                ins.sort_by(|a, b| {
                    distance_map
                        .get(target)
                        .unwrap()
                        .get(a)
                        .unwrap()
                        .cmp(distance_map.get(target).unwrap().get(b).unwrap())
                });
                current_index = ins.first().copied().unwrap();
            }
        } else {
            panic!("target must be present");
        }
    }

    let output = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut f = File::create("/home/gaus/o.dot").unwrap();
    f.write(output.as_bytes()).unwrap();
    let sol1: u64 = released_pressure as u64;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn get_valve<'a>(
    index: &NodeIndex,
    map: &'a HashMap<String, Valve>,
    graph: &Graph<String, ()>,
) -> &'a Valve {
    map.get(&graph[*index]).unwrap()
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u16,
    valves: Vec<String>,
}

impl Eq for Valve {}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
    }
}

fn parse(line: &str) -> Valve {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    //dbg!(&parts);
    let name = parts.get(1).unwrap().to_string();
    let rate = parts
        .get(4)
        .unwrap()
        .trim_start_matches("rate=")
        .trim_end_matches(";")
        .parse::<u16>()
        .unwrap();
    let mut vec = vec![];
    for i in 9..parts.len() {
        vec.push(parts.get(i).unwrap().trim_end_matches(',').to_string())
    }
    Valve {
        name,
        rate,
        valves: vec,
    }
}

use crate::days::day14::Material::{AIR, ROCK, SAND};
use crate::days::{get_file_lines, Grid};
use crate::{Solution, SolutionPair};
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(14);
    let reg = Regex::new("\\d+,\\d+").unwrap();
    let mut max = 0usize;
    for line in &lines {
        let m = reg
            .find_iter(line)
            .map(|m| m.as_str())
            .map(|s| s.split(',').last().unwrap().parse::<usize>().unwrap())
            .max()
            .unwrap();
        max = max.max(m);
    }

    let mut grid = Grid::new(1000, max + 3);
    for i in 0..(grid.width - 1) {
        grid.set(i, max + 2, ROCK);
    }
    for line in lines {
        let coords = line.split(" -> ").map(|s| s.trim()).collect::<Vec<&str>>();
        for i in 0..(coords.len() - 1) {
            let coord = coords.get(i).unwrap();
            let next_coord = coords.get(i + 1).unwrap();
            let c1 = coord
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let c2 = next_coord
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            assert!(c1[0] == c2[0] || c1[1] == c2[1]);
            if c1[0] == c2[0] {
                let min = c1[1].min(c2[1]);
                let max = c1[1].max(c2[1]);
                for y in min..=max {
                    grid.set(c1[0], y, ROCK);
                }
            } else {
                let min = c1[0].min(c2[0]);
                let max = c1[0].max(c2[0]);
                for x in min..=max {
                    grid.set(x, c1[1], ROCK);
                }
            }
        }
    }
    let mut sum = 0u64;
    /*while !insert_sand(&mut grid) {
        sum += 1;
    }*/
    let mut sum2 = 0u64;
    while matches!(grid.get(500, 0), AIR) {
        insert_sand(&mut grid);
        sum2 += 1;
    }

    let sol1: u64 = sum;
    let sol2: u64 = sum2;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn insert_sand(grid: &mut Grid<Material>) -> bool {
    let mut current = (500usize, 0usize);
    grid.set(current.0, current.1, SAND);
    loop {
        if current.1 >= grid.height - 1 {
            return true;
        }
        let down = (current.0, current.1 + 1);
        let ldown = (current.0 - 1, current.1 + 1);
        let rdown = (current.0 + 1, current.1 + 1);
        let down_mat = grid.get(down.0, down.1);
        let ldown_mat = grid.get(ldown.0, ldown.1);
        let rdown_mat = grid.get(rdown.0, rdown.1);
        if matches!(down_mat, AIR) {
            move_block(grid, current, down);
            current = down;
        } else if matches!(ldown_mat, AIR) {
            move_block(grid, current, ldown);
            current = ldown;
        } else if matches!(rdown_mat, AIR) {
            move_block(grid, current, rdown);
            current = rdown;
        } else {
            break;
        }
    }
    false
}

fn move_block(grid: &mut Grid<Material>, from: (usize, usize), to: (usize, usize)) {
    let from_material = grid.get(from.0, from.1);
    let to_material = grid.get(to.0, to.1);
    assert!(matches!(to_material, AIR));
    grid.set(from.0, from.1, AIR);
    grid.set(to.0, to.1, from_material);
}

#[derive(Clone, Copy, Debug)]
enum Material {
    AIR,
    ROCK,
    SAND,
}

impl Default for Material {
    fn default() -> Self {
        AIR
    }
}

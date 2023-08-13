use std::collections::HashSet;
use crate::days::day17::Material::{AIR, FLYING_ROCK, ROCK};
use crate::days::{get_file_lines, Grid};
use crate::{Solution, SolutionPair};
use std::fmt::{Display, Formatter};

///////////////////////////////////////////////////////////////////////////////
const WIDTH: usize = 7usize;

pub fn solve() -> SolutionPair {
    let input = get_file_lines(111).get(0).unwrap().to_string();
    let shapes = "-+lio".to_string();
    let mut grid = Grid::<Material>::new(WIDTH, 10000);
    let mut landed_rocks = 0;
    let mut shape_index = 0;
    let mut dir_index = 0;
    let mut flying = false;
    let mut vec = vec![];
    let mut record=true;
    let mut set=HashSet::new();
    while landed_rocks < 2022 {
        if dir_index % input.len() == 0 && dir_index % input.len() == 0 {
            let high = get_highest_rock(&grid);
            let mut s = String::new();
            for x in 0..grid.width {
                s.push(
                    grid.get_safe(x, high)
                        .unwrap_or(ROCK)
                        .to_string()
                        .chars()
                        .next()
                        .unwrap(),
                );
            }
            if !set.insert(s.clone()){
                record=false;
                println!("{}",s);
            }
            println!(
                "{} {} {}",
                landed_rocks,
                (grid.height - get_highest_rock(&grid) as usize),
                s
            );
        }
        if record{
            vec.push((landed_rocks,(grid.height - get_highest_rock(&grid) as usize)));
        }
        let dir = input.chars().nth(dir_index % input.len()).unwrap();
        let shape = shapes.chars().nth(shape_index % shapes.len()).unwrap();
        if !flying {
            spawn_rock(shape, &mut grid);
            flying = true;
            shape_index += 1;
        }
        shift_rocks(dir == '<', &mut grid);
        dir_index += 1;
        if fall(&mut grid) {
            flying = false;
            landed_rocks += 1;
        }
    }

    // Your solution here...
    let sol1: u64 = (grid.height - get_highest_rock(&grid) as usize) as u64;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn fall(grid: &mut Grid<Material>) -> bool {
    let mut coords = get_flying_rocks(grid);
    let new_coords = coords
        .iter()
        .map(|(x, y)| (*x, *y + 1))
        .collect::<Vec<(usize, usize)>>();
    if new_coords.iter().any(|(x, y)| {
        grid.get_safe(*x, *y).is_none() || matches!(grid.get_safe(*x, *y), Some(ROCK))
    }) {
        for (x, y) in coords {
            grid.set(x, y, ROCK);
        }
        true
    } else {
        for (x, y) in coords {
            grid.set(x, y, AIR);
        }
        for (x, y) in new_coords {
            grid.set(x, y, FLYING_ROCK);
        }
        false
    }
}

fn shift_rocks(left: bool, grid: &mut Grid<Material>) {
    let mut coords = get_flying_rocks(grid);
    let most_left = coords.iter().map(|p| p.0).min().unwrap();
    let most_right = coords.iter().map(|p| p.0).max().unwrap();
    let new_coords = if left && most_left > 0 {
        coords
            .iter()
            .map(|(x, y)| (*x - 1, *y))
            .collect::<Vec<(usize, usize)>>()
    } else if !left && most_right < (WIDTH - 1) {
        coords
            .iter()
            .map(|(x, y)| (*x + 1, *y))
            .collect::<Vec<(usize, usize)>>()
    } else {
        vec![]
    };
    if !new_coords.is_empty()
        && new_coords.iter().all(|(x, y)| match grid.get(*x, *y) {
            AIR | FLYING_ROCK => true,
            _ => false,
        })
    {
        for (x, y) in coords {
            grid.set(x, y, AIR);
        }
        for (x, y) in new_coords {
            grid.set(x, y, FLYING_ROCK);
        }
    }
}

fn get_flying_rocks(grid: &Grid<Material>) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    for x in 0..grid.width {
        for y in 0..grid.height {
            if matches!(grid.get(x, y), FLYING_ROCK) {
                coords.push((x, y));
            }
        }
    }
    coords
}

fn spawn_rock(id: char, grid: &mut Grid<Material>) {
    let high_y = get_highest_rock(grid);
    match id {
        '-' => {
            let y = (high_y - 4) as usize;
            grid.set(2, y, FLYING_ROCK);
            grid.set(3, y, FLYING_ROCK);
            grid.set(4, y, FLYING_ROCK);
            grid.set(5, y, FLYING_ROCK);
        }
        '+' => {
            let y = (high_y - 4) as usize;
            grid.set(3, y, FLYING_ROCK);
            grid.set(2, y - 1, FLYING_ROCK);
            grid.set(3, y - 1, FLYING_ROCK);
            grid.set(4, y - 1, FLYING_ROCK);
            grid.set(3, y - 2, FLYING_ROCK);
        }
        'l' => {
            let y = (high_y - 4) as usize;
            grid.set(2, y, FLYING_ROCK);
            grid.set(3, y, FLYING_ROCK);
            grid.set(4, y, FLYING_ROCK);
            grid.set(4, y - 1, FLYING_ROCK);
            grid.set(4, y - 2, FLYING_ROCK);
        }
        'i' => {
            let y = (high_y - 4) as usize;
            grid.set(2, y, FLYING_ROCK);
            grid.set(2, y - 1, FLYING_ROCK);
            grid.set(2, y - 2, FLYING_ROCK);
            grid.set(2, y - 3, FLYING_ROCK);
        }
        'o' => {
            let y = (high_y - 4) as usize;
            grid.set(2, y, FLYING_ROCK);
            grid.set(3, y, FLYING_ROCK);
            grid.set(2, y - 1, FLYING_ROCK);
            grid.set(3, y - 1, FLYING_ROCK);
        }
        _ => {
            panic!("invalid id {}", id)
        }
    };
}

fn get_highest_rock(grid: &Grid<Material>) -> usize {
    for y in 0..grid.height {
        for x in 0..grid.width {
            if matches!(grid.get(x, y), Material::ROCK) {
                return y as usize;
            }
        }
    }
    grid.height as usize
}

#[derive(Default, Copy, Clone)]
enum Material {
    #[default]
    AIR,
    FLYING_ROCK,
    ROCK,
}

impl Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AIR => String::from("."),
            FLYING_ROCK => String::from("@"),
            ROCK => String::from("#"),
        };
        write!(f, "{}", s)
    }
}

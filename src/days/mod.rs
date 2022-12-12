use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn get_file_lines(day: u8) -> Vec<String> {
    let string: String = fs::read_to_string(format!("input/{}.txt", day)).unwrap();
    string.lines().map(|s| s.to_string()).collect()
}

pub struct Grid<T> {
    vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy + Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            vec: Vec::from_iter((0..(width * height)).map(|_| Default::default())),
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        *self.vec.get(self.get_index(x, y)).unwrap()
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        let index = self.get_index(x, y);
        self.vec[index] = v;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_neighbors(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut vec = vec![];
        if pos.1 < self.height - 1 {
            vec.push((pos.0, pos.1 + 1));
        }
        if pos.0 < self.width - 1 {
            vec.push((pos.0 + 1, pos.1));
        }
        if pos.0 > 0 {
            vec.push((pos.0 - 1, pos.1));
        }
        if pos.1 > 0 {
            vec.push((pos.0, pos.1 - 1));
        }
        vec
    }
}

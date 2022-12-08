use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(8);
    let width = lines.get(0).unwrap().len();
    let height = lines.len();
    let mut grid = Grid::new(width, height);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c.to_digit(10).unwrap() as u8);
        }
    }

    let mut visible_tree_counter = 0;
    let mut max_score = 0;
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let tree = grid.get(x, y);
            //PART 1
            {
                let mut blocked = 0;
                for x2 in 0..x {
                    if grid.get(x2, y) >= tree {
                        blocked += 1;
                        break;
                    }
                }
                for x2 in (x + 1)..width {
                    if grid.get(x2, y) >= tree {
                        blocked += 1;
                        break;
                    }
                }

                for y2 in 0..y {
                    if grid.get(x, y2) >= tree {
                        blocked += 1;
                        break;
                    }
                }
                for y2 in (y + 1)..height {
                    if grid.get(x, y2) >= tree {
                        blocked += 1;
                        break;
                    }
                }

                if blocked < 4 {
                    visible_tree_counter += 1;
                }
            }

            //PART 2
            {
                let mut a = 0;
                let mut b = 0;
                let mut c = 0;
                let mut d = 0;
                for x2 in (0..x).rev() {
                    a += 1;
                    if grid.get(x2, y) >= tree {
                        break;
                    }
                }
                for x2 in (x + 1)..width {
                    b += 1;
                    if grid.get(x2, y) >= tree {
                        break;
                    }
                }

                for y2 in (0..y).rev() {
                    c += 1;
                    if grid.get(x, y2) >= tree {
                        break;
                    }
                }
                for y2 in (y + 1)..height {
                    d += 1;
                    if grid.get(x, y2) >= tree {
                        break;
                    }
                }

                max_score = max_score.max(a * b * c * d);
            }
        }
    }

    let sol1: u64 = (visible_tree_counter + 2 * width + 2 * height - 4) as u64;
    let sol2: u64 = max_score;

    (Solution::U64(sol1), Solution::U64(sol2))
}

struct Grid {
    vec: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            vec: Vec::from_iter((0..(width * height)).map(|_| 0)),
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        *self.vec.get(self.get_index(x, y)).unwrap()
    }

    fn set(&mut self, x: usize, y: usize, v: u8) {
        self.vec.insert(self.get_index(x, y), v)
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.height + x
    }
}

use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

const STEPS: [i32; 6] = [20, 60, 100, 140, 180, 220];

pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(10);
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 1;
    let mut pixels = vec![];

    for line in lines {
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();
        match command {
            "noop" => {
                draw_pixel(&cycle, &x, &mut pixels);
                if let Some(n) = increase_cycle(&mut cycle, &x) {
                    sum += n;
                }
            }
            "addx" => {
                let num = split.next().unwrap().parse::<i32>().unwrap();
                draw_pixel(&cycle, &x, &mut pixels);
                if let Some(n) = increase_cycle(&mut cycle, &x) {
                    sum += n;
                }
                draw_pixel(&cycle, &x, &mut pixels);
                x += num;
                if let Some(n) = increase_cycle(&mut cycle, &x) {
                    sum += n;
                }
            }
            &_ => {
                panic!("invalid command")
            }
        };

        fn draw_pixel(cycle: &i32, x: &i32, pixels: &mut Vec<char>) {
            let lit = ((x - 1)..=(x + 1)).contains(&((*cycle - 1) % 40));
            pixels.push(if lit { '#' } else { '.' });
        }

        fn increase_cycle(cycle: &mut i32, x: &i32) -> Option<i32> {
            *cycle += 1;
            if STEPS.contains(cycle) {
                Some(*cycle * x)
            } else {
                None
            }
        }
    }
    let image_lines = pixels
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();
    let mut s = String::new();
    for line in image_lines {
        s.extend(line.chars().into_iter());
        s.push('\n');
    }
    println!("{}", s);
    let sol1: i32 = sum;
    let sol2: u64 = 0;

    (Solution::I32(sol1), Solution::U64(sol2))
}

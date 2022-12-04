use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = get_file_lines(4);
    let mut num1 = 0u64;
    let mut num2 = 0u64;
    for line in lines {
        let mut split = line.split(',');
        let elv1 = get_elv(split.next());
        let elv2 = get_elv(split.next());
        if elv_contains_another(&elv1, &elv2) || elv_contains_another(&elv2, &elv1) {
            num1 += 1;
        }
        if elv_overlaps_another(&elv1, &elv2) {
            println!("{:?}   {:?}", elv1, elv2);
            num2 += 1;
        }
    }
    // Your solution here...
    let sol1: u64 = num1;
    let sol2: u64 = num2;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn get_elv(str: Option<&str>) -> Elv {
    str.map(|s| s.split('-').collect::<Vec<&str>>())
        .map(|v| Elv(v[0].parse::<u8>().unwrap(), v[1].parse::<u8>().unwrap()))
        .unwrap()
}
fn elv_contains_another(elv1: &Elv, elv2: &Elv) -> bool {
    elv1.0 <= elv2.0 && elv1.1 >= elv2.1
}
fn elv_overlaps_another(elv1: &Elv, elv2: &Elv) -> bool {
    (elv1.0..=elv1.1)
        .filter(|n| (elv2.0..=elv2.1).contains(n))
        .count()
        > 0
}
#[derive(Debug)]
struct Elv(u8, u8);

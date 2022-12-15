use crate::days::get_file_lines;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};

///////////////////////////////////////////////////////////////////////////////
type Sensor = (isize, isize);
type Beacon = (isize, isize);
pub fn solve() -> SolutionPair {
    // Your solution here...
    let lines = get_file_lines(15);
    let mut map = HashMap::new();
    for line in lines {
        let (s, b) = parse_line(&line);
        map.insert(s, b);
    }
    let beacons = map.values().collect::<HashSet<&Beacon>>();

    let mut pairs = map
        .iter()
        .map(|(s, b)| (s.0 - distance(s, b), s.0 + distance(s, b)))
        .collect::<Vec<(isize, isize)>>();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let most_left = pairs.first().map(|p| p.0).unwrap();
    pairs.sort_by(|a, b| a.1.cmp(&b.1));
    let most_right = pairs.last().map(|p| p.1).unwrap();

    let y = 2000000isize;
    let mut sum = 0u64;
    for x in most_left..=most_right {
        if beacons.contains(&(x, y)) {
            continue;
        }
        if is_in_sensor_range(x, y, &map) {
            sum += 1;
        }
    }

    let max = 4000000isize;
    let range = 0..=max;

    let mut prod = 0u64;
    for p in map.keys().flat_map(|p| get_out_border(p, &map)) {
        if range.contains(&p.0) && range.contains(&p.1) && !is_in_sensor_range(p.0, p.1, &map) {
            prod = (4000000 * p.0 + p.1) as u64;
            break;
        }
    }

    let sol1: u64 = sum;
    let sol2: u64 = prod;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn is_in_sensor_range(x: isize, y: isize, map: &HashMap<Sensor, Beacon>) -> bool {
    for sensor in map.keys() {
        let dis1 = distance(sensor, map.get(sensor).unwrap());
        let dis2 = distance(sensor, &(x, y));
        if dis1 >= dis2 {
            return true;
        }
    }
    false
}

fn get_out_border(sensor: &Sensor, map: &HashMap<Sensor, Beacon>) -> HashSet<(isize, isize)> {
    let mut set = HashSet::new();
    let dis = distance(sensor, map.get(sensor).unwrap());
    let start = sensor.1 - (dis + 1);
    let end = sensor.1 + (dis + 1);
    let mut diff = 0;
    for y in start..=end {
        set.insert((sensor.0 + diff, y));
        set.insert((sensor.0 - diff, y));
        if y < sensor.1 {
            diff += 1;
        } else {
            diff -= 1;
        }
    }
    set
}

fn distance(a: &(isize, isize), b: &(isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_line(line: &str) -> (Sensor, Beacon) {
    let parts = line.split(":").collect::<Vec<&str>>();
    return (parse_part(parts[0]), parse_part(parts[1]));

    fn parse_part(part: &str) -> (isize, isize) {
        let e1 = part.find('=').unwrap();
        let e2 = part.rfind('=').unwrap();
        let c = part.find(',').unwrap();

        (
            (&part[(e1 + 1)..c]).parse::<isize>().unwrap(),
            (&part[(e2 + 1)..part.len()]).parse::<isize>().unwrap(),
        )
    }
}

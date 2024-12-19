use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

pub fn solve() {
    RamRun::new(String::from("test0")).solve(12);
    RamRun::new(String::from("gh")).solve(1024);
    RamRun::new(String::from("google")).solve(1024);
}

impl RamRun {
    fn build_map(&mut self, byte_count: i32) {
        self.w = 0;
        self.h = 0;
        for coord in self.coords.iter().get(..byte_count as usize) {
            if self.w < coord.0 + 1 {
                self.w = coord.0 + 1;
            }
            if self.h < coord.1 + 1 {
                self.h = coord.1 + 1;
            }
        }

        self.map.clear();
        self.map.resize(self.w as usize, vec![]);
        self.map
            .iter_mut()
            .for_each(|x| x.resize(self.h as usize, '.'));
        for coord in self.coords.iter().get(..byte_count as usize) {
            self.map[coord.0 as usize][coord.1 as usize] = '#';
        }
    }

    fn print_map(&self) {
        println!("{}", "-".repeat(self.w as usize));
        for j in 0..self.h {
            for i in 0..self.w {
                print!(
                    "{}",
                    if self.route.contains(&(i, j)) {
                        'O'
                    } else {
                        self.map[i as usize][j as usize]
                    }
                );
            }
            println!();
        }
        println!("{}", "-".repeat(self.w as usize));
    }

    fn traverse_dfs(&mut self, p: &(i32, i32), route: Vec<(i32, i32)>) {
        if p.0 == self.w - 1 && p.1 == self.h - 1 {
            if self.route.is_empty() || self.route.len() > route.len() {
                self.route = route;
            }
            return;
        }
        let coords: Vec<(i32, i32)> = [
            (p.0 - 1, p.1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
        ]
        .iter()
        .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < self.w && *j < self.h)
        .filter(|(i, j)| self.map[*i as usize][*j as usize] == '.')
        .filter(|(i, j)| !route.contains(&(*i, *j)))
        .map(|(i, j)| (*i, *j))
        .collect();
        for coord in coords.iter() {
            let mut new_route = route.clone();
            new_route.push(*coord);
            self.traverse_dfs(coord, new_route);
        }
    }

    fn traverse_dijkstra(&self, start: (i32, i32), end: (i32, i32)) -> i32 {
        let mut viz: HashSet<(i32, i32)> = HashSet::new();
        let mut dst: HashMap<(i32, i32), i32> = HashMap::new();
        dst.insert((start.0, start.1), 0);
        let mut pq: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
        pq.push(Reverse((0i32, start.0, start.1)));

        loop {
            let p_raw = pq.pop().unwrap().0;
            let p = (p_raw.1, p_raw.2);
            viz.insert((p.0, p.1));

            let its: Vec<(i32, i32)> = [
                (p.0 - 1, p.1),
                (p.0 + 1, p.1),
                (p.0, p.1 - 1),
                (p.0, p.1 + 1),
            ]
            .iter()
            .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < self.w && *j < self.h)
            .filter(|(i, j)| self.map[*i as usize][*j as usize] == '.')
            .map(|(i, j)| (*i, *j))
            .collect();

            for it in its.iter() {
                if viz.contains(it) {
                    continue;
                }
                let new_dist = *dst.get(&p).unwrap_or(&i32::MAX) + 1;
                if new_dist < *dst.get(it).unwrap_or(&i32::MAX) {
                    dst.insert((it.0, it.1), new_dist);
                    pq.push(Reverse((new_dist, it.0, it.1)));
                }
            }
            if pq.is_empty() {
                break;
            }
        }

        if dst.contains_key(&end) {
            dst[&end]
        } else {
            i32::MAX
        }
    }

    fn find_blocker(&mut self, byte_count: i32) -> String {
        let coords: Vec<(i32, i32)> = self
            .coords
            .iter()
            .get(byte_count as usize..)
            .map(|x| (x.0, x.1))
            .collect();
        for coord in coords.iter() {
            // let pre = self.map[coord.0 as usize][coord.1 as usize];
            self.map[coord.0 as usize][coord.1 as usize] = '#';
            // self.build_map(byte_count);
            // println!("{:?}", coord);
            let res = self.traverse_dijkstra((0, 0), (self.w - 1, self.h - 1));
            // self.map[coord.0 as usize][coord.1 as usize] = pre;
            if res == i32::MAX {
                return format!("{},{}", coord.0, coord.1);
            }
        }
        String::from("N/A")
    }

    fn solve(&mut self, byte_count: i32) -> (i32, String) {
        self.build_map(byte_count);
        // self.traverse_dfs(&(0, 0), vec![(0, 0)]);
        // self.print_map();
        // self.part1 = self.route.len() as i32 - 1;
        self.part1 = self.traverse_dijkstra((0, 0), (self.w - 1, self.h - 1));
        self.part2 = self.find_blocker(byte_count);

        println!("Test Name: {}", self.test_name);
        println!("Day 18, Part 1: {}", self.part1);
        println!("Day 18, Part 2: {}", self.part2);

        (self.part1, self.part2.clone())
    }

    fn read_input(test_name: &str) -> Vec<(i32, i32)> {
        let raw =
            fs::read_to_string(format!("../data/day18/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.trim().is_empty())
            .map(|x| {
                let p: Vec<&str> = x.split(",").collect();
                (p[0].parse().unwrap(), p[1].parse().unwrap())
            })
            .collect()
    }

    fn new(test_name: String) -> RamRun {
        let coords = Self::read_input(&test_name);
        RamRun {
            map: vec![],
            w: 0,
            h: 0,
            coords,
            route: vec![],
            part1: 0,
            part2: String::new(),
            test_name,
        }
    }
}

struct RamRun {
    map: Vec<Vec<char>>,
    w: i32,
    h: i32,
    coords: Vec<(i32, i32)>,
    route: Vec<(i32, i32)>,
    part1: i32,
    part2: String,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::RamRun;

    #[test]
    fn test_part1() {
        assert_eq!(RamRun::new(String::from("test0")).solve(12).0, 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(RamRun::new(String::from("test0")).solve(12).1, "6,1");
    }
}

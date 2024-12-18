use itertools::Itertools;
use std::fs;

pub fn solve() {
    RamRun::new(String::from("test0")).solve();
    RamRun::new(String::from("gh")).solve();
    RamRun::new(String::from("google")).solve();
}

impl RamRun {
    fn build_map(&mut self) {
        self.w = 0;
        self.h = 0;
        for coord in self.coords.iter().get(..12) {
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
        for coord in self.coords.iter().get(..12) {
            self.map[coord.0 as usize][coord.1 as usize] = '#';
        }
    }

    fn print_map(&self) {
        println!("{}", "-".repeat(self.w as usize));
        for j in 0..self.h {
            for i in 0..self.w {
                print!("{}", self.map[i as usize][j as usize]);
            }
            println!();
        }
        println!("{}", "-".repeat(self.w as usize));
    }

    fn solve(&mut self) -> (i32, i32) {
        self.build_map();
        self.print_map();

        println!("Coords: {:?}", self.coords);

        println!("Test Name: {}", self.test_name);
        println!("Day 18, Part 1: {}", self.part1);
        println!("Day 18, Part 2: {}", self.part2);

        (self.part1, self.part2)
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
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct RamRun {
    map: Vec<Vec<char>>,
    w: i32,
    h: i32,
    coords: Vec<(i32, i32)>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::RamRun;
    #[test]
    fn test_part1() {
        assert_eq!(RamRun::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(RamRun::new(String::from("test1")).solve().1, 0);
    }
}

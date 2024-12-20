use itertools::Itertools;
use std::{collections::HashMap, fs};

pub fn solve() {
    RaceCondition::new(String::from("test0")).solve();
    RaceCondition::new(String::from("gh")).solve();
    RaceCondition::new(String::from("google")).solve();
}

impl RaceCondition {
    fn dfs(&mut self, curr: (i32, i32), route: Vec<(i32, i32)>) {
        if let Some(last) = route.last() {
            if *last == self.end {
                if self.route.is_empty() || self.route.len() > route.len() {
                    self.route = route.clone();
                }
                return;
            }
        }
        let dirs = [
            (curr.0 - 1, curr.1),
            (curr.0 + 1, curr.1),
            (curr.0, curr.1 - 1),
            (curr.0, curr.1 + 1),
        ];
        let w = self.w;
        let h = self.h;
        let dirs: Vec<(i32, i32)> = dirs
            .iter()
            .filter(|x| x.0 >= 0 && x.1 >= 0 && x.0 < w && x.1 < h)
            .map(|x| (x.0, x.1))
            .filter(|x| self.grid[x.0 as usize][x.1 as usize] != '#')
            .filter(|x| !route.contains(x))
            .collect();
        for dir in dirs.iter() {
            let mut new_route = route.clone();
            new_route.push(*dir);
            self.dfs(*dir, new_route);
        }
    }

    fn solve_internal(&mut self) -> i32 {
        self.reset();
        self.dfs(self.start, vec![self.start]);

        let mut fr: HashMap<i32, i32> = HashMap::new();

        for (ai, (ax, ay)) in self.route.iter().enumerate() {
            let ps = [
                (*ax - 2, *ay),
                (*ax + 2, *ay),
                (*ax, *ay - 2),
                (*ax, *ay + 2),
            ];
            for p in ps.iter() {
                if let Some(bi) = self.route.iter().position(|x| x.0 == p.0 && x.1 == p.1) {
                    if bi > ai {
                        let diff = bi as i32 - ai as i32 - 2;
                        if let Some(ex) = fr.get(&diff) {
                            fr.insert(diff, ex + 1);
                        } else {
                            fr.insert(diff, 1);
                        }
                    }
                }
            }
        }

        // println!("{:#?}", fr);
        let mut res = 0;
        for it in fr.iter().sorted() {
            // println!("{} - {}", *it.0, *it.1);
            if *it.0 >= 100 {
                res += *it.1;
            }
        }
        res
    }

    fn solve(&mut self) -> (i32, i32) {
        self.part1 = self.solve_internal();

        println!("Test Name: {}", self.test_name);
        println!("Day 20, Part 1: {}", self.part1);
        println!("Day 20, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn reset(&mut self) {
        self.route.clear();

        self.grid.clear();
        self.grid.resize(self.w as usize, vec![]);
        self.grid
            .iter_mut()
            .for_each(|x| x.resize(self.h as usize, '.'));

        self.start = (0, 0);
        self.end = (0, 0);

        for j in 0..self.h {
            for i in 0..self.w {
                match self.input[i as usize][j as usize] {
                    'S' => {
                        self.start = (i, j);
                        self.grid[i as usize][j as usize] = '.';
                    }
                    'E' => {
                        self.end = (i, j);
                        self.grid[i as usize][j as usize] = '.';
                    }
                    x => self.grid[i as usize][j as usize] = x,
                }
            }
        }
    }

    fn print_grid(&self) {
        println!("{}", "-".repeat(self.w as usize));
        for i in 0..self.w {
            for j in 0..self.h {
                print!("{}", self.grid[i as usize][j as usize]);
            }
            println!();
        }
        println!("{}", "-".repeat(self.w as usize));
    }

    fn read_input(test_name: &str) -> Vec<Vec<char>> {
        let raw =
            fs::read_to_string(format!("../data/day20/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().next().unwrap())
                    .collect()
            })
            .collect()
    }

    fn new(test_name: String) -> RaceCondition {
        let input = Self::read_input(&test_name);
        let w = input.len() as i32;
        let h = input[0].len() as i32;
        RaceCondition {
            input,
            grid: vec![],

            start: (0, 0),
            end: (0, 0),

            route: vec![],
            w,
            h,

            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct RaceCondition {
    input: Vec<Vec<char>>,
    grid: Vec<Vec<char>>,

    start: (i32, i32),
    end: (i32, i32),

    route: Vec<(i32, i32)>,
    w: i32,
    h: i32,

    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::RaceCondition;

    #[test]
    fn test_part1() {
        assert_eq!(RaceCondition::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(RaceCondition::new(String::from("test0")).solve().1, 0);
    }
}

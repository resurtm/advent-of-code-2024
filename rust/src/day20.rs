use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

pub fn solve() {
    RaceCondition::new(String::from("test0")).solve();
    // RaceCondition::new(String::from("gh")).solve();
    // RaceCondition::new(String::from("google")).solve();
}

impl RaceCondition {
    fn dfs(&mut self, curr: (usize, usize), route: Vec<(usize, usize)>) {
        if let Some(last) = route.last() {
            if *last == self.end {
                if self.dfs_res == 0 || self.dfs_res > route.len() {
                    self.dfs_res = route.len();
                }
                return;
            }
        }
        let dirs = [
            (curr.0 as i32 - 1, curr.1 as i32),
            (curr.0 as i32 + 1, curr.1 as i32),
            (curr.0 as i32, curr.1 as i32 - 1),
            (curr.0 as i32, curr.1 as i32 + 1),
        ];
        let w = self.w;
        let h = self.h;
        let dirs: Vec<(usize, usize)> = dirs
            .iter()
            .filter(|x| x.0 >= 0 && x.1 >= 0 && x.0 < w as i32 && x.1 < h as i32)
            .map(|x| (x.0 as usize, x.1 as usize))
            .filter(|x| self.grid[x.0][x.1] != '#')
            .filter(|x| !route.contains(x))
            .collect();
        for dir in dirs.iter() {
            let mut new_route = route.clone();
            new_route.push(*dir);
            self.dfs(*dir, new_route);
        }
    }

    fn solve_internal(&mut self) {
        self.reset();
        self.dfs(self.start, vec![]);
        let base_res = self.dfs_res;
        let mut fr: HashMap<usize, HashSet<((usize, usize), (usize, usize))>> = HashMap::new();

        for i in 0..self.w {
            for j in (0..self.h - 2).step_by(2) {
                self.reset();
                self.grid[i][j] = 'x';
                self.grid[i][j + 1] = 'x';
                self.dfs(self.start, vec![]);
                if self.dfs_res != base_res {
                    let diff = base_res - self.dfs_res;
                    if let Some(ex) = fr.get_mut(&diff) {
                        ex.insert(((i, j), (i, j + 1)));
                    } else {
                        let mut n = HashSet::new();
                        n.insert(((i, j), (i, j + 1)));
                        fr.insert(diff, n);
                    }
                }
            }
        }

        for i in (0..self.w - 2).step_by(2) {
            for j in 0..self.h {
                self.reset();
                self.grid[i][j] = 'x';
                self.grid[i + 1][j] = 'x';
                self.dfs(self.start, vec![]);
                if self.dfs_res != base_res {
                    let diff = base_res - self.dfs_res;
                    if let Some(ex) = fr.get_mut(&diff) {
                        ex.insert(((i, j), (i + 1, j)));
                    } else {
                        let mut n = HashSet::new();
                        n.insert(((i, j), (i + 1, j)));
                        fr.insert(diff, n);
                    }
                }
            }
        }

        for it in fr.iter() {
            println!("{} {}", it.0, it.1.len());
        }
        // println!("{:#?}", fr.get(&64usize).unwrap());
    }

    fn solve(&mut self) -> (i128, i128) {
        self.solve_internal();

        println!("Test Name: {}", self.test_name);
        println!("Day 20, Part 1: {}", self.part1);
        println!("Day 20, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn reset(&mut self) {
        self.dfs_res = 0;

        self.grid.clear();
        self.grid.resize(self.w, vec![]);
        self.grid.iter_mut().for_each(|x| x.resize(self.h, '.'));

        self.start = (0, 0);
        self.end = (0, 0);

        for j in 0..self.h {
            for i in 0..self.w {
                match self.input[i][j] {
                    'S' => {
                        self.start = (i, j);
                        self.grid[i][j] = '.';
                    }
                    'E' => {
                        self.end = (i, j);
                        self.grid[i][j] = '.';
                    }
                    x => self.grid[i][j] = x,
                }
            }
        }
    }

    fn print_grid(&self) {
        println!("{}", "-".repeat(self.w));
        for i in 0..self.w {
            for j in 0..self.h {
                print!("{}", self.grid[i][j]);
            }
            println!();
        }
        println!("{}", "-".repeat(self.w));
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
        let w = input.len();
        let h = input[0].len();
        RaceCondition {
            input,
            grid: vec![],

            start: (0, 0),
            end: (0, 0),

            dfs_res: 0,
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

    start: (usize, usize),
    end: (usize, usize),

    dfs_res: usize,
    w: usize,
    h: usize,

    part1: i128,
    part2: i128,
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

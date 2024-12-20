use std::{fs, usize};

pub fn solve() {
    RaceCondition::new(String::from("test0")).solve();
    // RaceCondition::new(String::from("gh")).solve();
    // RaceCondition::new(String::from("google")).solve();
}

impl RaceCondition {
    fn solve(&mut self) -> (i128, i128) {
        self.reset();
        self.print_grid();

        println!("Test Name: {}", self.test_name);
        println!("Day 20, Part 1: {}", self.part1);
        println!("Day 20, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn reset(&mut self) {
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

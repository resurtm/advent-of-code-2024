use std::fs;

pub fn solve() {
    RaceCondition::new(String::from("test0")).solve();
    // RaceCondition::new(String::from("gh")).solve();
    // RaceCondition::new(String::from("google")).solve();
}

impl RaceCondition {
    fn solve(&mut self) -> (i128, i128) {
        self.print_grid();

        println!("Test Name: {}", self.test_name);
        println!("Day 20, Part 1: {}", self.part1);
        println!("Day 20, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn print_grid(&self) {
        println!("{}", "-".repeat(self.w));
        for j in 0..self.h {
            for i in 0..self.w {
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
        let grid = Self::read_input(&test_name);
        let w = grid.len();
        let h = grid[0].len();
        RaceCondition {
            grid,
            w,
            h,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct RaceCondition {
    grid: Vec<Vec<char>>,
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

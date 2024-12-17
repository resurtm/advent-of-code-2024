use regex::Regex;
use std::{collections::HashMap, fs};

pub fn solve() {
    ChronospatialComputer::new(String::from("test0")).solve();
    // ChronospatialComputer::new(String::from("gh")).solve();
    // ChronospatialComputer::new(String::from("google")).solve();
}

impl ChronospatialComputer {
    fn solve(&mut self) -> (i32, i32) {
        println!("Registers: {:?}", self.regs);
        println!("Instructions: {:?}", self.insts);

        println!("Test Name: {}", self.test_name);
        println!("Day 17, Part 1: {}", self.part1);
        println!("Day 17, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> (HashMap<char, i32>, Vec<i32>) {
        let raw =
            fs::read_to_string(format!("../data/day17/{}.txt", test_name)).expect("input file");

        let mut regs = HashMap::new();
        let mut insts = Vec::new();

        let re = Regex::new(r"(Register (.*)|Program): (:\d+|.*)").unwrap();
        re.captures_iter(&raw).for_each(|x| {
            let kind = x.get(1).unwrap().as_str();
            if kind.starts_with("Register ") {
                regs.insert(
                    x.get(2).unwrap().as_str().chars().next().unwrap(),
                    x.get(3).unwrap().as_str().parse().unwrap(),
                );
            } else if kind.starts_with("Program") {
                x.get(3).unwrap().as_str().split(",").for_each(|c| {
                    insts.push(c.parse().unwrap());
                });
            }
        });

        (regs, insts)
    }

    fn new(test_name: String) -> ChronospatialComputer {
        let input = Self::read_input(&test_name);
        ChronospatialComputer {
            regs: input.0,
            insts: input.1,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct ChronospatialComputer {
    regs: HashMap<char, i32>,
    insts: Vec<i32>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::ChronospatialComputer;

    #[test]
    fn test_part1() {
        assert_eq!(
            ChronospatialComputer::new(String::from("test0")).solve().0,
            0
        );
        assert_eq!(
            ChronospatialComputer::new(String::from("test1")).solve().0,
            0
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            ChronospatialComputer::new(String::from("test0")).solve().1,
            0
        );
        assert_eq!(
            ChronospatialComputer::new(String::from("test1")).solve().1,
            0
        );
    }
}

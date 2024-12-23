use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs};

pub fn solve() {
    ChronospatialComputer::new(String::from("test0")).solve(1);
    ChronospatialComputer::new(String::from("test1")).solve(1);
    ChronospatialComputer::new(String::from("test1")).solve(2);
    ChronospatialComputer::new(String::from("gh")).solve(1);
    ChronospatialComputer::new(String::from("gh")).solve(2);
    ChronospatialComputer::new(String::from("google")).solve(1);
    ChronospatialComputer::new(String::from("google")).solve(2);
}

impl ChronospatialComputer {
    fn interpret(&mut self) {
        let mut idx = 0;
        loop {
            let inst = self.insts[idx as usize];
            let op = self.insts[idx as usize + 1];

            match inst {
                // adv
                0 => {
                    let x = *self.regs.get(&'A').unwrap() as f64;
                    let y = 2i128.pow(self.combo_op(op) as u32) as f64;
                    let z = (x / y).trunc() as i128;
                    self.regs.insert('A', z);
                }
                // bxl
                1 => {
                    let x = *self.regs.get(&'B').unwrap();
                    let y = op;
                    let z = x ^ y;
                    self.regs.insert('B', z);
                }
                // bst
                2 => {
                    let x = self.combo_op(op);
                    let y = 8;
                    let z = x % y;
                    self.regs.insert('B', z);
                }
                // jnz
                3 => {
                    let x = *self.regs.get(&'A').unwrap();
                    if x != 0 {
                        idx = op;
                        continue;
                    }
                }
                // bxc
                4 => {
                    let x = *self.regs.get(&'B').unwrap();
                    let y = *self.regs.get(&'C').unwrap();
                    let z = x ^ y;
                    self.regs.insert('B', z);
                }
                // out
                5 => {
                    let x = self.combo_op(op) % 8;
                    self.out.push(x);
                }
                // bdv
                6 => {
                    let x = *self.regs.get(&'A').unwrap() as f64;
                    let y = 2i128.pow(self.combo_op(op) as u32) as f64;
                    let z = (x / y).trunc() as i128;
                    self.regs.insert('B', z);
                }
                // cdv
                7 => {
                    let x = *self.regs.get(&'A').unwrap() as f64;
                    let y = 2i128.pow(self.combo_op(op) as u32) as f64;
                    let z = (x / y).trunc() as i128;
                    self.regs.insert('C', z);
                }
                _ => panic!("bad inst"),
            }

            idx += 2;
            if idx >= self.insts.len() as i128 {
                break;
            }
        }
    }

    fn combo_op(&self, op: i128) -> i128 {
        match op {
            0..4 => op,
            4 => *self.regs.get(&'A').unwrap(),
            5 => *self.regs.get(&'B').unwrap(),
            6 => *self.regs.get(&'C').unwrap(),
            7 => panic!("reserved"),
            _ => panic!("bad op"),
        }
    }

    fn find_register_a_bruteforce(&mut self) -> i128 {
        for reg_a in 0..1_000_000 {
            self.reset_state();
            self.regs.insert('A', reg_a);
            self.interpret();

            if self.out == self.insts {
                return reg_a;
            }
        }
        -1
    }

    fn reset_state(&mut self) {
        self.regs = HashMap::from([('A', 0), ('B', 0), ('C', 0)]);
        self.out.clear();
    }

    fn find_register_a(&mut self) -> i128 {
        let mut res = 0;
        for pos in (0..self.insts.len()).rev() {
            res <<= 3;

            loop {
                self.reset_state();
                self.regs.insert('A', res);
                self.interpret();

                if self.out.get(..).unwrap() == self.insts.get(pos..).unwrap() {
                    break;
                }

                res += 1;
            }
        }
        res
    }

    fn solve(&mut self, part: u8) -> (&str, i128) {
        if part == 1 {
            self.interpret();
            self.part1 = self.out.iter().map(|x| x.to_string()).join(",");
        }
        if part == 2 {
            // self.part2 = self.find_register_a_bruteforce();
            self.part2 = self.find_register_a();
        }

        println!("Test Name: {}", self.test_name);
        println!("Day 17, Part 1: {}", self.part1);
        println!("Day 17, Part 2: {}", self.part2);

        (&self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> (HashMap<char, i128>, Vec<i128>) {
        let raw =
            fs::read_to_string(format!("../data/day17/{}.txt", test_name)).expect("input file");

        let mut regs = HashMap::new();
        let mut ops = Vec::new();

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
                    ops.push(c.parse().unwrap());
                });
            }
        });

        (regs, ops)
    }

    fn new(test_name: String) -> ChronospatialComputer {
        let input = Self::read_input(&test_name);
        ChronospatialComputer {
            regs: input.0,
            insts: input.1,
            out: vec![],
            part1: String::new(),
            part2: 0,
            test_name,
        }
    }

    fn new_custom(regs: HashMap<char, i128>, insts: Vec<i128>) -> ChronospatialComputer {
        ChronospatialComputer {
            regs,
            insts,
            out: vec![],
            part1: String::new(),
            part2: 0,
            test_name: String::from(""),
        }
    }
}

struct ChronospatialComputer {
    regs: HashMap<char, i128>,
    insts: Vec<i128>,
    out: Vec<i128>,
    part1: String,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::ChronospatialComputer;
    use std::{collections::HashMap, vec};

    #[test]
    fn test_example1() {
        let mut computer = ChronospatialComputer::new_custom(
            HashMap::from([('A', 0), ('B', 0), ('C', 9)]),
            vec![2, 6],
        );
        computer.solve(1);
        assert_eq!(*computer.regs.get(&'B').unwrap(), 1i128);
    }

    #[test]
    fn test_example2() {
        let mut computer = ChronospatialComputer::new_custom(
            HashMap::from([('A', 10), ('B', 0), ('C', 0)]),
            vec![5, 0, 5, 1, 5, 4],
        );
        computer.solve(1);
        assert_eq!(computer.out, vec![0, 1, 2]);
    }

    #[test]
    fn test_example3() {
        let mut computer = ChronospatialComputer::new_custom(
            HashMap::from([('A', 2024), ('B', 0), ('C', 0)]),
            vec![0, 1, 5, 4, 3, 0],
        );
        computer.solve(1);
        assert_eq!(*computer.regs.get(&'A').unwrap(), 0i128);
        assert_eq!(computer.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_example4() {
        let mut computer = ChronospatialComputer::new_custom(
            HashMap::from([('A', 0), ('B', 29), ('C', 0)]),
            vec![1, 7],
        );
        computer.solve(1);
        assert_eq!(*computer.regs.get(&'B').unwrap(), 26i128);
    }

    #[test]
    fn test_example5() {
        let mut computer = ChronospatialComputer::new_custom(
            HashMap::from([('A', 0), ('B', 2024), ('C', 43690)]),
            vec![4, 0],
        );
        computer.solve(1);
        assert_eq!(*computer.regs.get(&'B').unwrap(), 44354i128);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            ChronospatialComputer::new(String::from("test0")).solve(1).0,
            "4,6,3,5,6,3,5,2,1,0"
        );
        assert_eq!(
            ChronospatialComputer::new(String::from("gh")).solve(1).0,
            "3,6,7,0,5,7,3,1,4"
        );
        assert_eq!(
            ChronospatialComputer::new(String::from("google"))
                .solve(1)
                .0,
            "2,1,4,7,6,0,3,1,4"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            ChronospatialComputer::new(String::from("test1")).solve(2).1,
            117_440
        );
    }
}

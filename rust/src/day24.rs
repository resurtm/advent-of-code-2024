use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;

pub fn solve() {
    CrossedWires::new(String::from("test0")).solve();
    CrossedWires::new(String::from("test1")).solve();
    CrossedWires::new(String::from("gh")).solve();
    CrossedWires::new(String::from("google")).solve();
}

impl CrossedWires {
    fn solve_internal(&mut self) -> i128 {
        let mut vals: HashMap<String, Option<bool>> = HashMap::new();
        self.gates.iter().for_each(|x| {
            vals.insert(x.inp0.to_owned(), None);
            vals.insert(x.inp1.to_owned(), None);
            vals.insert(x.out.to_owned(), None);
        });

        let mut queue = VecDeque::new();
        self.inp_wires
            .iter()
            .for_each(|x| queue.push_back((x.0.to_owned(), x.1)));
        while !queue.is_empty() {
            if let Some(curr) = queue.pop_front() {
                vals.insert(curr.0.to_owned(), Some(curr.1));
                for gate in self.gates.iter_mut() {
                    if gate.inp0 == curr.0 && gate.inp0v.is_none() {
                        gate.inp0v = Some(curr.1);
                    } else if gate.inp1 == curr.0 && gate.inp1v.is_none() {
                        gate.inp1v = Some(curr.1);
                    }
                    if gate.recalc() {
                        queue.push_back((gate.out.to_owned(), gate.outv.unwrap()));
                    }
                }
            }
        }

        let mut vals: Vec<_> = vals.iter().collect();
        vals.sort_by(|x, y| x.0.cmp(y.0));
        let val = vals
            .iter()
            .filter(|x| x.0.starts_with("z"))
            .map(|x| if x.1.unwrap() { '1' } else { '0' })
            .rev()
            .join("");
        i128::from_str_radix(&val, 2).unwrap()
    }

    fn solve(&mut self) -> (i128, i128) {
        self.part1 = self.solve_internal();

        println!("Test Name: {}", self.test_name);
        println!("Day 24, Part 1: {}", self.part1);
        println!("Day 24, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn build_gates(inp_gates: &Vec<(String, String, String, String)>) -> Vec<Gate> {
        inp_gates
            .iter()
            .map(|x| Gate {
                op: match x.0.as_str() {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    "XOR" => Operation::Xor,
                    _ => panic!("invalid op"),
                },
                inp0: x.1.to_owned(),
                inp1: x.2.to_owned(),
                out: x.3.to_owned(),
                inp0v: None,
                inp1v: None,
                outv: None,
            })
            .collect()
    }

    fn read_input(test_name: &str) -> (Vec<(String, bool)>, Vec<(String, String, String, String)>) {
        let raw =
            fs::read_to_string(format!("../data/day24/{}.txt", test_name)).expect("input file");
        let parts: Vec<_> = raw.split("\n\n").collect();
        let wires = parts[0]
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let parts: Vec<_> = x.split(": ").collect();
                (parts[0].to_owned(), parts[1] == "1")
            })
            .collect();
        let gates = parts[1]
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let parts: Vec<_> = x.split(" ").collect();
                (
                    parts[1].to_owned(),
                    parts[0].to_owned(),
                    parts[2].to_owned(),
                    parts[4].to_owned(),
                )
            })
            .collect();
        (wires, gates)
    }

    fn new(test_name: String) -> CrossedWires {
        let (inp_wires, inp_gates) = Self::read_input(&test_name);
        let gates = Self::build_gates(&inp_gates);
        CrossedWires {
            inp_wires,
            inp_gates,
            gates,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct CrossedWires {
    inp_wires: Vec<(String, bool)>,
    inp_gates: Vec<(String, String, String, String)>,
    gates: Vec<Gate>,
    part1: i128,
    part2: i128,
    test_name: String,
}

impl Gate {
    fn recalc(&mut self) -> bool {
        let mut changed = false;
        if let (Some(v0), Some(v1)) = (self.inp0v, self.inp1v) {
            if self.outv.is_none() {
                self.outv = Some(match self.op {
                    Operation::And => v0 & v1,
                    Operation::Or => v0 | v1,
                    Operation::Xor => v0 ^ v1,
                });
                changed = true;
            }
        }
        changed
    }
}

#[derive(Debug)]
struct Gate {
    op: Operation,
    inp0: String,
    inp1: String,
    out: String,
    inp0v: Option<bool>,
    inp1v: Option<bool>,
    outv: Option<bool>,
}

#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

#[cfg(test)]
mod tests {
    use super::CrossedWires;

    #[test]
    fn test_part1() {
        assert_eq!(CrossedWires::new(String::from("test1")).solve().0, 2024);
    }

    #[test]
    fn test_part2() {
        assert_eq!(CrossedWires::new(String::from("test0")).solve().1, 0);
    }
}

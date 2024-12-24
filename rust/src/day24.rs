use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

pub fn solve() {
    CrossedWires::new(String::from("test0")).solve();
    CrossedWires::new(String::from("test1")).solve();
    CrossedWires::new(String::from("test2")).solve();
    CrossedWires::new(String::from("gh")).solve();
    CrossedWires::new(String::from("google")).solve();
}

impl CrossedWires {
    fn solve_internal(&mut self) -> i128 {
        let mut regs = self.build_regs();

        let mut queue = VecDeque::new();
        self.inp_wires
            .iter()
            .for_each(|x| queue.push_back((x.0.to_owned(), x.1)));
        while !queue.is_empty() {
            if let Some(curr) = queue.pop_front() {
                regs.insert(curr.0.to_owned(), Some(curr.1));
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

        Self::parse_regs(&regs, "z")
    }

    fn build_regs(&self) -> HashMap<String, Option<bool>> {
        let mut regs: HashMap<String, Option<bool>> = HashMap::new();
        self.gates.iter().for_each(|x| {
            regs.insert(x.inp0.to_owned(), None);
            regs.insert(x.inp1.to_owned(), None);
            regs.insert(x.out.to_owned(), None);
        });
        self.inp_wires.iter().for_each(|x| {
            regs.insert(x.0.to_owned(), Some(x.1));
        });
        regs
    }

    fn parse_regs(regs: &HashMap<String, Option<bool>>, prefix: &str) -> i128 {
        let mut regs: Vec<_> = regs.iter().collect();
        regs.sort_by(|x, y| x.0.cmp(y.0));
        let val = regs
            .iter()
            .filter(|x| x.0.starts_with(prefix))
            .map(|x| if x.1.unwrap_or(false) { '1' } else { '0' })
            .rev()
            .join("");
        i128::from_str_radix(&val, 2).unwrap()
    }

    fn solve_wrong_gates(&mut self) -> String {
        let mut highest_z = String::from("z00");
        for g in self.gates.iter() {
            if g.out.starts_with("z") && g.out > highest_z {
                highest_z = g.out.to_owned();
            }
        }

        let mut wrong = HashSet::new();
        for (op, inp0, inp1, out) in self.inp_gates.iter() {
            if out.starts_with("z") && op != "XOR" && out != &highest_z {
                wrong.insert(out.to_owned());
            }
            if op == "XOR"
                && !inp0.starts_with("x")
                && !inp0.starts_with("y")
                && !inp0.starts_with("z")
                && !inp1.starts_with("x")
                && !inp1.starts_with("y")
                && !inp1.starts_with("z")
                && !out.starts_with("x")
                && !out.starts_with("y")
                && !out.starts_with("z")
            {
                wrong.insert(out.to_owned());
            }
            if op == "AND" && inp0 != "x00" && inp1 != "x00" {
                for (_op, _inp0, _inp1, _out) in self.inp_gates.iter() {
                    if _op != "OR" && (out == _inp0 || out == _inp1) {
                        wrong.insert(out.to_owned());
                    }
                }
            }
            if op == "XOR" {
                for (_op, _inp0, _inp1, _out) in self.inp_gates.iter() {
                    if _op == "OR" && (out == _inp0 || out == _inp1) {
                        wrong.insert(out.to_owned());
                    }
                }
            }
        }

        let mut wrong: Vec<_> = wrong.iter().collect();
        wrong.sort();
        wrong.iter().join(",")
    }

    fn solve(&mut self) -> (i128, String) {
        self.gates = Self::build_gates(&self.inp_gates);
        self.part1 = self.solve_internal();

        self.gates = Self::build_gates(&self.inp_gates);
        self.part2 = self.solve_wrong_gates();

        println!("Test Name: {}", self.test_name);
        println!("Day 24, Part 1: {}", self.part1);
        println!("Day 24, Part 2: {}", self.part2);

        (self.part1, self.part2.to_owned())
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
        CrossedWires {
            inp_wires,
            inp_gates,
            gates: vec![],
            part1: 0,
            part2: String::new(),
            test_name,
        }
    }
}

struct CrossedWires {
    inp_wires: Vec<(String, bool)>,
    inp_gates: Vec<(String, String, String, String)>,
    gates: Vec<Gate>,
    part1: i128,
    part2: String,
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

#[derive(Debug, PartialEq)]
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
        assert_eq!(CrossedWires::new(String::from("test0")).solve().1, "z00");
        assert_eq!(
            CrossedWires::new(String::from("test1")).solve().1,
            "ffh,hwm,kjc,mjb,ntg,rvg,tgd,wpb,z02,z03,z05,z06,z07,z08,z10,z11"
        );
    }
}

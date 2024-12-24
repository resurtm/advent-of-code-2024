use std::fs;

pub fn solve() {
    CrossedWires::new(String::from("test0")).solve();
    // CrossedWires::new(String::from("test1")).solve();
    // CrossedWires::new(String::from("gh")).solve();
    // CrossedWires::new(String::from("google")).solve();
}

impl CrossedWires {
    fn solve(&mut self) -> (i32, i32) {
        println!("Wires: {:?}", self.inp_wires);
        println!("Gates: {:?}", self.inp_gates);
        println!("Gates: {:?}", self.gates);

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
    part1: i32,
    part2: i32,
    test_name: String,
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
        assert_eq!(CrossedWires::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(CrossedWires::new(String::from("test0")).solve().1, 0);
    }
}

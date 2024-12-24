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

        println!("Test Name: {}", self.test_name);
        println!("Day 24, Part 1: {}", self.part1);
        println!("Day 24, Part 2: {}", self.part2);

        (self.part1, self.part2)
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
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct CrossedWires {
    inp_wires: Vec<(String, bool)>,
    inp_gates: Vec<(String, String, String, String)>,
    part1: i32,
    part2: i32,
    test_name: String,
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

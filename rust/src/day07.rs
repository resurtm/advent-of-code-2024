use std::fs;

pub fn solve() {
    BridgeRepair::new(String::from("test0")).solve();
    // BridgeRepair::new(String::from("gh")).solve();
    // BridgeRepair::new(String::from("google")).solve();
}

impl BridgeRepair {
    fn new(test_name: String) -> BridgeRepair {
        BridgeRepair {
            eqs: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve_internal(&mut self) {
        println!("Equations: {:?}", self.eqs);
    }

    fn solve(&mut self) -> (i32, i32) {
        self.solve_internal();
        println!("Test Name: {}", self.test_name);
        println!("Day 07, Part 1: {}", self.part1);
        println!("Day 07, Part 2: {}", self.part2);
        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<(i32, Vec<i32>)> {
        let raw =
            fs::read_to_string(format!("../data/day07/{}.txt", test_name)).expect("input file");
        raw.split("\n")
            .filter(|x| !x.is_empty())
            .map(|x: &str| {
                let parts: Vec<&str> = x.split(":").collect();
                (
                    parts
                        .get(0)
                        .expect("left")
                        .parse::<i32>()
                        .expect("correct left"),
                    parts
                        .get(1)
                        .expect("right")
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<i32>().expect("correct right"))
                        .collect(),
                )
            })
            .collect()
    }
}

struct BridgeRepair {
    eqs: Vec<(i32, Vec<i32>)>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::BridgeRepair;

    #[test]
    fn test_part1() {
        assert_eq!(BridgeRepair::new(String::from("test0")).solve().0, 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(BridgeRepair::new(String::from("test0")).solve().1, 0);
    }
}

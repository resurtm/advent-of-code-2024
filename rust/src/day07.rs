use itertools::Itertools;
use std::fs;

pub fn solve() {
    BridgeRepair::new(String::from("test0")).solve();
    BridgeRepair::new(String::from("gh")).solve();
    BridgeRepair::new(String::from("google")).solve();
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

    fn solve_internal(&self) -> (i128, i128) {
        let mut p1 = 0;
        let mut p2 = 0;
        for eq in self.eqs.iter() {
            p1 += Self::check_eq(&eq, false);
            p2 += Self::check_eq(&eq, true);
        }
        (p1, p2)
    }

    fn check_eq(eq: &(i128, Vec<i128>), ex: bool) -> i128 {
        let chars = if ex {
            vec!['+', '*', '|']
        } else {
            vec!['+', '*']
        };
        for perm in (0..eq.1.len() - 1)
            .map(|_| chars.iter())
            .multi_cartesian_product()
        {
            let mut res = eq.1[0];
            for i in 0..perm.len() {
                match perm[i] {
                    '+' => res += eq.1[i + 1],
                    '*' => res *= eq.1[i + 1],
                    '|' => {
                        res = format!("{}{}", res, eq.1[i + 1])
                            .parse::<i128>()
                            .expect("invalid str");
                    }
                    _ => panic!("invalid op"),
                }
            }
            if eq.0 == res {
                return eq.0;
            }
        }
        0
    }

    fn solve(&mut self) -> (i128, i128) {
        let (a, b) = self.solve_internal();
        self.part1 = a;
        self.part2 = b;
        println!("Test Name: {}", self.test_name);
        println!("Day 07, Part 1: {}", self.part1);
        println!("Day 07, Part 2: {}", self.part2);
        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<(i128, Vec<i128>)> {
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
                        .parse::<i128>()
                        .expect("correct left"),
                    parts
                        .get(1)
                        .expect("right")
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<i128>().expect("correct right"))
                        .collect(),
                )
            })
            .collect()
    }
}

struct BridgeRepair {
    eqs: Vec<(i128, Vec<i128>)>,
    part1: i128,
    part2: i128,
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
        assert_eq!(BridgeRepair::new(String::from("test0")).solve().1, 11387);
    }
}

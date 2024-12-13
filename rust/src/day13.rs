use regex::Regex;
use std::fs;

pub fn solve() {
    ClawContraption::new(String::from("test0")).solve();
    ClawContraption::new(String::from("gh")).solve();
    ClawContraption::new(String::from("google")).solve();
}

impl ClawContraption {
    fn solve_internal(&mut self, part: u8) {
        for it in self.inp.iter() {
            let det = it.a.0 * it.b.1 - it.b.0 * it.a.1;
            if det == 0 {
                continue;
            }
            let x = (it.p.0 * it.b.1 - it.b.0 * it.p.1) as f64 / det as f64;
            let y = (it.a.0 * it.p.1 - it.p.0 * it.a.1) as f64 / det as f64;
            if x.trunc() != x || y.trunc() != y {
                continue;
            }
            match part {
                0 => self.part1 += x as i128 * 3 + y as i128,
                1 => self.part2 += x as i128 * 3 + y as i128,
                _ => panic!("bad part"),
            }
        }
    }

    fn adjust(&mut self) {
        for it in self.inp.iter_mut() {
            it.p.0 += 10_000_000_000_000;
            it.p.1 += 10_000_000_000_000;
        }
    }

    fn solve(&mut self) -> (i128, i128) {
        self.solve_internal(0);
        self.adjust();
        self.solve_internal(1);

        println!("Test Name: {}", self.test_name);
        println!("Day 13, Part 1: {}", self.part1);
        println!("Day 13, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<Entry> {
        let raw =
            fs::read_to_string(format!("../data/day13/{}.txt", test_name)).expect("input file");
        let re = Regex::new(r"(Prize|Button (A|B)): X\+?=?(\d+), Y\+?=?(\d+)").unwrap();
        let mut entry = Entry::new();
        let mut res = Vec::new();
        re.captures_iter(&raw)
            .for_each(|x| match x.get(1).unwrap().as_str() {
                "Button A" => {
                    entry.a = (
                        x.get(3).unwrap().as_str().parse::<i128>().unwrap(),
                        x.get(4).unwrap().as_str().parse::<i128>().unwrap(),
                    );
                }
                "Button B" => {
                    entry.b = (
                        x.get(3).unwrap().as_str().parse::<i128>().unwrap(),
                        x.get(4).unwrap().as_str().parse::<i128>().unwrap(),
                    );
                }
                "Prize" => {
                    entry.p = (
                        x.get(3).unwrap().as_str().parse::<i128>().unwrap(),
                        x.get(4).unwrap().as_str().parse::<i128>().unwrap(),
                    );
                    res.push(entry.clone());
                    entry = Entry::new();
                }
                _ => panic!("invalid data"),
            });
        res
    }

    fn new(test_name: String) -> ClawContraption {
        let inp = Self::read_input(&test_name);
        ClawContraption {
            inp,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

impl Entry {
    fn new() -> Entry {
        Entry {
            a: (0, 0),
            b: (0, 0),
            p: (0, 0),
        }
    }
}

struct ClawContraption {
    inp: Vec<Entry>,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[derive(Clone, Debug)]
struct Entry {
    a: (i128, i128),
    b: (i128, i128),
    p: (i128, i128),
}

#[cfg(test)]
mod tests {
    use super::ClawContraption;

    #[test]
    fn test_part1() {
        assert_eq!(ClawContraption::new(String::from("test0")).solve().0, 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(ClawContraption::new(String::from("test0")).solve().1, 0);
    }
}

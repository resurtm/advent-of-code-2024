use regex::Regex;
use std::fs;

pub fn solve() {
    ClawContraption::new(String::from("test0")).solve();
    ClawContraption::new(String::from("gh")).solve();
    ClawContraption::new(String::from("google")).solve();
}

impl ClawContraption {
    fn solve_internal(&mut self) {
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
            self.part1 += x as i32 * 3 + y as i32;
        }
    }

    fn solve(&mut self) -> (i32, i32) {
        self.solve_internal();

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
                        x.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                        x.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    );
                }
                "Button B" => {
                    entry.b = (
                        x.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                        x.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    );
                }
                "Prize" => {
                    entry.p = (
                        x.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                        x.get(4).unwrap().as_str().parse::<i32>().unwrap(),
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
    part1: i32,
    part2: i32,
    test_name: String,
}

#[derive(Clone, Debug)]
struct Entry {
    a: (i32, i32),
    b: (i32, i32),
    p: (i32, i32),
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

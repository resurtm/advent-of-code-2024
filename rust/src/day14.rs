use regex::Regex;
use std::fs;

pub fn solve() {
    RestroomRedoubt::new(String::from("test0")).solve();
    // RestroomRedoubt::new(String::from("gh")).solve();
    // RestroomRedoubt::new(String::from("google")).solve();
}

impl RestroomRedoubt {
    fn solve(&mut self) -> (i32, i32) {
        println!("Input: {:#?}", self.inp);
        println!("Test Name: {}", self.test_name);
        println!("Day 14, Part 1: {}", self.part1);
        println!("Day 14, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<Entry> {
        let raw =
            fs::read_to_string(format!("../data/day14/{}.txt", test_name)).expect("input file");
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut entry = Entry::new();
        let mut res = Vec::new();
        re.captures_iter(&raw).for_each(|x| {
            entry.p = (
                x.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                x.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            );
            entry.v = (
                x.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                x.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            );
            res.push(entry.clone());
            entry = Entry::new();
        });
        res
    }

    fn new(test_name: String) -> RestroomRedoubt {
        let inp = Self::read_input(&test_name);
        RestroomRedoubt {
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
            p: (0, 0),
            v: (0, 0),
        }
    }
}

struct RestroomRedoubt {
    inp: Vec<Entry>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[derive(Clone, Debug)]
struct Entry {
    p: (i32, i32),
    v: (i32, i32),
}

#[cfg(test)]
mod tests {
    use super::RestroomRedoubt;

    #[test]
    fn test_part1() {
        assert_eq!(RestroomRedoubt::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(RestroomRedoubt::new(String::from("test0")).solve().1, 0);
    }
}

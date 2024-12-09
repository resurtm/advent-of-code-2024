use std::collections::VecDeque;
use std::fs;

pub fn solve() {
    DiskFragmenter::new(String::from("test0")).solve();
    DiskFragmenter::new(String::from("gh")).solve();
    DiskFragmenter::new(String::from("google")).solve();
}

impl DiskFragmenter {
    fn new(test_name: String) -> DiskFragmenter {
        DiskFragmenter {
            input: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve_internal(&self) -> i128 {
        // step 1
        let mut d: VecDeque<i128> = VecDeque::new();
        let mut cnt = 0;
        for (idx, ch) in self.input.chars().enumerate() {
            for _ in 0..ch.to_digit(10).unwrap_or(0) {
                d.push_back(if idx % 2 == 0 { cnt } else { -1 });
            }
            if idx % 2 == 0 {
                cnt += 1;
            }
        }

        // step 2
        let mut d2 = VecDeque::new();
        'outer: loop {
            if let Some(it) = d.pop_front() {
                if it == -1 {
                    let mut x1 = 0;
                    'inner: loop {
                        if let Some(x2) = d.pop_back() {
                            if x2 == -1 {
                                continue;
                            }
                            x1 = x2;
                            break 'inner;
                        } else {
                            break 'outer;
                        }
                    }
                    d2.push_back(x1);
                } else {
                    d2.push_back(it);
                }
            } else {
                break;
            }
        }

        // step 3
        let mut res = 0;
        for (idx, it) in d2.iter().enumerate() {
            res += idx as i128 * it;
        }
        res
    }

    fn solve(&mut self) -> (i128, i128) {
        self.part1 = self.solve_internal();

        println!("Test Name: {}", self.test_name);
        println!("Day 09, Part 1: {}", self.part1);
        println!("Day 09, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> String {
        fs::read_to_string(format!("../data/day09/{}.txt", test_name)).expect("input file")
    }
}

struct DiskFragmenter {
    input: String,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::DiskFragmenter;

    #[test]
    fn test_part1() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().0, 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().1, 0);
    }
}

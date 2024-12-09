use std::fs;

pub fn solve() {
    DiskFragmenter::new(String::from("test0")).solve();
    // DiskFragmenter::new(String::from("gh")).solve();
    // DiskFragmenter::new(String::from("google")).solve();
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

    fn solve(&mut self) -> (i32, i32) {
        println!("Input: {}", self.input);

        println!("Test Name: {}", self.test_name);
        println!("Day 08, Part 1: {}", self.part1);
        println!("Day 08, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> String {
        fs::read_to_string(format!("../data/day09/{}.txt", test_name)).expect("input file")
    }
}

struct DiskFragmenter {
    input: String,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::DiskFragmenter;

    #[test]
    fn test_part1() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().0, 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().1, 34);
    }
}

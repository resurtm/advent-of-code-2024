use std::fs;

pub fn solve() {
    LinenLayout::new(String::from("test0")).solve();
    // LinenLayout::new(String::from("gh")).solve();
    // LinenLayout::new(String::from("google")).solve();
}

impl LinenLayout {
    fn solve(&mut self) -> (i32, i32) {
        println!("Patterns: {:?}", self.patterns);
        println!("Items: {:?}", self.items);

        println!("Test Name: {}", self.test_name);
        println!("Day 19, Part 1: {}", self.part1);
        println!("Day 19, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> (Vec<String>, Vec<String>) {
        let raw =
            fs::read_to_string(format!("../data/day19/{}.txt", test_name)).expect("input file");
        let parts: Vec<&str> = raw.split("\n\n").collect();
        let patterns = parts
            .first()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect();
        let items = parts
            .get(1)
            .unwrap()
            .lines()
            .filter(|x| !x.trim().is_empty())
            .map(String::from)
            .collect();
        (patterns, items)
    }

    fn new(test_name: String) -> LinenLayout {
        let (patterns, items) = Self::read_input(&test_name);
        LinenLayout {
            patterns,
            items,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct LinenLayout {
    patterns: Vec<String>,
    items: Vec<String>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::LinenLayout;

    #[test]
    fn test_part1() {
        assert_eq!(LinenLayout::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(LinenLayout::new(String::from("test0")).solve().1, 0);
    }
}

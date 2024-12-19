use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() {
    LinenLayout::new(String::from("test0")).solve();
    LinenLayout::new(String::from("gh")).solve();
    LinenLayout::new(String::from("google")).solve();
}

impl LinenLayout {
    fn internal(&self, item: &str, cache: &mut HashMap<String, i128>) -> i128 {
        if item.is_empty() {
            return 1;
        }
        if !cache.contains_key(item) {
            let mut sum = 0;
            for pattern in self.patterns.iter() {
                if item.starts_with(pattern) {
                    sum += self.internal(&item[pattern.len()..], cache);
                }
            }
            cache.insert(item.to_string(), sum);
        }
        *cache.get(item).unwrap_or(&0)
    }

    fn solve_internal(&self) -> (i128, i128) {
        let mut r1 = 0;
        let mut r2 = 0;
        let mut cache = HashMap::new();
        for item in self.items.iter() {
            let res = self.internal(item, &mut cache);
            r1 += if res > 0 { 1 } else { 0 };
            r2 += res;
        }
        (r1, r2)
    }

    fn solve(&mut self) -> (i128, i128) {
        let (p1, p2) = self.solve_internal();
        self.part1 = p1;
        self.part2 = p2;

        println!("Test Name: {}", self.test_name);
        println!("Day 19, Part 1: {}", self.part1);
        println!("Day 19, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> (HashSet<String>, Vec<String>) {
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
    patterns: HashSet<String>,
    items: Vec<String>,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::LinenLayout;

    #[test]
    fn test_part1() {
        assert_eq!(LinenLayout::new(String::from("test0")).solve().0, 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(LinenLayout::new(String::from("test0")).solve().1, 16);
    }
}

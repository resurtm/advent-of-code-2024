use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() {
    MonkeyMarket::new(String::from("test0")).solve();
    MonkeyMarket::new(String::from("test1")).solve();
    MonkeyMarket::new(String::from("gh")).solve();
    MonkeyMarket::new(String::from("google")).solve();
}

impl MonkeyMarket {
    fn process_secret(inp: i128) -> i128 {
        let prune_const = 16_777_216;
        let mut secret = inp;
        secret = ((secret * 64) ^ secret) % prune_const;
        secret = ((secret / 32) ^ secret) % prune_const;
        secret = ((secret * 2048) ^ secret) % prune_const;
        secret
    }

    fn solve_part1(&self) -> i128 {
        let mut ret = 0;
        for &it in self.input.iter() {
            let mut secret = it;
            for _ in 0..2000 {
                secret = Self::process_secret(secret);
            }
            ret += secret;
        }
        ret
    }

    fn solve_part2(&self) -> i128 {
        let mut patterns: HashMap<Vec<i128>, i128> = HashMap::new();
        let mut checked: HashSet<Vec<i128>> = HashSet::new();

        for &it in self.input.iter() {
            let mut secret = it;
            let mut changes = vec![];
            let mut change = 0i128;
            for _ in 0..2000 + 1 {
                changes.push((secret % 10, change));
                let new_secret = Self::process_secret(secret);
                change = new_secret % 10 - secret % 10;
                secret = new_secret;
            }

            checked.clear();
            for idx in 0..changes.len() - 3 {
                let pattern = changes[idx..idx + 4]
                    .iter()
                    .map(|x| x.1)
                    .collect::<Vec<i128>>();
                if !checked.contains(&pattern) {
                    checked.insert(pattern.clone());
                    let curr = patterns.get(&pattern).unwrap_or(&0);
                    patterns.insert(pattern, curr + changes[idx + 3].0);
                }
            }
        }

        let res = patterns.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
        // println!("{:?}", res);
        *res.1
    }

    fn solve(&mut self) -> (i128, i128) {
        self.part1 = self.solve_part1();
        self.part2 = self.solve_part2();

        println!("Test Name: {}", self.test_name);
        println!("Day 22, Part 1: {}", self.part1);
        println!("Day 22, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<i128> {
        let raw =
            fs::read_to_string(format!("../data/day22/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i128>().unwrap())
            .collect()
    }

    fn new(test_name: String) -> MonkeyMarket {
        let input = Self::read_input(&test_name);
        MonkeyMarket {
            input,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct MonkeyMarket {
    input: Vec<i128>,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::MonkeyMarket;

    #[test]
    fn test_part1() {
        assert_eq!(
            MonkeyMarket::new(String::from("test0")).solve().0,
            37_327_623
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(MonkeyMarket::new(String::from("test1")).solve().1, 23);
    }
}

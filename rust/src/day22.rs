use std::fs;

pub fn solve() {
    MonkeyMarket::new(String::from("test0")).solve();
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

    fn solve_internal(&self) -> i128 {
        let mut ret = 0;
        for &it in self.input.iter() {
            let mut secret = it;
            for _ in 0..2000 {
                secret = Self::process_secret(secret);
            }
            ret += secret;
            // println!("{}: {}", it, secret);
        }
        ret
    }

    fn solve(&mut self) -> (i128, i128) {
        self.part1 = self.solve_internal();

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
        assert_eq!(MonkeyMarket::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(MonkeyMarket::new(String::from("test0")).solve().1, 0);
    }
}

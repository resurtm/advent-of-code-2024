use std::{collections::HashMap, fs};

pub fn solve() {
    PlutonianPebbles::new(String::from("gh")).solve(-1);
    PlutonianPebbles::new(String::from("google")).solve(-1);
}

impl PlutonianPebbles {
    fn new(test_name: String) -> PlutonianPebbles {
        PlutonianPebbles {
            input: Self::read_input(&test_name),
            stones: vec![],
            cache: HashMap::new(),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve_part1(&mut self, blinks: i32) {
        self.stones = self.input.clone();
        for _ in 0..blinks {
            let mut idx = 0;
            loop {
                let it = self.stones[idx];
                let its = it.to_string();
                if it == 0 {
                    self.stones[idx] = 1;
                } else if its.len() % 2 == 0 {
                    let mut first = its[..its.len() / 2].trim_start_matches('0').to_owned();
                    if first.len() == 0 {
                        first = String::from("0");
                    }
                    let mut second = its[its.len() / 2..].trim_start_matches('0').to_owned();
                    if second.len() == 0 {
                        second = String::from("0");
                    }
                    self.stones.remove(idx);
                    self.stones.insert(idx, second.parse::<i128>().unwrap());
                    self.stones.insert(idx, first.parse::<i128>().unwrap());
                    idx += 1;
                } else {
                    self.stones[idx] *= 2024;
                }
                idx += 1;
                if idx == self.stones.len() {
                    break;
                }
            }
        }
        self.part1 = self.stones.len() as i128;
    }

    fn recursive(&mut self, stone: i128, blink: i32, blinks: i32) -> i128 {
        let key = format!("{}-{}", stone, blink);
        if let Some(v) = self.cache.get(&key) {
            return *v;
        }
        if blink > blinks {
            return 1;
        }
        if stone == 0 {
            let v = self.recursive(1, blink + 1, blinks);
            self.cache.insert(key, v);
            return v;
        }
        let st = stone.to_string();
        if st.len() % 2 == 0 {
            let mut first = st[..st.len() / 2].trim_start_matches('0').to_owned();
            if first.len() == 0 {
                first = String::from("0");
            }
            let mut second = st[st.len() / 2..].trim_start_matches('0').to_owned();
            if second.len() == 0 {
                second = String::from("0");
            }
            let a = self.recursive(first.parse::<i128>().unwrap(), blink + 1, blinks);
            let b = self.recursive(second.parse::<i128>().unwrap(), blink + 1, blinks);
            let v = a + b;
            self.cache.insert(key, v);
            return v;
        }
        let v = self.recursive(stone * 2024, blink + 1, blinks);
        self.cache.insert(key, v);
        return v;
    }

    fn solve_part2(&mut self, blinks: i32) {
        self.stones = self.input.clone();
        let mut idx = 0;
        loop {
            self.part2 += self.recursive(self.stones[idx], 1, blinks);
            idx += 1;
            if idx == self.stones.len() {
                break;
            }
        }
    }

    fn solve(&mut self, blinks: i32) -> (i128, i128) {
        self.solve_part1(if blinks == -1 { 25 } else { blinks });
        self.solve_part2(if blinks == -1 { 75 } else { blinks });

        println!("Test Name: {}", self.test_name);
        println!("Day 11, Part 1: {}", self.part1);
        println!("Day 11, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<i128> {
        let raw =
            fs::read_to_string(format!("../data/day11/{}.txt", test_name)).expect("input file");
        raw.split_ascii_whitespace()
            .into_iter()
            .map(|x| x.parse::<i128>().unwrap())
            .collect()
    }
}

struct PlutonianPebbles {
    input: Vec<i128>,
    stones: Vec<i128>,
    cache: HashMap<String, i128>,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::PlutonianPebbles;

    #[test]
    fn test() {
        assert_eq!(
            PlutonianPebbles::new(String::from("test0")).solve(1),
            (7, 7)
        );
        assert_eq!(
            PlutonianPebbles::new(String::from("test1")).solve(6),
            (22, 22)
        );
        assert_eq!(
            PlutonianPebbles::new(String::from("test1")).solve(25),
            (55312, 55312)
        );
    }
}

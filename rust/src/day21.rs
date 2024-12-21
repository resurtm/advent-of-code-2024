use std::{collections::HashMap, fs};

pub fn solve() {
    KeypadConundrum::new(String::from("test0")).solve();
    // KeypadConundrum::new(String::from("gh")).solve();
    // KeypadConundrum::new(String::from("google")).solve();
}

impl KeypadConundrum {
    fn simulate(&self, actions: &str) -> String {
        let mut pos = self.keypad0[&'A'];
        let mut res = vec![];

        for action in actions.chars() {
            let dst = self.keypad0[&action];
            let diff = (dst.0 - pos.0, dst.1 - pos.1);

            match diff.1.cmp(&0) {
                std::cmp::Ordering::Less => {
                    (0..-diff.1).for_each(|_| res.push('^'));
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    (0..diff.1).for_each(|_| res.push('v'));
                }
            }
            match diff.0.cmp(&0) {
                std::cmp::Ordering::Less => {
                    (0..-diff.0).for_each(|_| res.push('<'));
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    (0..diff.0).for_each(|_| res.push('>'));
                }
            }
            res.push('A');

            pos = dst;
        }

        let mut res_str = String::new();
        res.iter().for_each(|ch| res_str.push(*ch));
        res_str
    }

    fn solve(&mut self) -> (i32, i32) {
        // println!("Input: {:#?}", self.input);
        println!("{}", self.simulate(&self.input[0]));

        println!("Test Name: {}", self.test_name);
        println!("Day 21, Part 1: {}", self.part1);
        println!("Day 21, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<String> {
        let raw =
            fs::read_to_string(format!("../data/day21/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned())
            .collect()
    }

    fn new(test_name: String) -> KeypadConundrum {
        let input = Self::read_input(&test_name);
        let keypad0 = HashMap::from([
            ('7', (0, 0)),
            ('8', (1, 0)),
            ('9', (2, 0)),
            ('4', (0, 1)),
            ('5', (1, 1)),
            ('6', (2, 1)),
            ('1', (0, 2)),
            ('2', (1, 2)),
            ('3', (2, 2)),
            ('0', (1, 3)),
            ('A', (2, 3)),
        ]);
        KeypadConundrum {
            input,
            keypad0,

            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct KeypadConundrum {
    input: Vec<String>,
    keypad0: HashMap<char, (i32, i32)>,

    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::KeypadConundrum;

    #[test]
    fn test_part1() {
        assert_eq!(KeypadConundrum::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(KeypadConundrum::new(String::from("test0")).solve().1, 0);
    }
}

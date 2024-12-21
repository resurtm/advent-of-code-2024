use std::{collections::HashMap, fs};

pub fn solve() {
    KeypadConundrum::new(String::from("test0")).solve();
    // KeypadConundrum::new(String::from("gh")).solve();
    // KeypadConundrum::new(String::from("google")).solve();
}

impl KeypadConundrum {
    fn simulate(actions: &str, keypad: &HashMap<char, (i32, i32)>) -> String {
        let mut pos = keypad[&'A'];
        let mut res = vec![];

        for action in actions.chars() {
            let dst = keypad[&action];
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

    fn code_nums(code: &str) -> i32 {
        code.chars()
            .filter(|&x| x >= '0' && x <= '9')
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
    }

    fn solve_internal(&self) -> i32 {
        let mut res = 0;
        for input in self.input.iter() {
            let pushes0 = Self::simulate(input, &self.keypad0);
            let pushes1 = Self::simulate(&pushes0, &self.keypad1);
            let pushes2 = Self::simulate(&pushes1, &self.keypad1);

            let a = pushes2.len() as i32;
            let b = Self::code_nums(input);

            println!("{} {} {}", a, b, a * b);
            res += a * b;
        }
        res
    }

    fn solve(&mut self) -> (i32, i32) {
        self.part1 = self.solve_internal();

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
        let keypad1 = HashMap::from([
            ('^', (1, 0)),
            ('A', (2, 0)),
            ('<', (0, 1)),
            ('v', (1, 1)),
            ('>', (2, 1)),
        ]);

        KeypadConundrum {
            input,
            keypad0,
            keypad1,

            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct KeypadConundrum {
    input: Vec<String>,
    keypad0: HashMap<char, (i32, i32)>,
    keypad1: HashMap<char, (i32, i32)>,

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

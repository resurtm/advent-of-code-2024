use std::fs;

pub fn solve() {
    KeypadConundrum::new(String::from("test0")).solve();
    KeypadConundrum::new(String::from("gh")).solve();
    KeypadConundrum::new(String::from("google")).solve();
}

impl KeypadConundrum {
    fn solve(&mut self) -> (i32, i32) {
        println!("Input: {:#?}", self.input);

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
        KeypadConundrum {
            input,

            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct KeypadConundrum {
    input: Vec<String>,

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

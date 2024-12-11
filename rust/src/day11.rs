use std::fs;

pub fn solve() {
    PlutonianPebbles::new(String::from("test0")).solve();
    // PlutonianPebbles::new(String::from("gh")).solve();
    // PlutonianPebbles::new(String::from("google")).solve();
}

impl PlutonianPebbles {
    fn new(test_name: String) -> PlutonianPebbles {
        PlutonianPebbles {
            input: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve(&mut self) -> (i32, i32) {
        println!("Input: {:?}", self.input);

        println!("Test Name: {}", self.test_name);
        println!("Day 11, Part 1: {}", self.part1);
        println!("Day 11, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<i32> {
        let raw =
            fs::read_to_string(format!("../data/day11/{}.txt", test_name)).expect("input file");
        raw.split_ascii_whitespace()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap_or(-1))
            .collect()
    }
}

struct PlutonianPebbles {
    input: Vec<i32>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::PlutonianPebbles;

    #[test]
    fn test_part1() {
        assert_eq!(
            PlutonianPebbles::new(String::from("test0")).solve().0,
            55312
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(PlutonianPebbles::new(String::from("test0")).solve().1, 0);
    }
}

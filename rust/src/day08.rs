use std::fs;

pub fn solve() {
    ResonantCollinearity::new(String::from("test0")).solve();
    // ResonantCollinearity::new(String::from("gh")).solve();
    // ResonantCollinearity::new(String::from("google")).solve();
}

impl ResonantCollinearity {
    fn new(test_name: String) -> ResonantCollinearity {
        ResonantCollinearity {
            map: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve(&mut self) -> (i128, i128) {
        println!("Test Name: {}", self.test_name);
        println!("Day 08, Part 1: {}", self.part1);
        println!("Day 08, Part 2: {}", self.part2);
        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<Vec<char>> {
        fs::read_to_string(format!("../data/day08/{}.txt", test_name))
            .expect("input file")
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x: &str| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().next().expect("input data"))
                    .collect()
            })
            .collect()
    }
}

struct ResonantCollinearity {
    map: Vec<Vec<char>>,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::ResonantCollinearity;

    #[test]
    fn test_part1() {
        assert_eq!(
            ResonantCollinearity::new(String::from("test0")).solve().0,
            14
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            ResonantCollinearity::new(String::from("test0")).solve().1,
            0
        );
    }
}

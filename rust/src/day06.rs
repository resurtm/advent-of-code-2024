use std::fs;

pub fn solve() {
    GuardGallivant::new(String::from("test0")).solve();
}

impl GuardGallivant {
    fn new(test_name: String) -> GuardGallivant {
        GuardGallivant {
            tiles: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve_internal(&mut self) {}

    fn solve(&mut self) -> (i32, i32) {
        self.solve_internal();
        println!("Test Name: {}", self.test_name);
        println!("Tiles: {:?}", self.tiles);
        println!("Day 06, Part 1: {}", self.part1);
        println!("Day 06, Part 2: {}", self.part2);
        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<Vec<char>> {
        let raw =
            fs::read_to_string(format!("../data/day06/{}.txt", test_name)).expect("input file");
        raw.split_ascii_whitespace()
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().next().expect("char"))
                    .collect()
            })
            .collect()
    }
}

struct GuardGallivant {
    tiles: Vec<Vec<char>>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::GuardGallivant;

    #[test]
    fn test_part1() {
        assert_eq!(GuardGallivant::new(String::from("test0")).solve().0, 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(GuardGallivant::new(String::from("test0")).solve().1, 0);
    }
}

use std::fs;

pub fn solve() {
    GardenGroups::new(String::from("test0")).solve();
    // GardenGroups::new(String::from("gh")).solve();
    // GardenGroups::new(String::from("google")).solve();
}

impl GardenGroups {
    fn new(test_name: String) -> GardenGroups {
        GardenGroups {
            input: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve(&mut self) -> (i32, i32) {
        println!("Input: {:?}", self.input);

        println!("Test Name: {}", self.test_name);
        println!("Day 12, Part 1: {}", self.part1);
        println!("Day 12, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<Vec<char>> {
        let raw =
            fs::read_to_string(format!("../data/day12/{}.txt", test_name)).expect("input file");
        raw.lines()
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().next().unwrap())
                    .collect()
            })
            .collect()
    }
}

struct GardenGroups {
    input: Vec<Vec<char>>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::GardenGroups;

    #[test]
    fn test_part1() {
        assert_eq!(GardenGroups::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(GardenGroups::new(String::from("test0")).solve().1, 0);
    }
}

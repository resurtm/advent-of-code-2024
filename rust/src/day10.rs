use std::fs;

pub fn solve() {
    HoofIt::new(String::from("test0")).solve();
    // HoofIt::new(String::from("gh")).solve();
    // HoofIt::new(String::from("google")).solve();
}

impl HoofIt {
    fn new(test_name: String) -> HoofIt {
        HoofIt {
            map: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve(&mut self) -> (i32, i32) {
        println!("{:?}", self.map);

        println!("Test Name: {}", self.test_name);
        println!("Day 10, Part 1: {}", self.part1);
        println!("Day 10, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<Vec<char>> {
        let raw =
            fs::read_to_string(format!("../data/day10/{}.txt", test_name)).expect("input file");
        raw.lines()
            .into_iter()
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().next().unwrap())
                    .collect()
            })
            .collect()
    }
}

struct HoofIt {
    map: Vec<Vec<char>>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::HoofIt;

    #[test]
    fn test_part1() {
        assert_eq!(HoofIt::new(String::from("test0")).solve().0, 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(HoofIt::new(String::from("test0")).solve().1, 0);
    }
}

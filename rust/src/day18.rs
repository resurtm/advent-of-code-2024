use std::fs;

pub fn solve() {
    RamRun::new(String::from("test0")).solve();
    RamRun::new(String::from("gh")).solve();
    RamRun::new(String::from("google")).solve();
}

impl RamRun {
    fn solve(&self) -> (i32, i32) {
        println!("Input: {:?}", self.coords);

        println!("Test Name: {}", self.test_name);
        println!("Day 18, Part 1: {}", self.part1);
        println!("Day 18, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<(i32, i32)> {
        let raw =
            fs::read_to_string(format!("../data/day18/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.trim().is_empty())
            .map(|x| {
                let p: Vec<&str> = x.split(",").collect();
                (p[0].parse().unwrap(), p[1].parse().unwrap())
            })
            .collect()
    }

    fn new(test_name: String) -> RamRun {
        let coords = Self::read_input(&test_name);
        RamRun {
            coords,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct RamRun {
    coords: Vec<(i32, i32)>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::RamRun;
    #[test]
    fn test_part1() {
        assert_eq!(RamRun::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(RamRun::new(String::from("test1")).solve().1, 0);
    }
}

use std::{collections::HashMap, fs};

pub fn solve() {
    ResonantCollinearity::new(String::from("test0")).solve();
    // ResonantCollinearity::new(String::from("gh")).solve();
    // ResonantCollinearity::new(String::from("google")).solve();
}

impl ResonantCollinearity {
    fn new(test_name: String) -> ResonantCollinearity {
        let map = Self::read_input(&test_name);
        ResonantCollinearity {
            w: map.len() as i32,
            h: map[0].len() as i32,
            map,
            ant: HashMap::new(),

            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve_internal(&mut self) {
        println!("{:?}", self.ant);
    }

    fn solve(&mut self) -> (i128, i128) {
        self.find_ants();
        self.solve_internal();
        println!("Test Name: {}", self.test_name);
        println!("Day 08, Part 1: {}", self.part1);
        println!("Day 08, Part 2: {}", self.part2);
        (self.part1, self.part2)
    }

    fn find_ants(&mut self) {
        for i in 0..self.w {
            for j in 0..self.h {
                let ch = self.map[i as usize][j as usize];
                if ch == '.' {
                    continue;
                }
                if self.ant.contains_key(&ch) {
                    self.ant.get_mut(&ch).unwrap().push((i, j));
                } else {
                    self.ant.insert(ch, vec![(i, j)]);
                }
            }
        }
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
    w: i32,
    h: i32,
    map: Vec<Vec<char>>,
    ant: HashMap<char, Vec<(i32, i32)>>,

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

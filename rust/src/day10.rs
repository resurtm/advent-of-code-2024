use std::{collections::HashSet, fs};

pub fn solve() {
    HoofIt::new(String::from("test0")).solve();
    // HoofIt::new(String::from("test1")).solve();
    // HoofIt::new(String::from("test2")).solve();
    HoofIt::new(String::from("gh")).solve();
    HoofIt::new(String::from("google")).solve();
}

impl HoofIt {
    fn new(test_name: String) -> HoofIt {
        let map = Self::read_input(&test_name);
        let w = map.len() as i32;
        let h = map[0].len() as i32;
        HoofIt {
            map,
            w,
            h,
            peaks: HashSet::new(),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn traverse(&mut self, i: i32, j: i32, curr: i32) {
        if curr == 9 {
            self.peaks.insert((i, j));
            return;
        }
        let dirs: Vec<(i32, i32)> = vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
            .iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < self.w && *y < self.h)
            .map(|(x, y)| (*x, *y))
            .collect();
        for (x, y) in dirs.iter() {
            if self.map[*x as usize][*y as usize] == curr + 1 {
                self.traverse(*x, *y, curr + 1);
            }
        }
    }

    fn solve_internal(&mut self) {
        for i in 0..self.w {
            for j in 0..self.h {
                if self.map[i as usize][j as usize] == 0 {
                    self.peaks.clear();
                    self.traverse(i, j, 0);
                    self.part1 += self.peaks.len() as i32;
                }
            }
        }
    }

    fn solve(&mut self) -> (i32, i32) {
        self.solve_internal();

        println!("Test Name: {}", self.test_name);
        println!("Day 10, Part 1: {}", self.part1);
        println!("Day 10, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<Vec<i32>> {
        let raw =
            fs::read_to_string(format!("../data/day10/{}.txt", test_name)).expect("input file");
        raw.lines()
            .into_iter()
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().next().unwrap().to_digit(10).unwrap_or(100) as i32)
                    .collect()
            })
            .collect()
    }
}

struct HoofIt {
    map: Vec<Vec<i32>>,
    w: i32,
    h: i32,
    peaks: HashSet<(i32, i32)>,
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

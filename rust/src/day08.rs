use std::{collections::HashMap, fs};

pub fn solve() {
    ResonantCollinearity::new(String::from("test0")).solve();
    ResonantCollinearity::new(String::from("gh")).solve();
    ResonantCollinearity::new(String::from("google")).solve();
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
        for (ch, pos) in self.ant.iter() {
            for m in 0..pos.len() {
                for n in m + 1..pos.len() {
                    let delta = ((pos[m].0 - pos[n].0), (pos[m].1 - pos[n].1));
                    let its = vec![
                        (pos[m].0 - delta.0, pos[m].1 - delta.1),
                        (pos[m].0 + delta.0, pos[m].1 + delta.1),
                        (pos[n].0 - delta.0, pos[n].1 - delta.1),
                        (pos[n].0 + delta.0, pos[n].1 + delta.1),
                    ];
                    for (x, y) in its.iter() {
                        if *x < 0 || *y < 0 || *x >= self.w || *y >= self.h {
                            continue;
                        }
                        if self.map[*x as usize][*y as usize] == *ch {
                            continue;
                        }
                        self.map[*x as usize][*y as usize] = '#';
                    }
                }
            }
            // self.print_map();
        }
    }

    fn find_fills(&self) -> i32 {
        let mut ret = 0;
        for i in 0..self.w {
            for j in 0..self.h {
                if self.map[i as usize][j as usize] == '#' {
                    ret += 1;
                }
            }
        }
        ret
    }

    fn solve(&mut self) -> (i32, i32) {
        self.find_ants();
        self.solve_internal();
        self.part1 = self.find_fills();
        // self.print_map();
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

    fn print_map(&self) {
        println!("{}", "-".repeat(self.w as usize));
        for i in 0..self.w {
            for j in 0..self.h {
                print!("{}", self.map[i as usize][j as usize]);
            }
            println!("");
        }
        println!("{}", "-".repeat(self.w as usize));
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

    part1: i32,
    part2: i32,
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

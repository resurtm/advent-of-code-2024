use std::collections::HashSet;
use std::fs;

pub fn solve() {
    GuardGallivant::new(String::from("test0")).solve();
    GuardGallivant::new(String::from("gh")).solve();
    GuardGallivant::new(String::from("google")).solve();
}

impl GuardGallivant {
    fn new(test_name: String) -> GuardGallivant {
        let tiles = Self::read_input(&test_name);
        let w = tiles.len() as i32;
        let h = tiles[0].len() as i32;
        GuardGallivant {
            tiles,
            w,
            h,
            pos: (0, 0),
            dir: 0,
            points: HashSet::new(),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve_internal(&mut self) {
        self.find_start();

        loop {
            self.points.insert(self.pos);

            // turn
            match self.dir {
                0 => {
                    if self.pos.0 > 0
                        && self.tiles[self.pos.0 as usize - 1][self.pos.1 as usize] == '#'
                    {
                        self.dir = 1;
                    }
                }
                1 => {
                    if self.pos.1 < self.h - 1
                        && self.tiles[self.pos.0 as usize][self.pos.1 as usize + 1] == '#'
                    {
                        self.dir = 2;
                    }
                }
                2 => {
                    if self.pos.0 < self.w - 1
                        && self.tiles[self.pos.0 as usize + 1][self.pos.1 as usize] == '#'
                    {
                        self.dir = 3;
                    }
                }
                3 => {
                    if self.pos.1 > 0
                        && self.tiles[self.pos.0 as usize][self.pos.1 as usize - 1] == '#'
                    {
                        self.dir = 0;
                    }
                }
                _ => panic!("invalid dir"),
            }

            // advance/move
            match self.dir {
                0 => self.pos.0 -= 1,
                1 => self.pos.1 += 1,
                2 => self.pos.0 += 1,
                3 => self.pos.1 -= 1,
                _ => panic!("invalid dir"),
            }

            if self.pos.0 < 0 || self.pos.0 >= self.w || self.pos.1 < 0 || self.pos.1 >= self.h {
                break;
            }
        }

        self.part1 = self.points.len() as i32;
    }

    fn solve(&mut self) -> (i32, i32) {
        self.solve_internal();
        println!("Test Name: {}", self.test_name);
        println!("Day 06, Part 1: {}", self.part1);
        println!("Day 06, Part 2: {}", self.part2);
        (self.part1, self.part2)
    }

    fn find_start(&mut self) {
        for x in 0..self.w {
            for y in 0..self.h {
                if self.tiles[x as usize][y as usize] == '^' {
                    self.pos = (x, y);
                    return;
                }
            }
        }
        panic!("invalid input")
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
    w: i32,
    h: i32,
    pos: (i32, i32),
    dir: i32,
    points: HashSet<(i32, i32)>,
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

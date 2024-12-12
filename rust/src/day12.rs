use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn solve() {
    GardenGroups::new(String::from("test0")).solve();
    // GardenGroups::new(String::from("gh")).solve();
    // GardenGroups::new(String::from("google")).solve();
}

impl GardenGroups {
    fn traverse_regs(&mut self) {
        for (ch, reg) in self.regs.iter() {
            let mut viz: HashSet<(i32, i32, i32)> = HashSet::new();
            for (ii, jj) in reg.iter() {
                let i = *ii;
                let j = *jj;
                let dirs = vec![(i - 1, j, 0), (i + 1, j, 1), (i, j - 1, 2), (i, j + 1, 3)];
                dirs.iter()
                    .filter(|(x, y, _)| *x >= 0 && *y >= 0 && *x < self.w && *y < self.h)
                    .filter(|(x, y, _)| self.inp[*x as usize][*y as usize] != *ch)
                    .for_each(|(mm, nn, dir)| {
                        viz.insert((*mm, *nn, *dir));
                    });
                dirs.iter()
                    .filter(|(x, y, _)| !(*x >= 0 && *y >= 0 && *x < self.w && *y < self.h))
                    .for_each(|(mm, nn, dir)| {
                        viz.insert((*mm, *nn, *dir));
                    });
            }
            self.part1 += (viz.len() * reg.len()) as i32;
        }
    }

    fn build_reg(&mut self, ch: char, m: i32, n: i32) {
        let mut reg: HashSet<(i32, i32)> = HashSet::new();
        let mut q = VecDeque::new();
        q.push_front((m, n));
        loop {
            if let Some((i, j)) = q.pop_front() {
                reg.insert((i, j));
                self.viz.insert((i, j));
                vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < self.w && *y < self.h)
                    .filter(|(x, y)| self.inp[*x as usize][*y as usize] == ch)
                    .filter(|x| !reg.contains(x))
                    .for_each(|x| q.push_back(*x));
            } else {
                break;
            }
        }
        self.regs.push((ch, reg));
    }

    fn build_regs(&mut self) {
        for i in 0..self.w {
            for j in 0..self.h {
                if !self.viz.contains(&(i, j)) {
                    self.build_reg(self.inp[i as usize][j as usize], i, j);
                }
            }
        }
    }

    fn solve(&mut self) -> (i32, i32) {
        self.build_regs();
        self.traverse_regs();

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

    fn new(test_name: String) -> GardenGroups {
        let inp = Self::read_input(&test_name);
        let w = inp.len() as i32;
        let h = inp[0].len() as i32;

        GardenGroups {
            inp,
            w,
            h,

            viz: HashSet::new(),
            regs: Vec::new(),

            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct GardenGroups {
    inp: Vec<Vec<char>>,
    w: i32,
    h: i32,

    viz: HashSet<(i32, i32)>,
    regs: Vec<(char, HashSet<(i32, i32)>)>,

    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::GardenGroups;

    #[test]
    fn test_part1() {
        assert_eq!(GardenGroups::new(String::from("test0")).solve().0, 140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(GardenGroups::new(String::from("test0")).solve().1, 0);
    }
}

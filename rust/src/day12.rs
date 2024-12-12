use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn solve() {
    GardenGroups::new(String::from("test0")).solve();
    GardenGroups::new(String::from("test1")).solve();
    GardenGroups::new(String::from("test2")).solve();
    GardenGroups::new(String::from("gh")).solve();
    GardenGroups::new(String::from("google")).solve();
}

impl GardenGroups {
    fn count_sides_internal(&self, points: &HashSet<(i32, i32)>) -> i32 {
        let mut sides = 0;
        for (ii, jj) in points.iter() {
            let i = *ii;
            let j = *jj;

            let mut t = false;
            let mut r = false;
            let mut b = false;
            let mut l = false;

            let mut tl = false;
            let mut tr = false;
            let mut br = false;
            let mut bl = false;

            vec![
                (i, j - 1, 0),
                (i + 1, j, 1),
                (i, j + 1, 2),
                (i - 1, j, 3),
                (i - 1, j - 1, 4),
                (i + 1, j - 1, 5),
                (i + 1, j + 1, 6),
                (i - 1, j + 1, 7),
            ]
            .iter()
            .map(|(x, y, d)| {
                (
                    if *x >= 0 && *y >= 0 && *x < self.w && *y < self.h {
                        points.contains(&(*x, *y))
                    } else {
                        false
                    },
                    *d,
                )
            })
            .for_each(|(st, dir)| match dir {
                0 => t = st,
                1 => r = st,
                2 => b = st,
                3 => l = st,
                4 => tl = st,
                5 => tr = st,
                6 => br = st,
                7 => bl = st,
                _ => panic!("not expected"),
            });

            let not_attached: i32 = if t { 0 } else { 1 }
                + if r { 0 } else { 1 }
                + if b { 0 } else { 1 }
                + if l { 0 } else { 1 };

            sides += match not_attached {
                4 => 4,
                3 => 2,
                2 => {
                    if t && b || l && r {
                        0
                    } else {
                        1
                    }
                }
                _ => 0,
            };

            if !tl && t && l {
                sides += 1;
            }
            if !tr && t && r {
                sides += 1;
            }
            if !bl && b && l {
                sides += 1;
            }
            if !br && b && r {
                sides += 1;
            }
        }
        sides
    }

    fn count_sides(&mut self) {
        for (_, reg) in self.regs.iter() {
            let sides = self.count_sides_internal(&reg);
            self.part2 += sides * reg.len() as i32;
        }
    }

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

        self.viz.insert((m, n));
        reg.insert((m, n));
        q.push_front((m, n));

        loop {
            if let Some((i, j)) = q.pop_front() {
                let its: Vec<(i32, i32)> = vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < self.w && *y < self.h)
                    .filter(|(x, y)| self.inp[*x as usize][*y as usize] == ch)
                    .filter(|x| !reg.contains(x))
                    .map(|(x, y)| (*x, *y))
                    .collect();
                for it in its.iter() {
                    self.viz.insert(*it);
                    reg.insert(*it);
                    q.push_back(*it);
                }
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
        self.count_sides();

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
        assert_eq!(GardenGroups::new(String::from("test1")).solve().0, 772);
        assert_eq!(GardenGroups::new(String::from("test2")).solve().0, 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(GardenGroups::new(String::from("test0")).solve().1, 80);
        assert_eq!(GardenGroups::new(String::from("test1")).solve().1, 436);
        assert_eq!(GardenGroups::new(String::from("test4")).solve().1, 368);
        assert_eq!(GardenGroups::new(String::from("test5")).solve().1, 236);
    }
}

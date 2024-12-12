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

        let mut c: HashSet<i32> = HashSet::new();
        for x in 0..self.w {
            let its: Vec<i32> = points
                .iter()
                .filter(|it| it.0 == x)
                .map(|it| it.1)
                .collect();

            let mut curr = -1;
            let mut ch = HashSet::new();
            for (idx, it) in its.iter().enumerate() {
                if *it != curr {
                    curr = *it;
                    ch.insert(idx as i32);
                }
            }
            if ch.len() > 0 && ch.len() % 2 != 0 {
                ch.insert(its.len() as i32);
            }

            if ch.len() > 0 {
                let diff: HashSet<i32> = ch.iter().filter(|x| !c.contains(x)).map(|x| *x).collect();
                sides += diff.len() as i32 / 2;
            }
            c = ch.iter().map(|x| *x).collect();
        }

        let mut c: HashSet<i32> = HashSet::new();
        for y in 0..self.h {
            let its: Vec<i32> = points
                .iter()
                .filter(|it| it.1 == y)
                .map(|it| it.0)
                .collect();

            let mut curr = -1;
            let mut ch = HashSet::new();
            for (idx, it) in its.iter().enumerate() {
                if *it != curr {
                    curr = *it;
                    ch.insert(idx as i32);
                }
            }
            if ch.len() > 0 && ch.len() % 2 != 0 {
                ch.insert(its.len() as i32);
            }

            if ch.len() > 0 {
                let diff: HashSet<i32> = ch.iter().filter(|x| !c.contains(x)).map(|x| *x).collect();
                sides += diff.len() as i32 / 2;
            }
            c = ch.iter().map(|x| *x).collect();
        }

        // for y in 0..self.h {
        //     let its: Vec<i32> = points
        //         .iter()
        //         .filter(|it| it.1 == y)
        //         .map(|it| it.0)
        //         .collect();
        // }

        sides
    }

    fn count_sides(&mut self) {
        for (ch, reg) in self.regs.iter() {
            let sides = self.count_sides_internal(&reg);
            println!("{} {}", ch, sides);
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
    }
}

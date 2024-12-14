use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn solve() {
    // RestroomRedoubt::new(11, 7, String::from("test0")).solve();
    RestroomRedoubt::new(101, 103, String::from("gh")).solve();
    RestroomRedoubt::new(101, 103, String::from("google")).solve();
    // RestroomRedoubt::new(15, 15, String::from("test1")).solve();
}

impl RestroomRedoubt {
    fn simulate(&mut self, seconds: i32, do_check: bool) -> i32 {
        for second in 0..seconds {
            for robot in self.robots.iter_mut() {
                robot.p.0 += robot.v.0;
                robot.p.1 += robot.v.1;
                Self::limit(robot, self.w, self.h);
            }
            if do_check {
                self.build_map();
                if self.check_tree() {
                    self.print();
                    return second + 101;
                }
            }
        }
        0
    }

    fn check_tree(&self) -> bool {
        // let mut areas = 0;
        let mut found = false;
        let mut viz: HashSet<(i32, i32)> = HashSet::new();
        for st in self.robots.iter() {
            if viz.contains(&(st.p.0, st.p.1)) {
                continue;
            }
            let mut area: HashSet<(i32, i32)> = HashSet::new();
            let mut deq: VecDeque<(i32, i32)> = VecDeque::new();
            deq.push_back((st.p.0, st.p.1));
            while !deq.is_empty() {
                let (m, n) = deq.pop_front().unwrap();
                vec![
                    (m - 1, n - 1),
                    (m - 1, n),
                    (m - 1, n + 1),
                    (m + 1, n - 1),
                    (m + 1, n),
                    (m + 1, n + 1),
                    (m, n - 1),
                    (m, n + 1),
                ]
                .iter()
                .for_each(|(u, v)| {
                    let tmp = (*u, *v);
                    if tmp.0 >= 0
                        && tmp.1 >= 0
                        && tmp.0 < self.w
                        && tmp.1 < self.h
                        && self.map[tmp.0 as usize][tmp.1 as usize]
                        && !viz.contains(&tmp)
                        && !area.contains(&tmp)
                    {
                        viz.insert(tmp);
                        area.insert(tmp);
                        deq.push_back(tmp);
                    }
                });
            }
            // areas += 1;
            if area.len() >= self.robots.len() / 3 {
                found = true
            }
        }
        found
    }

    fn limit(robot: &mut Robot, w: i32, h: i32) {
        if robot.p.0 < 0 {
            robot.p.0 = robot.p.0 + w;
        } else if robot.p.0 >= w {
            robot.p.0 = robot.p.0 - w;
        }
        if robot.p.1 < 0 {
            robot.p.1 = robot.p.1 + h;
        } else if robot.p.1 >= h {
            robot.p.1 = robot.p.1 - h;
        }
    }

    fn calculate_safety(&self) -> i32 {
        let quads = vec![
            (
                0,
                0,
                (self.w as f32 / 2.0).floor() as i32,
                (self.h as f32 / 2.0).floor() as i32,
            ),
            (
                (self.w as f32 / 2.0).ceil() as i32,
                (self.h as f32 / 2.0).ceil() as i32,
                self.w,
                self.h,
            ),
            (
                (self.w as f32 / 2.0).ceil() as i32,
                0,
                self.w,
                (self.h as f32 / 2.0).floor() as i32,
            ),
            (
                0,
                (self.h as f32 / 2.0).ceil() as i32,
                (self.w as f32 / 2.0).floor() as i32,
                self.h,
            ),
        ];
        let mut res = 1;
        for quad in quads.iter() {
            let mut loc = 0;
            for robot in self.robots.iter() {
                if robot.p.0 >= quad.0
                    && robot.p.0 < quad.2
                    && robot.p.1 >= quad.1
                    && robot.p.1 < quad.3
                {
                    loc += 1;
                }
            }
            res *= loc;
        }
        res
    }

    fn solve(&mut self) -> (i32, i32) {
        self.simulate(100, false);
        self.part1 = self.calculate_safety();
        self.part2 = self.simulate(10_000, true);

        println!("Test Name: {}", self.test_name);
        println!("Day 14, Part 1: {}", self.part1);
        println!("Day 14, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn build_map(&mut self) {
        self.map.clear();
        self.map.resize(self.w as usize, vec![]);
        for it in self.map.iter_mut() {
            it.resize(self.h as usize, false);
        }
        for robot in self.robots.iter() {
            self.map[robot.p.0 as usize][robot.p.1 as usize] = true;
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{}", "-".repeat(self.h as usize));
        for y in 0..self.h {
            for x in 0..self.w {
                let mut found = false;
                for robot in self.robots.iter() {
                    if robot.p.0 == x && robot.p.1 == y {
                        found = true;
                        break;
                    }
                }
                print!("{}", if found { '@' } else { '.' });
            }
            println!();
        }
    }

    fn read_input(test_name: &str) -> Vec<Robot> {
        let raw =
            fs::read_to_string(format!("../data/day14/{}.txt", test_name)).expect("input file");
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut robot = Robot::new();
        let mut robots = Vec::new();
        re.captures_iter(&raw).for_each(|x| {
            robot.p = (
                x.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                x.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            );
            robot.v = (
                x.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                x.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            );
            robots.push(robot.clone());
            robot = Robot::new();
        });
        robots
    }

    fn new(w: i32, h: i32, test_name: String) -> RestroomRedoubt {
        let robots = Self::read_input(&test_name);
        RestroomRedoubt {
            robots,
            map: Vec::new(),
            w,
            h,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

impl Robot {
    fn new() -> Robot {
        Robot {
            p: (0, 0),
            v: (0, 0),
        }
    }
}

struct RestroomRedoubt {
    robots: Vec<Robot>,
    map: Vec<Vec<bool>>,
    w: i32,
    h: i32,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[derive(Clone, Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

#[cfg(test)]
mod tests {
    use super::RestroomRedoubt;

    #[test]
    fn test_part1() {
        assert_eq!(
            RestroomRedoubt::new(11, 7, String::from("test0")).solve().0,
            12
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            RestroomRedoubt::new(11, 7, String::from("test0")).solve().1,
            104
        );
    }
}

use regex::Regex;
use std::fs;

pub fn solve() {
    RestroomRedoubt::new(11, 7, String::from("test0")).solve();
    RestroomRedoubt::new(101, 103, String::from("gh")).solve();
    RestroomRedoubt::new(101, 103, String::from("google")).solve();
}

impl RestroomRedoubt {
    fn simulate(&mut self) {
        for _ in 0..100 {
            for robot in self.robots.iter_mut() {
                robot.p.0 += robot.v.0;
                robot.p.1 += robot.v.1;
                Self::limit(robot, self.w, self.h);
            }
        }
        // self.print();
    }

    fn limit(robot: &mut Robot, w: i32, h: i32) {
        if robot.p.0 < 0 {
            robot.p.0 = w + robot.p.0;
        }
        if robot.p.1 < 0 {
            robot.p.1 = h + robot.p.1;
        }
        if robot.p.0 >= w {
            robot.p.0 = robot.p.0 - w;
        }
        if robot.p.1 >= h {
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
        self.simulate();
        self.part1 = self.calculate_safety();

        println!("Test Name: {}", self.test_name);
        println!("Day 14, Part 1: {}", self.part1);
        println!("Day 14, Part 2: {}", self.part2);

        (self.part1, self.part2)
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
            0
        );
    }
}

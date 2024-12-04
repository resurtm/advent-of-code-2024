use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("test1");
    solve_internal("test2");
    solve_internal("gh");
    solve_internal("google");
}

fn solve_internal(test_name: &str) -> (i32, i32) {
    let mut mat = Mat::new(
        &fs::read_to_string(format!("../data/day04/{}.txt", test_name)).expect("input file"),
    );
    mat.check();
    println!("Test Name: {}", test_name);
    println!("Day 01, Part 1: {}", mat.part1);
    println!("Day 01, Part 2: {}", mat.part2);
    (mat.part1, mat.part2)
}

struct Mat {
    mat: Vec<Vec<char>>,
    w: i32,
    h: i32,
    part1: i32,
    part2: i32,
}

impl Mat {
    fn traverse(&mut self, sx: i32, sy: i32, dir: i32, curr_char: char) {
        if curr_char == 'S' {
            self.part1 += 1;
            return;
        }
        let (x, y) = match dir {
            0 => (sx - 1, sy - 1),
            1 => (sx - 1, sy),
            2 => (sx - 1, sy + 1),
            3 => (sx, sy - 1),
            4 => (sx + 1, sy - 1),
            5 => (sx + 1, sy),
            6 => (sx + 1, sy + 1),
            7 => (sx, sy + 1),
            _ => return,
        };
        if !(x >= 0 && y >= 0 && x < self.w && y < self.h) {
            return;
        }
        let next_char = Self::next_char(curr_char);
        if self.mat[x as usize][y as usize] == next_char {
            self.traverse(x, y, dir, next_char);
        }
    }

    fn next_char(curr_char: char) -> char {
        match curr_char {
            'X' => 'M',
            'M' => 'A',
            'A' => 'S',
            'S' => panic!("not expected"),
            _ => panic!("not expected"),
        }
    }

    fn check(&mut self) {
        // part 1
        for i in 0..self.w {
            for j in 0..self.h {
                if self.mat[i as usize][j as usize] == 'X' {
                    for k in 0..8 {
                        self.traverse(i, j, k, 'X');
                    }
                }
            }
        }
        // part 2
        for i in 1..self.w - 1 {
            for j in 1..self.h - 1 {
                if self.mat[i as usize][j as usize] == 'A' {
                    self.diagonals(i as usize, j as usize);
                }
            }
        }
    }

    fn diagonals(&mut self, x: usize, y: usize) {
        let a = self.mat[x - 1][y - 1] == 'M'
            && self.mat[x + 1][y - 1] == 'M'
            && self.mat[x - 1][y + 1] == 'S'
            && self.mat[x + 1][y + 1] == 'S';
        let b = self.mat[x - 1][y - 1] == 'S'
            && self.mat[x + 1][y - 1] == 'S'
            && self.mat[x - 1][y + 1] == 'M'
            && self.mat[x + 1][y + 1] == 'M';
        let c = self.mat[x - 1][y - 1] == 'M'
            && self.mat[x - 1][y + 1] == 'M'
            && self.mat[x + 1][y - 1] == 'S'
            && self.mat[x + 1][y + 1] == 'S';
        let d = self.mat[x - 1][y - 1] == 'S'
            && self.mat[x - 1][y + 1] == 'S'
            && self.mat[x + 1][y - 1] == 'M'
            && self.mat[x + 1][y + 1] == 'M';
        if a || b || c || d {
            self.part2 += 1;
        }
    }

    fn new(input: &str) -> Mat {
        let mat = input
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.chars().next().expect("char"))
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        let w = mat.len() as i32;
        let h = mat[0].len() as i32;
        Mat {
            mat,
            w,
            h,
            part1: 0,
            part2: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test1").0, 18);
    }

    #[test]
    fn test_part3() {
        assert_eq!(solve_internal("test2").0, 4);
    }

    #[test]
    fn test_part4() {
        assert_eq!(solve_internal("test3").1, 1);
    }

    #[test]
    fn test_part5() {
        assert_eq!(solve_internal("test4").1, 9);
    }
}

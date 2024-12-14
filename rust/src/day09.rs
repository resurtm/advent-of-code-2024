use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

pub fn solve() {
    DiskFragmenter::new(String::from("test0")).solve();
    DiskFragmenter::new(String::from("gh")).solve();
    DiskFragmenter::new(String::from("google")).solve();
}

impl Chunk {
    fn index(&mut self, index: i128) {
        let size = self.len();
        self.start = index;
        self.end = index + size;
    }

    fn len(&self) -> i128 {
        return self.end - self.start;
    }
}

impl DiskFragmenter {
    fn new(test_name: String) -> DiskFragmenter {
        DiskFragmenter {
            input: Self::read_input(&test_name),
            deque: VecDeque::new(),
            chunks: Vec::new(),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn build_deque(&mut self) {
        let mut cnt = 0;
        for (idx, ch) in self.input.chars().enumerate() {
            for _ in 0..ch.to_digit(10).unwrap_or(0) {
                self.deque.push_back(if idx % 2 == 0 { cnt } else { -1 });
            }
            if idx % 2 == 0 {
                cnt += 1;
            }
        }
    }

    fn solve_part2(&mut self) {
        let mut chunks = self.chunks.clone();
        let mut i = (chunks.len() - 1) as i128;

        loop {
            let (idx, chunk) = chunks.iter().find_position(|x| x.id == i).unwrap();

            for j in 1..idx + 1 {
                let fr_start = chunks[j - 1].end;
                let fr_end = chunks[j].start;
                let fr_len = fr_end - fr_start;

                if fr_len >= chunk.len() {
                    let mut chunk = chunk.clone();
                    chunk.index(fr_start);
                    chunks.remove(idx);
                    chunks.insert(j, chunk);
                    break;
                }
            }

            if i == 0 {
                break;
            }

            i -= 1;
        }

        for chunk in chunks.iter() {
            let mut id = chunk.start;
            for _ in 0..chunk.len() {
                self.part2 += id * chunk.id;
                id += 1;
            }
        }
    }

    fn build_chunks(&mut self) {
        let mut i = 0i128;
        let mut is = true;
        let mut id = 0i128;

        for ch in self.input.lines().next().unwrap().chars() {
            let sz = ch.to_digit(10).unwrap() as i128;
            let end = i + sz;
            if is {
                self.chunks.push(Chunk { id, start: i, end });
                id += 1
            }
            is = is ^ true;
            i = end;
        }
    }

    fn solve_part1(&mut self) {
        let mut deque = VecDeque::new();
        'outer: loop {
            if let Some(it) = self.deque.pop_front() {
                if it == -1 {
                    #[allow(unused_assignments)]
                    let mut x1 = 0;
                    'inner: loop {
                        if let Some(x2) = self.deque.pop_back() {
                            if x2 == -1 {
                                continue;
                            }
                            x1 = x2;
                            break 'inner;
                        } else {
                            break 'outer;
                        }
                    }
                    deque.push_back(x1);
                } else {
                    deque.push_back(it);
                }
            } else {
                break;
            }
        }

        for (idx, it) in deque.iter().enumerate() {
            self.part1 += idx as i128 * it;
        }
    }

    fn solve(&mut self) -> (i128, i128) {
        self.build_deque();
        self.build_chunks();

        self.solve_part1();
        self.solve_part2();

        println!("Test Name: {}", self.test_name);
        println!("Day 09, Part 1: {}", self.part1);
        println!("Day 09, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> String {
        fs::read_to_string(format!("../data/day09/{}.txt", test_name)).expect("input file")
    }
}

#[derive(Clone)]
struct Chunk {
    id: i128,
    start: i128,
    end: i128,
}

struct DiskFragmenter {
    input: String,
    deque: VecDeque<i128>,
    chunks: Vec<Chunk>,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::DiskFragmenter;

    #[test]
    fn test_part1() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().0, 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().1, 2858);
    }
}

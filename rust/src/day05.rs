use std::fs;

pub fn solve() {
    PrintQueue::new(String::from("test0")).solve();
    PrintQueue::new(String::from("gh")).solve();
    PrintQueue::new(String::from("google")).solve();
}

impl PrintQueue {
    fn new(test_name: String) -> PrintQueue {
        let (orders, pages) = Self::read_input(&test_name);
        PrintQueue {
            orders,
            pages,
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve(&mut self) -> (i32, i32) {
        self.solve_internal();
        println!("Test Name: {}", self.test_name);
        println!("Day 05, Part 1: {}", self.part1);
        println!("Day 05, Part 2: {}", self.part2);
        (self.part1, self.part2)
    }

    fn solve_internal(&mut self) {
        for pages in self.pages.iter_mut() {
            let mut valid = true;
            for i in 0..pages.len() {
                for j in i + 1..pages.len() {
                    if self.orders.contains(&(pages[j], pages[i])) {
                        valid = false;
                        pages.swap(i, j);
                    }
                }
            }
            if valid {
                self.part1 += pages[pages.len() / 2];
            } else {
                self.part2 += pages[pages.len() / 2];
            }
        }
    }

    fn read_input(test_name: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let raw =
            fs::read_to_string(format!("../data/day05/{}.txt", test_name)).expect("input file");
        let parts = raw.split("\n\n").collect::<Vec<&str>>();
        let orders = parts
            .get(0)
            .expect("orders")
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.split("|").collect::<Vec<&str>>())
            .map(|x| {
                (
                    x.get(0)
                        .expect("left order")
                        .parse::<i32>()
                        .expect("left i32"),
                    x.get(1)
                        .expect("right order")
                        .parse::<i32>()
                        .expect("right i32"),
                )
            })
            .collect::<Vec<(i32, i32)>>();
        let pages = parts
            .get(1)
            .expect("pages")
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split(",")
                    .map(|x| x.parse::<i32>().expect("page i32"))
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();
        (orders, pages)
    }
}

struct PrintQueue {
    orders: Vec<(i32, i32)>,
    pages: Vec<Vec<i32>>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::PrintQueue;

    #[test]
    fn test_part1() {
        assert_eq!(PrintQueue::new(String::from("test0")).solve().0, 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(PrintQueue::new(String::from("test0")).solve().1, 123);
    }
}

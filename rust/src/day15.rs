use std::fs;

pub fn solve() {
    WarehouseWoes::new(String::from("test0")).solve();
    // WarehouseWoes::new(String::from("gh")).solve();
    // WarehouseWoes::new(String::from("google")).solve();
}

impl WarehouseWoes {
    fn solve(&mut self) -> (i32, i32) {
        println!("Map:\n{:?}", self.map);
        println!("Moves:\n{:?}", self.moves);

        println!("Test Name: {}", self.test_name);
        println!("Day 15, Part 1: {}", self.part1);
        println!("Day 15, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> (Vec<Vec<char>>, Vec<char>) {
        let raw =
            fs::read_to_string(format!("../data/day15/{}.txt", test_name)).expect("input file");
        let parts: Vec<&str> = raw.split("\n\n").collect();

        let map = parts
            .first()
            .unwrap()
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect())
            .collect();
        let moves = parts.get(1).unwrap().chars().collect();

        (map, moves)
    }

    fn new(test_name: String) -> WarehouseWoes {
        let (map, moves) = Self::read_input(&test_name);
        let w = map.len() as i32;
        let h = map[0].len() as i32;
        WarehouseWoes {
            map,
            moves,
            w,
            h,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct WarehouseWoes {
    map: Vec<Vec<char>>,
    moves: Vec<char>,
    w: i32,
    h: i32,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::WarehouseWoes;

    #[test]
    fn test_part1() {
        assert_eq!(WarehouseWoes::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(WarehouseWoes::new(String::from("test0")).solve().1, 0);
    }
}

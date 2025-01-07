use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashSet;
pub fn solve(input: File) {
    let formatted = format_input(input);

    let part1 = solve_part1(&formatted.clone());
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&formatted.clone());
    println!("The answer for part 2 is: {}", part2);
}

pub fn solve_part1(input: &Vec<Vec<String>>) -> i32 {
    let mut guard = find_guard(&input);
    guard.predict_path(&input);
    get_distinct_positions(&guard.path).len() as i32
}

pub fn solve_part2(input: &Vec<Vec<String>>) -> i32 {
    let looped = find_loops(&input);

    looped.len() as i32
}

fn format_input(input: File) -> Vec<Vec<String>> {
    let reader = io::BufReader::new(input);
    let content: Vec<Vec<String>> = reader.lines()
        .map(|line| {
            line
                .unwrap()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    content
}

fn find_guard(input: &Vec<Vec<String>>) -> Guard {
    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == "^" {
                return Guard { 
                    direction: Direction::Up, 
                    position: (i, j), 
                    path: vec![Vector { x: i, y: j, dir: Direction::Up }], 
                    bounds: (input.len(), row.len()), 
                };
            }
        }
    }
    Guard { direction: Direction::Up, position: (0, 0), path: vec![Vector { x: 0, y: 0, dir: Direction::Up }], bounds: (0, 0) }
}

fn get_distinct_positions(path: &Vec<Vector>) -> Vec<(usize, usize)> {
    path.iter().map(|v| (v.x, v.y)).collect::<HashSet<(usize, usize)>>().into_iter().collect()
}

fn find_loops(input: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    let guard = find_guard(&input);
    let mut looped = Vec::new();
    let mut input = input.clone();
    for i in 0..input.len() {
        println!("i: {}", i);
        for j in 0..input[i].len() {
            println!("    j: {}", j);
            if input[i][j] != "."  {
                continue;
            }
            let original = input[i][j].clone();
            input[i][j] = String::from("#");
            let mut trial = guard.clone().predict_path(&input);
            if trial.is_loop() {
                println!("     - loop found\n");
                looped.push((i, j));
            }
            input[i][j] = String::from(".");
        }
    }
    looped
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_forward(&self, position: (usize, usize)) -> Vector {
        match self {
            Direction::Up => Vector { x: position.0 - 1, y: position.1, dir: self.clone() },
            Direction::Down => Vector { x: position.0 + 1, y: position.1, dir: self.clone() },
            Direction::Left => Vector { x: position.0, y: position.1 - 1, dir: self.clone() },
            Direction::Right => Vector { x: position.0, y: position.1 + 1, dir: self.clone() },
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Vector {
    x: usize,
    y: usize,
    dir: Direction,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Guard {
    direction: Direction,
    position: (usize, usize),
    path: Vec<Vector>,
    bounds: (usize, usize),
}

impl Guard {
    fn predict_path(&mut self, input: &Vec<Vec<String>>) -> Guard {
        while !self.leaving_bounds() && !self.is_loop() {
            self.traverse(&input);
        }
        self.clone()
    }

    fn traverse(&mut self, input: &Vec<Vec<String>>) {
        if self.check_move(input) {
            self.move_forward();
        } else {
            self.turn_right();
        }
    }

    fn check_move(&mut self, input: &Vec<Vec<String>>) -> bool {
        let new_pos = self.direction.move_forward(self.position);
        input[new_pos.x][new_pos.y] != "#"
    }

    fn move_forward(&mut self) {
        let new_pos = self.direction.move_forward(self.position);
        self.position = (new_pos.x, new_pos.y);
        self.path.push(Vector { x: self.position.0, y: self.position.1, dir: self.direction.clone() });
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn leaving_bounds(&self) -> bool {
        match self.direction {
            Direction::Up => self.position.0 == 0,
            Direction::Down => self.position.0 == self.bounds.0 - 1,
            Direction::Left => self.position.1 == 0,
            Direction::Right => self.position.1 == self.bounds.1 - 1,
        }
    }

    fn will_loop(&self) -> bool {
        let new_pos = self.direction.move_forward(self.position);
        if(self.path.contains(&new_pos)) {
            return true;
        }
        false
    }

    fn is_loop(&self) -> bool {
        let mut path = self.path.clone();
        let last_pos = path.pop().unwrap();
        if path.contains(&last_pos) {
            println!("path: {:?}, last_pos: {:?}", path, last_pos);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, vec};
    use tempfile::NamedTempFile;

    #[test]
    fn it_will_solve_part_one() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "....#.....");
        writeln!(temp_file, ".........#");
        writeln!(temp_file, "..........");
        writeln!(temp_file, "..#.......");
        writeln!(temp_file, ".......#..");
        writeln!(temp_file, "..........");
        writeln!(temp_file, ".#..^.....");
        writeln!(temp_file, "........#.");
        writeln!(temp_file, "#.........");
        writeln!(temp_file, "......#...");
        let formatted = format_input(temp_file.reopen().unwrap());
        let part1 = solve_part1(&formatted);
        assert_eq!(part1, 41);
    }

    #[test]
    fn it_will_solve_part_two() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "....#.....");
        writeln!(temp_file, ".........#");
        writeln!(temp_file, "..........");
        writeln!(temp_file, "..#.......");
        writeln!(temp_file, ".......#..");
        writeln!(temp_file, "..........");
        writeln!(temp_file, ".#..^.....");
        writeln!(temp_file, "........#.");
        writeln!(temp_file, "#.........");
        writeln!(temp_file, "......#...");
        let formatted = format_input(temp_file.reopen().unwrap());
        let part1 = solve_part2(&formatted);
        assert_eq!(part1, 6);
    }

    #[test]
    fn it_will_format_the_input_correctly() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "....#");
        writeln!(temp_file, ".....");
        writeln!(temp_file, "....^");
        writeln!(temp_file, "..#..");
        writeln!(temp_file, ".....");
        let formatted = format_input(temp_file.reopen().unwrap());
        assert_eq!(formatted, vec![
            vec![".", ".", ".", ".", "#"],
            vec![".", ".", ".", ".", "."],
            vec![".", ".", ".", ".", "^"],
            vec![".", ".", "#",".", "."],
            vec![".", ".", ".", ".", "."]
        ]);
    }

    #[test]
    fn it_will_find_the_caret() {
        let input = vec![
            vec![String::from("."), String::from("."), String::from("."), String::from("#"), String::from(".")],
            vec![String::from("."), String::from("."), String::from("."), String::from("."), String::from("#")],
            vec![String::from("."), String::from("."), String::from("."), String::from("^"), String::from(".")],
            vec![String::from("."), String::from("#"), String::from("."), String::from("."), String::from(".")],
            vec![String::from("."), String::from("."), String::from("."), String::from("#"), String::from(".")]
        ];
        let result = find_guard(&input);
        assert_eq!(result, Guard { direction: Direction::Up, position: (2, 3), path: vec![Vector { x: 2, y: 3, dir: Direction::Up }], bounds: (5, 5)});
    }

    #[test]
    fn it_will_predict_the_path() {
        let mut guard = Guard { direction: Direction::Up, position: (2, 3), path: vec![Vector { x: 2, y: 3, dir: Direction::Up }], bounds: (3, 2) };
        let input = vec![
            vec![String::from("."), String::from("."), String::from("."), String::from("#"), String::from(".")],
            vec![String::from("."), String::from("."), String::from("."), String::from("."), String::from("#")],
            vec![String::from("."), String::from("."), String::from("."), String::from("^"), String::from(".")]
        ];
        let result = guard.predict_path(&input).path;
        assert_eq!(result, vec![
            Vector { x: 2, y: 3, dir: Direction::Up },
            Vector { x: 1, y: 3, dir: Direction::Up },
            Vector { x: 2, y: 3, dir: Direction::Down }
        ]);
    }
}

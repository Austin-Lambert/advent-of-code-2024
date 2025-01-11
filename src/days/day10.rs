use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::vec;

const TRAIL_LENGTH: usize = 10;

pub fn solve(input: File) {
    let input = format_input(input);
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn format_input(input: File) -> TrailMap {
    let reader = io::BufReader::new(input);
    let mut map = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.unwrap().chars().enumerate() {
            row.push(TrailPoint { x: i, y: j, value: c.to_digit(10).unwrap() });
        }
        map.push(row);
    }
    TrailMap { map }
}

fn solve_part1(input: &TrailMap) -> u32 {
    let trails = input.get_one_way_trails();
    let trailheads = input.find_trailheads();
    let mut sum = 0;
    for trailhead in trailheads {
        let endpoints = get_unique_trail_endpoints_for_trailhead(&trails, &trailhead);
        sum += endpoints.len() as u32;
    }
    sum
}

fn solve_part2(input: &TrailMap) -> u32 {
    let trails = input.get_one_way_trails();
    trails.len() as u32
}

fn get_unique_trail_endpoints_for_trailhead(trails: &Vec<Trail>, trailhead: &TrailPoint) -> Vec<TrailPoint> {
    let mut endpoints = HashSet::new();
    for trail in trails {
        if trail.points.first().unwrap() == trailhead {
            endpoints.insert(trail.points.last().unwrap().clone());
        }
    }
    endpoints.into_iter().collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TrailMap {
    map: Vec<Vec<TrailPoint>>,
}

impl TrailMap {
    fn get_one_way_trails(&self) -> Vec<Trail> {
        let trailheads = self.find_trailheads();
        let mut trails = Vec::new();
        for trailhead in trailheads {
            let mut trail = Trail { points: vec![trailhead.clone()] };
            trails.extend(self.walk_trail(trail));
        }
        trails
    }

    fn find_trailheads(&self) -> Vec<TrailPoint> {
        let mut trailheads = Vec::new();
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.map[i][j].value == 0 {
                    trailheads.push(self.map[i][j].clone());
                }
            }
        }
        trailheads
    }

    fn walk_trail(&self, trail: Trail) -> Vec<Trail> {
        let mut trails = Vec::new();
        if trail.is_complete() {
            trails.push(trail);
            return trails;
        }
        let current_point = trail.points.last().unwrap();
        let adjacents = self.get_adjacent_points(current_point.clone());
        for adjacent in adjacents {
            if adjacent.value == (current_point.value + 1) {
                let mut new_trail = trail.clone();
                new_trail.add_point(adjacent);
                trails.extend(self.walk_trail(new_trail));
            }
        }
        trails
    }

    fn get_adjacent_points(&self, point: TrailPoint) -> Vec<TrailPoint> {
        let mut adjacent_points = Vec::new();
        if point.x > 0 {
            adjacent_points.push(self.map[point.x - 1][point.y].clone());
        }
        if point.x < self.map.len() - 1 {
            adjacent_points.push(self.map[point.x + 1][point.y].clone());
        }
        if point.y > 0 {
            adjacent_points.push(self.map[point.x][point.y - 1].clone());
        }
        if point.y < self.map[point.x].len() - 1 {
            adjacent_points.push(self.map[point.x][point.y + 1].clone());
        }
        adjacent_points
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TrailPoint {
    x: usize,
    y: usize,
    value: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Trail {
    points: Vec<TrailPoint>,
}

impl Trail {
    fn add_point(&mut self, point: TrailPoint) {
        self.points.push(point);
    }

    fn is_complete(&self) -> bool {
        if self.points.len() != TRAIL_LENGTH {
            return false;
        }
        for i in 1..self.points.len() {
            let prev_value = self.points[i-1].value;
            let curr_value = self.points[i].value;
            
            if curr_value != (prev_value + 1) % 10 {
                return false;
            }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn it_will_solve_part1() {
        let input = create_file_input();
        let formatted_input = format_input(input);
        assert_eq!(solve_part1(&formatted_input), 36);
    }

    #[test]
    fn it_will_solve_part2() {
        let input = create_file_input();
        let formatted_input = format_input(input);
        assert_eq!(solve_part2(&formatted_input), 81);
    }

    #[test]
    fn it_will_format_input() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "890").unwrap();
        writeln!(temp_file, "781").unwrap();
        writeln!(temp_file, "204").unwrap();
        assert_eq!(format_input(temp_file.reopen().unwrap()), 
            TrailMap {
                map: vec![
                    vec![TrailPoint { x: 0, y: 0, value: 8 }, TrailPoint { x: 0, y: 1, value: 9 }, TrailPoint { x: 0, y: 2, value: 0 }],
                    vec![TrailPoint { x: 1, y: 0, value: 7 }, TrailPoint { x: 1, y: 1, value: 8 }, TrailPoint { x: 1, y: 2, value: 1 }],
                    vec![TrailPoint { x: 2, y: 0, value: 2 }, TrailPoint { x: 2, y: 1, value: 0 }, TrailPoint { x: 2, y: 2, value: 4 }],
                ],
            }
        );
    }

    #[test]
    fn it_will_check_if_trail_is_complete() {
        let trail = Trail { points: vec![TrailPoint { x: 0, y: 0, value: 8 }, TrailPoint { x: 0, y: 1, value: 9 }, TrailPoint { x: 0, y: 2, value: 0 }] };
        assert_eq!(trail.is_complete(), false);
        let trail = Trail { points: vec![
            TrailPoint { x: 0, y: 2, value: 0 }, 
            TrailPoint { x: 0, y: 3, value: 1 }, 
            TrailPoint { x: 0, y: 4, value: 2 }, 
            TrailPoint { x: 0, y: 5, value: 3 }, 
            TrailPoint { x: 0, y: 6, value: 4 }, 
            TrailPoint { x: 0, y: 7, value: 5 }, 
            TrailPoint { x: 0, y: 8, value: 6 }, 
            TrailPoint { x: 0, y: 9, value: 7 },
            TrailPoint { x: 0, y: 0, value: 8 }, 
            TrailPoint { x: 0, y: 1, value: 9 }
        ]};
        assert_eq!(trail.is_complete(), true);
    }

    #[test]
    fn it_will_find_trailheads() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "890").unwrap();
        writeln!(temp_file, "781").unwrap();
        writeln!(temp_file, "204").unwrap();
        let file = temp_file.reopen().unwrap();
        let formatted_input = format_input(file);
        assert_eq!(formatted_input.find_trailheads(), vec![TrailPoint { x: 0, y: 2, value: 0 }, TrailPoint { x: 2, y: 1, value: 0 }]);
    }

    #[test]
    fn it_will_find_adjacent_points() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "890").unwrap();
        writeln!(temp_file, "781").unwrap();
        writeln!(temp_file, "204").unwrap();
        let file = temp_file.reopen().unwrap();
        let formatted_input = format_input(file);
        let point = TrailPoint { x: 1, y: 1, value: 8 };
        let adjacents = formatted_input.get_adjacent_points(point);
        assert_eq!(adjacents, vec![TrailPoint { x: 0, y: 1, value: 9 }, TrailPoint { x: 2, y: 1, value: 0 }, TrailPoint { x: 1, y: 0, value: 7 }, TrailPoint { x: 1, y: 2, value: 1 }]);
    }

    fn create_file_input() -> File {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "89010123").unwrap();
        writeln!(temp_file, "78121874").unwrap();
        writeln!(temp_file, "87430965").unwrap();
        writeln!(temp_file, "96549874").unwrap();
        writeln!(temp_file, "45678903").unwrap();
        writeln!(temp_file, "32019012").unwrap();
        writeln!(temp_file, "01329801").unwrap();
        writeln!(temp_file, "10456732").unwrap();
        temp_file.reopen().unwrap()
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

pub fn solve(input: File) {
    let formatted = format_input(input);

    let part1 = solve_part1(&formatted);
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&formatted);
    println!("The answer for part 2 is: {}", part2);
}

fn solve_part1(input: &Map) -> i32 {
    let antennas_by_frequency = group_antennas_by_frequency(&input.antennas);
    let mut antis = Vec::new();
    for (_, antennas) in antennas_by_frequency {
        let antenna_pairs = get_unique_antenna_pairs(&antennas);
        let antinodes = get_anti_nodes(&antenna_pairs);
        antis.extend(antinodes);
    }    
    let points = get_distinct_points_with_antinodes(&antis);
    let points_on_map = filter_points_on_map(&points, &input);
    points_on_map.len() as i32
}

fn solve_part2(input: &Map) -> i32 {
    let antennas_by_frequency = group_antennas_by_frequency(&input.antennas);
    let mut antis = Vec::new();
    for (_, antennas) in antennas_by_frequency {
        let antenna_pairs = get_unique_antenna_pairs(&antennas);
        let antinodes = get_anti_nodes_harmonic(&antenna_pairs, &input);
        antis.extend(antinodes);
    }    
    let points = get_distinct_points_with_antinodes(&antis);
    let points_on_map = filter_points_on_map(&points, &input);
    points_on_map.len() as i32
}

fn format_input(input: File) -> Map {
    let reader = io::BufReader::new(input);
    let mut map = Map {
        points: Vec::new(),
        antennas: Vec::new(),
    };
    let mut line_num = 0;
    for line in reader.lines() {
        let str = line.unwrap();
        let chars = str.chars().collect::<Vec<char>>();
        let mut point_num = 0;
        for c in chars {
            map.points.push(Point { x: point_num, y: line_num });
            let c_str = c.to_string();
            if c_str != "." {
                map.antennas.push(Antenna {
                    pos: Point { x: point_num, y: line_num },
                    freq: c_str,
                });
            }
            point_num += 1;
        }
        line_num += 1;
    }
    map
}

fn group_antennas_by_frequency(antennas: &Vec<Antenna>) -> HashMap<String, Vec<Antenna>> {
    let mut map = HashMap::new();
    for antenna in antennas {
        map.entry(antenna.freq.clone()).or_insert(vec![]).push(antenna.clone());
    }
    map
}

fn get_unique_antenna_pairs(antennas: &Vec<Antenna>) -> Vec<AntennaPair> {
    if antennas.len() < 2 {
        return Vec::new();
    }
    let mut antennas = antennas.clone();
    let mut pairs = Vec::new();
    let first = antennas.swap_remove(0);
    for second in &antennas {
        pairs.push(AntennaPair {
            first: first.clone(),
            second: second.clone(),
        });
    }
    pairs.extend(get_unique_antenna_pairs(&antennas));
    pairs
}

fn get_anti_nodes(antennas: &Vec<AntennaPair>) -> Vec<AntiNode> {
    let mut nodes = Vec::new();
    for pair in antennas {
        let first = pair.first.pos.clone();
        let second = pair.second.pos.clone();
        let diff = first.clone() - second.clone();

        nodes.push(AntiNode { antennas: pair.clone(), point: first.clone() + diff.clone() });
        nodes.push(AntiNode { antennas: pair.clone(), point: second.clone() - diff.clone() });
    }
    nodes
}

fn get_anti_nodes_harmonic(antennas: &Vec<AntennaPair>, map: &Map) -> Vec<AntiNode> {
    let mut nodes = Vec::new();
    for pair in antennas {
        let first = pair.first.pos.clone();
        let second = pair.second.pos.clone();
        let diff = first.clone() - second.clone();
        let mut new_point = first.clone();
        while map.is_point_in_map(new_point) {
            nodes.push(AntiNode { antennas: pair.clone(), point: new_point });
            new_point = new_point + diff.clone();
        }
        new_point = second.clone();
        while map.is_point_in_map(new_point) {
            nodes.push(AntiNode { antennas: pair.clone(), point: new_point });
            new_point = new_point - diff.clone();
        }
    }
    nodes
}

fn get_distinct_points_with_antinodes(antinodes: &Vec<AntiNode>) -> Vec<Point> {
    let mut points = HashSet::new();
    for node in antinodes {
        points.insert(node.point.clone());
    }
    points.into_iter().collect()
}

fn filter_points_on_map(points: &Vec<Point>, map: &Map) -> Vec<Point> {
    points.iter().filter(|point| map.is_point_in_map(**point)).cloned().collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    points: Vec<Point>,
    antennas: Vec<Antenna>,
}

impl Map {
    fn is_point_in_map(&self, point: Point) -> bool {
        self.points.contains(&point)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Antenna {
    pos: Point,
    freq: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AntennaPair {
    first: Antenna,
    second: Antenna,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AntiNode {
    antennas: AntennaPair,
    point: Point,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, vec};
    use tempfile::NamedTempFile;

    #[test]
    fn it_will_solve_part1() {
        let input = create_file_input();
        let formatted = format_input(input);
        let part1 = solve_part1(&formatted);
        assert_eq!(part1, 14);
    }

    #[test]
    fn it_will_solve_part2() {
        let input = create_file_input();
        let formatted = format_input(input);
        let part2 = solve_part2(&formatted);
        assert_eq!(part2, 34);
    }

    #[test]
    fn it_will_group_antennas_by_frequency() {
        let antennas = vec![Antenna { pos: Point { x: 0, y: 0 }, freq: "1".to_string() }, Antenna { pos: Point { x: 1, y: 0 }, freq: "1".to_string() }, Antenna { pos: Point { x: 0, y: 1 }, freq: "2".to_string() }];
        let grouped = group_antennas_by_frequency(&antennas);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped.get("1").unwrap().len(), 2);
        assert_eq!(grouped.get("2").unwrap().len(), 1);
    }

    #[test]
    fn it_will_get_antenna_pairs() {
        let antennas = vec![Antenna { pos: Point { x: 0, y: 0 }, freq: "1".to_string() }, Antenna { pos: Point { x: 1, y: 0 }, freq: "2".to_string() }, Antenna { pos: Point { x: 0, y: 1 }, freq: "3".to_string() }];
        let pairs = get_unique_antenna_pairs(&antennas);
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0].first.freq, "1");
        assert_eq!(pairs[0].second.freq, "3");
        assert_eq!(pairs[1].first.freq, "1");
        assert_eq!(pairs[1].second.freq, "2");
        assert_eq!(pairs[2].first.freq, "3");
        assert_eq!(pairs[2].second.freq, "2");
    }

    #[test]
    fn it_will_get_anti_nodes() {
        let pairs = vec![AntennaPair { first: Antenna { pos: Point { x: 5, y: 5 }, freq: "1".to_string() }, second: Antenna { pos: Point { x: 3, y: 4 }, freq: "1".to_string() } }];
        let nodes = get_anti_nodes(&pairs);
        assert_eq!(nodes.len(), 2);
        println!("{:?}", nodes);
        assert_eq!(nodes[0].point, Point { x: 7, y: 6 });
        assert_eq!(nodes[1].point, Point { x: 1, y: 3 });
    }

    fn create_file_input() -> File {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "............").unwrap();
        writeln!(temp_file, "........0...").unwrap();
        writeln!(temp_file, ".....0......").unwrap();
        writeln!(temp_file, ".......0....").unwrap();
        writeln!(temp_file, "....0.......").unwrap();
        writeln!(temp_file, "......A.....").unwrap();
        writeln!(temp_file, "............").unwrap();
        writeln!(temp_file, "........A...").unwrap();
        writeln!(temp_file, ".........A..").unwrap();
        writeln!(temp_file, "............").unwrap();
        writeln!(temp_file, "............").unwrap();
        File::open(temp_file.path()).unwrap()
    }
}

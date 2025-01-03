use std::io::{self, BufRead};
use std::fs::File;
use regex::Regex;

pub fn solve(input: File) {
    let input_string = format_input(input);

    let part1 = solve_part1(&input_string);
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&input_string);
    println!("The answer for part 2 is: {}", part2);
}

fn solve_part1(input: &String) -> i32 {
    let muls = search_for_mul(input);
    let parsed_muls = parse_mul(muls);
    let mut result = 0;
    for mul in parsed_muls {
        result += mul.a * mul.b;
    }
    result
}

fn solve_part2(input: &String) -> i32 {
    let mut ans = solve_part1(input);
    let donts = parse_donts(input);
    for dont in donts {
        ans -= dont.a * dont.b;
    }
    ans
}

fn format_input(input: File) -> String {
    let reader = io::BufReader::new(input);
    let content: String = reader.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("");
    content
}

fn search_for_mul(input: &String) -> Vec<String> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let matches = re.find_iter(input);
    let mut result = Vec::new();
    for m in matches {
        result.push(m.as_str().to_string());
    }
    result
}

fn parse_mul(mul_strs: Vec<String>) -> Vec<Mul> {
    let mut result = Vec::new();
    for str in mul_strs {
        let num_str = str.replace("mul(", "").replace(")", "");
        let nums = num_str.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        result.push(Mul { a: nums[0], b: nums[1] });
    }
    result
}

fn parse_donts(input: &String) -> Vec<Mul> {
    let re = Regex::new(r"don't\(\)([\s\S]*?)(?:do\(\)|$)").unwrap();
    let matches = re.find_iter(input);
    let mut result = Vec::new();
    for m in matches {
        let donts = search_for_mul(&m.as_str().to_string());
        result.extend(parse_mul(donts).into_iter());
    }
    result
}

struct Mul {
    a: i32,
    b: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn it_will_format_the_input_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let expected = "lorem ipsum dolor sit amet";
        temp_file.write_all(expected.as_bytes()).unwrap();
        let actual = format_input(temp_file.reopen().unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_will_search_a_string_for_mul() {
        let input = String::from("lorem ipsum dolor sit ametmul(1,2)+mul[3,4]mul(5,6)mul(7, 8)");
        let result = search_for_mul(&input);
        assert_eq!(result, vec!["mul(1,2)","mul(5,6)"]);
    }

    #[test]
    fn it_will_parse_mul() {
        let mul_strs = vec![String::from("mul(1,2)")];
        let result = parse_mul(mul_strs);
        assert_eq!(result[0].a, 1);
        assert_eq!(result[0].b, 2);
    }

    #[test]
    fn it_will_solve_part1() {
        let input = String::from("lorem ipsum dolor sit ametmul(1,2)+mul[3,4]mul(5,6)mul(7, 8)");
        let result = solve_part1(&input);
        assert_eq!(result, 32);
    }

    #[test]
    fn it_will_parse_donts() {
        let input = String::from("lorem ipsum dolor sit ametdon't()mul(1,2))do(mul(3,4))");
        let result = parse_donts(&input);
        assert_eq!(result[0].a, 1);
        assert_eq!(result[0].b, 2);
        assert_eq!(result[1].a, 3);
        assert_eq!(result[1].b, 4);
    }

    #[test]
    fn it_will_solve_part2() {
        let input = String::from("don't()mul(1,2)do()mul(3,4)mul(5,6)");
        let result = solve_part2(&input);
        assert_eq!(result, 42);
    }
}

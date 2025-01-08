use std::fs::File;
use std::io::{self, BufRead};

pub fn solve(input: File) {
    let formatted = format_input(input);

    let part1 = solve_part1(&formatted);
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&formatted);
    println!("The answer for part 2 is: {}", part2);
}

fn solve_part1(input: &Vec<Calibration>) -> i64 {
    let mut valid_calibrations = check_calibrations(&input);
    let mut sum = 0;
    sum_calibrations(&valid_calibrations)
}

fn solve_part2(input: &Vec<Calibration>) -> i64 {
    let mut valid_calibrations = check_calibrations_split(&input);
    sum_calibrations(&valid_calibrations)
}

fn format_input(input: File) -> Vec<Calibration> {
    let mut parsed_input = Vec::new();
    let reader = io::BufReader::new(input);
    for line in reader.lines() {
        let line_text = line.unwrap();
        let result_to_terms = line_text.split(":").map(|s| s.trim()).collect::<Vec<&str>>();
        let result = result_to_terms[0].parse::<i64>().unwrap();
        let terms = result_to_terms[1].split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        parsed_input.push(Calibration { result, terms });
    }
    parsed_input
}

fn check_calibrations(calibrations: &Vec<Calibration>) -> Vec<Calibration> {
    let mut results = vec![];
    for calibration in calibrations {
        if check_calibration(&calibration.result, &mut calibration.terms.clone(), Operation::Add, false) 
            || check_calibration(&calibration.result, &mut calibration.terms.clone(), Operation::Multiply, false) {
            results.push(calibration.clone());
        }
    }
    results
}

fn check_calibrations_split(calibrations: &Vec<Calibration>) -> Vec<Calibration> {
    let mut results = vec![];
    for calibration in calibrations {
        println!("{:?}", calibration);
        if check_calibration(&calibration.result, &mut calibration.terms.clone(), Operation::Add, true) 
            || check_calibration(&calibration.result, &mut calibration.terms.clone(), Operation::Multiply, true)
            || check_calibration(&calibration.result, &mut calibration.terms.clone(), Operation::Split, true) {
            results.push(calibration.clone());
        }
    }
    results
}

fn check_calibration(target: &i64, terms: &mut Vec<i64>, operator: Operation, split: bool) -> bool {
    println!("{:?} {:?} {:?} {:?}", target, terms, operator, split);
    let mut terms = terms.clone();
    let last_term = terms.pop().unwrap();
    let new_target = if operator == Operation::Add {
        if *target < last_term {
            return false;
        }
        target - last_term
    } else if operator == Operation::Multiply {
        if target % last_term != 0 {
            return false;
        }
        target / last_term
    } else if operator == Operation::Split {
        let last_term_str = last_term.to_string();
        let target_str = target.to_string();
        if target_str.len() < last_term_str.len() || !target_str.ends_with(&last_term_str) {
            return false;
        }
        target_str[..target_str.len() - last_term_str.len()].parse::<i64>().unwrap()
    } else {
        0
    }; 

    if terms.is_empty() {
        return (new_target == 0 && operator == Operation::Add)
        || (new_target == 1 && operator == Operation::Multiply)
        || (new_target == last_term && operator == Operation::Split);
    }
    
    if check_calibration(&new_target, &mut terms, Operation::Add, split) { return true; }
    if check_calibration(&new_target, &mut terms, Operation::Multiply, split) { return true; }
    if split && check_calibration(&new_target, &mut terms, Operation::Split, split) { return true; }
    return false;
}

fn sum_calibrations(calibrations: &Vec<Calibration>) -> i64 {
    let mut sum = 0;
    for calibration in calibrations {
        sum += calibration.result;
    }
    sum
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Calibration {
    result: i64,
    terms: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Split,
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
        let result = solve_part1(&formatted.clone());
        assert_eq!(result, 3749);
    }

    #[test]
    fn it_will_solve_part2() {
        let input = create_file_input();
        let formatted = format_input(input);
        let result = solve_part2(&formatted.clone());
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_format_input() {
        let input = create_file_input();
        let result = format_input(input);
        assert_eq!(result.len(), 9);
        assert_eq!(result[0], Calibration { result: 190, terms: vec![10, 19] });
        assert_eq!(result[1], Calibration { result: 3267, terms: vec![81, 40, 27] });
        assert_eq!(result[2], Calibration { result: 83, terms: vec![17, 5] });
        assert_eq!(result[3], Calibration { result: 156, terms: vec![15, 6] });
        assert_eq!(result[4], Calibration { result: 7290, terms: vec![6, 8, 6, 15] });
        assert_eq!(result[5], Calibration { result: 161011, terms: vec![16, 10, 13] });
        assert_eq!(result[6], Calibration { result: 192, terms: vec![17, 8, 14] });
        assert_eq!(result[7], Calibration { result: 21037, terms: vec![9, 7, 18, 13] });
        assert_eq!(result[8], Calibration { result: 292, terms: vec![11, 6, 16, 20] });
    }

    #[test]
    fn it_will_check_calibration() {
        let mut terms = vec![10, 19];
        let result = check_calibration(&190, &mut terms, Operation::Add, false);
        assert_eq!(result, false);

        let mut terms = vec![10, 19];
        let result = check_calibration(&190, &mut terms, Operation::Multiply, false);
        assert_eq!(result, true);
    }

    fn create_file_input() -> File {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "190: 10 19");
        writeln!(temp_file, "3267: 81 40 27");
        writeln!(temp_file, "83: 17 5");
        writeln!(temp_file, "156: 15 6");
        writeln!(temp_file, "7290: 6 8 6 15");
        writeln!(temp_file, "161011: 16 10 13");
        writeln!(temp_file, "192: 17 8 14");
        writeln!(temp_file, "21037: 9 7 18 13");
        writeln!(temp_file, "292: 11 6 16 20");
        File::open(temp_file.path()).unwrap()
    }

}
use std::io::{self, BufRead};
use std::fs::File;

pub fn solve(input: File) {
    let reports = format_input(input);

    let part1 = solve_part1(&reports);
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&reports);
    println!("The answer for part 2 is: {}", part2);
}

fn solve_part1(input: &Vec<Report>) -> i32 {
    let mut count = 0;
    for report in input {
        if is_safe(calc_changes(report)) {
            count += 1;
        }
    }
    count
}

fn solve_part2(input: &Vec<Report>) -> i32 {
    let mut count = 0;
    let unsafe_reports = input.iter().filter(|report| !is_safe(calc_changes(report)));
    for report in unsafe_reports {
        if is_safe_dampened(report) {
            count += 1;
        }
    }
    count += solve_part1(input);
    count
}

fn format_input(input: File) -> Vec<Report> {
    let mut reports = Vec::new();
    let reader = io::BufReader::new(input);
    let content: String = reader.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    
    println!("File contents:\n{}", content);

    // Now recreate the reader since we consumed it
    let reader = io::BufReader::new(content.as_bytes());
    for line in reader.lines() {
        reports.push(Report { levels: format_line(line.unwrap()) });
    }
    reports
}

fn format_line(line: String) -> Vec<i32> {
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn is_safe(changes: Vec<i32>) -> bool {
    (changes.iter().all(|&change| change.is_positive()) 
        || changes.iter().all(|&change| change.is_negative()))
        && changes.iter().all(|&change| change.abs() <= 3 && change.abs() >= 1)
}

fn is_safe_dampened(report: &Report) -> bool {
    for i in 0..report.levels.len() {
        let mut levels_copy = report.levels.clone();
        levels_copy.remove(i);
        let new_report = Report { levels: levels_copy };
        if is_safe(calc_changes(&new_report)) {
            return true;
        }
    }
    false
}

fn calc_changes(report: &Report) -> Vec<i32> {
    let mut changes = Vec::new();
    for i in 0..report.levels.len() - 1 {
        changes.push(report.levels[i + 1] - report.levels[i]);
    }
    changes
}

#[derive(Clone)]
struct Report {
    levels: Vec<i32>
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn it_will_solve_part_1() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "7 6 4 2 1").unwrap();
        writeln!(temp_file, "1 2 7 8 9").unwrap();
        writeln!(temp_file, "9 7 6 2 1").unwrap();
        writeln!(temp_file, "1 3 2 4 5").unwrap();
        writeln!(temp_file, "8 6 4 4 1").unwrap();
        writeln!(temp_file, "1 3 6 7 9").unwrap();

        let input = format_input(temp_file.reopen().unwrap());
        let result = solve_part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_will_solve_part_2() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "7 6 4 2 1").unwrap();
        writeln!(temp_file, "1 2 7 8 9").unwrap();
        writeln!(temp_file, "9 7 6 2 1").unwrap();
        writeln!(temp_file, "1 3 2 4 5").unwrap();
        writeln!(temp_file, "8 6 4 4 1").unwrap();
        writeln!(temp_file, "1 3 6 7 9").unwrap();

        let input =format_input(temp_file.reopen().unwrap());

        let result = solve_part2(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_will_format_the_input_correctly() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "10 9 7 6 3").unwrap();
        writeln!(temp_file, "4 5 6 7 8").unwrap();
        writeln!(temp_file, "10 9 7 6 1").unwrap();
        writeln!(temp_file, "4 10 11 12 13").unwrap();
        writeln!(temp_file, "1 2 3 3 5").unwrap();
        writeln!(temp_file, "1 2 3 2 1").unwrap();

        let result = format_input(temp_file.reopen().unwrap());
        assert_eq!(result.len(), 6);
        assert_eq!(result[0].levels, vec![10, 9, 7, 6, 3]);
        assert_eq!(result[1].levels, vec![4, 5, 6, 7, 8]);
        assert_eq!(result[2].levels, vec![10, 9, 7, 6, 1]);
        assert_eq!(result[3].levels, vec![4, 10, 11, 12, 13]);
        assert_eq!(result[4].levels, vec![1, 2, 3, 3, 5]);
        assert_eq!(result[5].levels, vec![1, 2, 3, 2, 1]);
    }

    #[test]
    fn it_will_determine_if_a_report_is_safe_based_on_changes() {
        assert_eq!(is_safe(vec![-1, -2, -1, -3]), true);
        assert_eq!(is_safe(vec![-1, -2, 1, 3]), false);
        assert_eq!(is_safe(vec![3, 2, 1, 3]), true);
        assert_eq!(is_safe(vec![1, 10, 1, 3]), false);
    }

    #[test]
    fn it_will_calculate_the_changes_in_a_report() {
        let report = Report { levels: vec![10, 9, 7, 6, 3] };
        assert_eq!(calc_changes(&report), vec![-1, -2, -1, -3]);
    }

    #[test]
    fn it_will_ignore_one_unsafe_change() {
        let mut report = Report { levels: vec![9, 8, 6, 7, 4] };
        assert_eq!(is_safe_dampened(&report), true);

        report = Report {levels: vec![11, 10, 8, 9, 4] };
        assert_eq!(is_safe_dampened(&report), false);

        report = Report {levels: vec![2, 3, 1, 2, 5]};
        assert_eq!(is_safe_dampened(&report), false);
    }
}

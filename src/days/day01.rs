use std::io::{self, BufRead};
use std::fs::File;

pub fn solve(input: File) {
    let formatted = format_input(input);

    let part1 = solve_part1(formatted.clone());
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(formatted.clone());
    println!("The answer for part 2 is: {}", part2);
}

pub fn solve_part1(input: Input) -> i32 {
    let sorted = sort_input(input);
    let subtracted = subtract_lists(sorted.first, sorted.second);
    let sum = sum_list(subtracted);
    sum
}

pub fn solve_part2(input: Input) -> i32 {
    let similarity = calculate_similarity(input.first, input.second);
    let sum = sum_list(similarity);
    sum
}

fn format_input(input: File) -> Input {
    let mut parsed_input = Input {
        first: Vec::new(),
        second: Vec::new()
    };
    let reader = io::BufReader::new(input);
    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split("   ").collect::<Vec<&str>>();
        parsed_input.first.push(split[0].parse::<i32>().unwrap());
        parsed_input.second.push(split[1].parse::<i32>().unwrap());
    }
    parsed_input
}

fn sort_input(input: Input) -> Input {
    let mut sorted_input = input;
    sorted_input.first.sort();
    sorted_input.second.sort();
    sorted_input
}

fn subtract_lists(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..first.len() {
        result.push((first[i] - second[i]).abs());
    }
    result
}

fn sum_list(list: Vec<i32>) -> i32 {
    list.iter().sum()
}

fn calculate_similarity(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let mut similarity = Vec::new();
    for i in 0..first.len() {
        let mut count = 0;
        for j in 0..second.len() {
            if first[i] == second[j] {
                count += 1;
            }
        }
        similarity.push(first[i] * count);
    }
    similarity
}

#[derive(Clone)]
struct Input {
    first: Vec<i32>,
    second: Vec<i32>
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn it_will_solve_the_problem_for_part_1() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "1000   4000").unwrap();
        writeln!(temp_file, "2000   5000").unwrap();
        writeln!(temp_file, "3000   6000").unwrap();

        let input = temp_file.reopen().unwrap();
        let formatted = format_input(input);

        let answer = solve_part1(formatted);
        assert_eq!(answer, 9000);
    }

    #[test]
    fn it_will_solve_the_problem_for_part_2() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "1234   4000").unwrap();
        writeln!(temp_file, "4000   1234").unwrap();
        writeln!(temp_file, "3000   1234").unwrap();

        let input = temp_file.reopen().unwrap();
        let formatted = format_input(input);

        let answer = solve_part2(formatted);
        assert_eq!(answer, (1234 * 2) + (4000 * 1));
    }

    #[test]
    fn it_will_format_the_input_file_into_two_lists() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "1000   4000").unwrap();
        writeln!(temp_file, "2000   5000").unwrap();
        writeln!(temp_file, "3000   6000").unwrap();

        let formatted_input = format_input(temp_file.reopen().unwrap());
        assert_eq!(formatted_input.first, vec![1000, 2000, 3000]);
        assert_eq!(formatted_input.second, vec![4000, 5000, 6000]);

        temp_file.close().unwrap();
    }

    #[test]
    fn it_will_sort_the_input_lists_smallest_to_largest() {
        let input = Input {
            first: vec![1234, 1233, 1232],
            second: vec![4321, 4320, 4319]
        };
        let sorted_input = sort_input(input);
        assert_eq!(sorted_input.first, vec![1232, 1233, 1234]);
        assert_eq!(sorted_input.second, vec![4319, 4320, 4321]);
    }

    #[test]
    fn it_will_subtract_the_two_lists_into_a_new_list() {
        let input = Input {
            first: vec![5, 5, 5],
            second: vec![2, 8, 3]
        };
        let subtracted = subtract_lists(input.first, input.second);
        assert_eq!(subtracted, vec![3, 3, 2]);
    }

    #[test]
    fn it_will_sum_a_list_of_numbers() {
        let input = vec![3, 3, 2];
        let sum = sum_list(input);
        assert_eq!(sum, 8);
    }

    #[test]
    fn it_will_calculate_similarity_between_two_lists() {
        let input = Input {
            first: vec![1, 2, 3],
            second: vec![2, 3, 4]
        };
        let similarity = calculate_similarity(input.first, input.second);
        assert_eq!(similarity, vec![0, 2, 3]);
    }
} 

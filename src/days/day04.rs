use std::io::{self, BufRead};
use std::fs::File;
use regex::Regex;

pub fn solve(input: File) {
    let formatted_input = format_input(input);

    let part1 = solve_part1(&formatted_input);
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&formatted_input);
    println!("The answer for part 2 is: {}", part2);
}

fn solve_part1(input: &Vec<Vec<String>>) -> i32 {
    let lines = get_lines(input);
    let mega_string = get_mega_string(lines);
    search_for_xmas(mega_string)
}

fn solve_part2(input: &Vec<Vec<String>>) -> i32 {
    find_crossed_mas(input)
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

fn find_crossed_mas(matrix: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for i in 1..matrix.len()-1 {
        for j in 1..matrix[i].len()-1 {
            if matrix[i][j] == "A" && check_adjacents(matrix, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn check_adjacents(matrix: &Vec<Vec<String>>, i: usize, j: usize) -> bool {
    let dr = matrix[i - 1][j - 1].clone() + &matrix[i + 1][j + 1];
    let dl = matrix[i - 1][j + 1].clone() + &matrix[i + 1][j - 1];
    (dr == "MS" && (dl == "MS" || dl == "SM")) || (dr == "SM" && (dl == "MS" || dl == "SM"))
}

fn get_mega_string(lines: Vec<String>) -> String {
    lines.join("|")
}

fn get_lines(matrix: &Vec<Vec<String>>) -> Vec<String> {
    let mut lines = Vec::new();
    lines.extend(get_horizontal_lines(&matrix));
    lines.extend(get_vertical_lines(&matrix));
    lines.extend(get_down_right_diagonal_lines(&matrix));
    lines.extend(get_down_left_diagonal_lines(&matrix));
    lines
}

fn search_for_xmas(mega_string: String) -> i32 {
    let re = Regex::new(r"XMAS").unwrap();
    let revre = Regex::new(r"SAMX").unwrap();
    let matches = re.find_iter(&mega_string);
    let revmatches = revre.find_iter(&mega_string);
    let mut count = 0;
    for _m in matches {
        count += 1;
    }
    for _m in revmatches {
        count += 1;
    }
    count
}

fn get_horizontal_lines(matrix: &Vec<Vec<String>>) -> Vec<String> {
    let mut lines = Vec::new();
    for i in 0..matrix.len() {
        lines.push(matrix[i].join(""));
    }
    lines
}

fn get_vertical_lines(matrix: &Vec<Vec<String>>) -> Vec<String> {
    let mut lines = Vec::new();
    for j in 0..matrix[0].len() {
        let mut vertical = Vec::new();
        for i in 0..matrix.len() {
            vertical.push(matrix[i][j].clone());
        }
        lines.push(vertical.join(""));
    }
    lines
}

fn get_down_right_diagonal_lines(matrix: &Vec<Vec<String>>) -> Vec<String> {
    let mut lines = Vec::new();
    let max_height = matrix[0].len();
    let max_width = matrix.len();
    let mut edge = Vec::new();
    for i in (0..max_height).rev() {
        edge.push((i, 0));
    }
    for j in 1..max_width {
        edge.push((0, j));
    }
    for pos in edge { // 2,0 - 1,0 - 0,0 - 0,1 - 0,2
        let mut diagonal = Vec::new();
        let mut x = pos.0;
        let mut y = pos.1;
        loop {
            diagonal.push(matrix[x][y].clone());
            x += 1;
            y += 1;
            if x >= max_height || y >= max_width {
                break;
            }
        }
        lines.push(diagonal.join(""));
    }
    lines
}

fn get_down_left_diagonal_lines(matrix: &Vec<Vec<String>>) -> Vec<String> {
    let mut lines = Vec::new();
    let max_height = matrix[0].len();
    let max_width = matrix.len();
    let mut edge = Vec::new();
    for i in 0..max_width {
        edge.push((0, i));
    }
    for j in 1..max_height {
        edge.push((j, max_width - 1));
    }
    for pos in edge { // 0,0 - 0,1 - 0,2 - 1,2 - 2,2
        let mut diagonal = Vec::new();
        let mut x = pos.0;
        let mut y = pos.1;
        loop {
            diagonal.push(matrix[x][y].clone());
            x += 1;
            if y <= 0 { break }
            y -= 1;
            if x >= max_height { break }
        }
        lines.push(diagonal.join(""));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, vec};
    use tempfile::NamedTempFile;

    #[test]
    fn it_will_solve_part1() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "MMMSXXMASM");
        writeln!(temp_file, "MSAMXMSMSA");
        writeln!(temp_file, "AMXSXMAAMM");
        writeln!(temp_file, "MSAMASMSMX");
        writeln!(temp_file, "XMASAMXAMM");
        writeln!(temp_file, "XXAMMXXAMA");
        writeln!(temp_file, "SMSMSASXSS");
        writeln!(temp_file, "SAXAMASAAA");
        writeln!(temp_file, "MAMMMXMMMM");
        writeln!(temp_file, "MXMXAXMASX");
        let input = format_input(temp_file.reopen().unwrap());
        assert_eq!(solve_part1(&input), 18);
    }

    #[test]
    fn it_will_solve_part2() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "MMMSXXMASM");
        writeln!(temp_file, "MSAMXMSMSA");
        writeln!(temp_file, "AMXSXMAAMM");
        writeln!(temp_file, "MSAMASMSMX");
        writeln!(temp_file, "XMASAMXAMM");
        writeln!(temp_file, "XXAMMXXAMA");
        writeln!(temp_file, "SMSMSASXSS");
        writeln!(temp_file, "SAXAMASAAA");
        writeln!(temp_file, "MAMMMXMMMM");
        writeln!(temp_file, "MXMXAXMASX");
        let input = format_input(temp_file.reopen().unwrap());
        assert_eq!(solve_part2(&input), 9);
    }

    #[test]
    fn it_will_search_for_xmas() {
        let input = String::from("asdXMASasdfXMASAMXasdfXMAS");
        assert_eq!(search_for_xmas(input), 4);
    }

    #[test]
    fn it_will_take_the_string_input_and_make_it_a_matrix() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "abc").unwrap();
        writeln!(temp_file, "def").unwrap();
        let formatted_input = format_input(temp_file.reopen().unwrap());
        assert_eq!(formatted_input, vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
    }

    #[test]
    fn it_will_turn_the_matrix_into_lines() {
        let input = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")]
        ];
        let lines = get_lines(&input);
        assert_eq!(lines.len(), 16);
        assert_eq!(lines, vec![
            "abc", "def", "ghi", // horizontal
            "adg", "beh", "cfi", // vertical
            "g", "dh", "aei", "bf", "c", // diag down-right
            "a", "bd", "ceg", "fh", "i" // diag down-left
        ]);
    }

    #[test]
    fn it_will_turn_the_matrix_into_horizontal_lines() {
        let input = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")]
        ];
        let lines = get_horizontal_lines(&input);
        assert_eq!(lines, vec!["abc", "def", "ghi"]);
    }

    #[test]
    fn it_will_turn_the_matrix_into_vertical_lines() {
        let input = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")]
        ];
        let lines = get_vertical_lines(&input);
        assert_eq!(lines, vec!["adg", "beh", "cfi"]);
    }

    #[test]
    fn it_will_turn_the_matrix_into_diagonal_lines_down_right() {
        let input = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")]
        ];
        let lines = get_down_right_diagonal_lines(&input);
        assert_eq!(lines, vec!["g", "dh", "aei", "bf", "c"]);
    }

    #[test]
    fn it_will_turn_the_matrix_into_diagonal_lines_down_left() {
        let input = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")]
        ];
        let lines = get_down_left_diagonal_lines(&input);
        assert_eq!(lines, vec!["a", "bd", "ceg", "fh", "i"]);
    }

    #[test]
    fn it_will_get_the_mega_string() {
        let input = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")]
        ];
        let lines = get_mega_string(get_lines(&input));
        assert_eq!(lines, "abc|def|ghi|adg|beh|cfi|g|dh|aei|bf|c|a|bd|ceg|fh|i");
    }

    fn it_will_check_adjacents() {
        let input = vec![
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("d"), String::from("e"), String::from("f")],
            vec![String::from("g"), String::from("h"), String::from("i")]
        ];
        assert_eq!(find_crossed_mas(&input), 1);
    }
}

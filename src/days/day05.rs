use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::ops::Index;

pub fn solve(input: File) {
    let formatted_input = format_input(input);
    let parsed_input = parse_raw_input(&formatted_input);
    let results = check_all_updates(&parsed_input.page_rules, &parsed_input.page_updates);

    let part1 = solve_part1(&results);
    println!("The answer for part 1 is: {}", part1);

    let part2 = solve_part2(&results, &parsed_input.page_rules);
    println!("The answer for part 2 is: {}", part2);
}

fn solve_part1(input: &CheckResults) -> u32 {
    sum_up_midpoints(&input.correct)
}

fn solve_part2(input: &CheckResults, rules: &HashMap<u32, Page>) -> u32 {
    let corrected = correct_updates(input, rules);
    sum_up_midpoints(&corrected)
}

fn format_input(input: File) -> RawInput {
    let reader = io::BufReader::new(input);
    let mut should_be_rules = true;
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            should_be_rules = false;
            continue;
        }
        if should_be_rules {
            rules.push(line.clone());
        } else {
            updates.push(line);
        }
    }
    
    RawInput { rules, updates }
}

fn parse_raw_input(raw_input: &RawInput) -> ParsedInput {
    let page_rules = parse_rules(raw_input.rules.clone());
    let page_updates = parse_updates(raw_input.updates.clone());
    ParsedInput { page_rules, page_updates }
}

fn parse_rules(raw_rules: Vec<String>) -> HashMap<u32, Page> {
    let mut pages = HashMap::new();
    for rule_line in raw_rules {
        let rule = parse_rule(rule_line);
        
        pages.entry(rule.0)
            .or_insert(Page { 
                number: rule.0,
                should_come_before: Vec::new(), 
                should_come_after: Vec::new() 
            })
            .should_come_before.push(rule.1);

        pages.entry(rule.1)
            .or_insert(Page { 
                number: rule.1,
                should_come_before: Vec::new(), 
                should_come_after: Vec::new() 
            })
            .should_come_after.push(rule.0);
    }
    pages
}

fn parse_rule(line: String) -> (u32, u32) {
    let pages = line.split("|").collect::<Vec<&str>>().into_iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    (pages[0], pages[1])
}

fn parse_updates(updates: Vec<String>) -> Vec<Vec<u32>> {
    updates.into_iter().map(|update| update.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>()
}

fn check_all_updates(rules: &HashMap<u32, Page>, updates: &Vec<Vec<u32>>) -> CheckResults {
    let mut results = CheckResults { correct: Vec::new(), incorrect: Vec::new() };
    for update in updates {
        if check_update_set(rules, update) {
            results.correct.push(update.clone());
        } else {
            results.incorrect.push(update.clone());
        }
    }
    results
}

fn check_update_set(rules: &HashMap<u32, Page>, update_set: &Vec<u32>) -> bool {
    for i in 0..update_set.len() {
        let rule = rules.get(&update_set[i]).unwrap();
        if i > 0 {
            if rule.check_before(update_set[i-1]) {
            return false;
            }
        }
        if i < update_set.len() - 1 {
            if rule.check_after(update_set[i+1]) {
                return false;
            }
        }
    }
    true
}

fn sum_up_midpoints(updates: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for update in updates {
        sum += update[update.len() / 2];
    }
    sum
}

fn correct_updates(input: &CheckResults, rules: &HashMap<u32, Page>) -> Vec<Vec<u32>> {
    let incorrect = input.incorrect.clone();
    let mut corrected = Vec::new();
    for update in incorrect {
        corrected.push(correct_update(&update, rules, Vec::new()));
    }
    corrected
}

fn correct_update(update: &Vec<u32>, rules: &HashMap<u32, Page>, pre_corrected: Vec<u32>) -> Vec<u32> {
    let relevant_rules = find_relevant_rules(update, rules);
    let mut corrected = Vec::new();
    let mut to_remove = Vec::new();
    let mut update = update.clone();

    if update.len() == 1 {
        corrected.push(relevant_rules[0].number);
        return add_outsides(&update, &pre_corrected);
    } else if update.len() == 0 {
        return pre_corrected;
    }
    for i in 0..update.len() {
        let rule = relevant_rules.get(i).unwrap().clone();
        if rule.should_come_after.is_empty() {
            corrected.insert(0, rule.number);
            to_remove.push(i);
        } else if rule.should_come_before.is_empty() {
            corrected.push(rule.number);
            to_remove.push(i);
        }
    }
    for i in to_remove.iter().rev() {
        update.remove(*i);
    }
    correct_update(&update, rules, add_outsides(&corrected, &pre_corrected))
}

fn add_outsides(inside: &Vec<u32>, outside: &Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    if outside.len() == 0 {
        return inside.clone();
    }
    result.push(outside[0]);
    result.extend(inside);
    result.push(outside[1]);
    result
}

fn find_relevant_rules(update: &Vec<u32>, rules: &HashMap<u32, Page>) -> Vec<Page> {
    let mut relevant_rules = Vec::new();
    for i in 0..update.len() {
        let rule = rules.get(&update[i]).unwrap();
        relevant_rules.push(rule.pare(&update));
    }
    relevant_rules
}

struct RawInput {
    rules: Vec<String>,
    updates: Vec<String>,
}

struct ParsedInput {
    page_rules: HashMap<u32, Page>,
    page_updates: Vec<Vec<u32>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Page {
    number: u32,
    should_come_before: Vec<u32>,
    should_come_after: Vec<u32>,
}

struct CheckResults {
    correct: Vec<Vec<u32>>,
    incorrect: Vec<Vec<u32>>,
}

impl Page {
    fn check_before(&self, update: u32) -> bool {
        !self.should_come_after.contains(&update)
    }

    fn check_after(&self, update: u32) -> bool {
        !self.should_come_before.contains(&update)
    }

    fn all_should_be_after(&self, update: &Vec<u32>) -> bool {
        !self.should_come_after.iter().all(|&i| update.contains(&i)) && 
        self.should_come_before.iter().all(|&i| update.contains(&i))
    }

    fn all_should_be_before(&self, update: &Vec<u32>) -> bool {
        !self.should_come_before.iter().all(|&i| update.contains(&i)) && 
        self.should_come_after.iter().all(|&i| update.contains(&i))
    }

    fn pare(&self, update: &Vec<u32>) -> Page {
        Page { 
            number: self.number,
            should_come_before: self.should_come_before.iter().filter(|&&i| update.contains(&i)).copied().collect(), 
            should_come_after: self.should_come_after.iter().filter(|&&i| update.contains(&i)).copied().collect() 
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, vec};
    use tempfile::NamedTempFile;

    #[test]
    fn it_will_solve_part1() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "47|53").unwrap();
        writeln!(temp_file, "97|13").unwrap();
        writeln!(temp_file, "97|61").unwrap();
        writeln!(temp_file, "97|47").unwrap();
        writeln!(temp_file, "75|29").unwrap();
        writeln!(temp_file, "61|13").unwrap();
        writeln!(temp_file, "75|53").unwrap();
        writeln!(temp_file, "29|13").unwrap();
        writeln!(temp_file, "97|29").unwrap();
        writeln!(temp_file, "53|29").unwrap();
        writeln!(temp_file, "61|53").unwrap();
        writeln!(temp_file, "97|53").unwrap();
        writeln!(temp_file, "61|29").unwrap();
        writeln!(temp_file, "47|13").unwrap();
        writeln!(temp_file, "75|47").unwrap();
        writeln!(temp_file, "97|75").unwrap();
        writeln!(temp_file, "47|61").unwrap();
        writeln!(temp_file, "75|61").unwrap();
        writeln!(temp_file, "47|29").unwrap();
        writeln!(temp_file, "75|13").unwrap();
        writeln!(temp_file, "53|13").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "75,47,61,53,29").unwrap();
        writeln!(temp_file, "97,61,53,29,13").unwrap();
        writeln!(temp_file, "75,29,13").unwrap();
        writeln!(temp_file, "75,97,47,61,53").unwrap();
        writeln!(temp_file, "61,13,29").unwrap();
        writeln!(temp_file, "97,13,75,29,47").unwrap();
        let formatted_input = format_input(temp_file.reopen().unwrap());
        let parsed_input = parse_raw_input(&formatted_input);
        let results = check_all_updates(&parsed_input.page_rules, &parsed_input.page_updates);
        assert_eq!(solve_part1(&results), 143);
    }

    #[test]
    fn it_will_solve_part2() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "47|53").unwrap();
        writeln!(temp_file, "97|13").unwrap();
        writeln!(temp_file, "97|61").unwrap();
        writeln!(temp_file, "97|47").unwrap();
        writeln!(temp_file, "75|29").unwrap();
        writeln!(temp_file, "61|13").unwrap();
        writeln!(temp_file, "75|53").unwrap();
        writeln!(temp_file, "29|13").unwrap();
        writeln!(temp_file, "97|29").unwrap();
        writeln!(temp_file, "53|29").unwrap();
        writeln!(temp_file, "61|53").unwrap();
        writeln!(temp_file, "97|53").unwrap();
        writeln!(temp_file, "61|29").unwrap();
        writeln!(temp_file, "47|13").unwrap();
        writeln!(temp_file, "75|47").unwrap();
        writeln!(temp_file, "97|75").unwrap();
        writeln!(temp_file, "47|61").unwrap();
        writeln!(temp_file, "75|61").unwrap();
        writeln!(temp_file, "47|29").unwrap();
        writeln!(temp_file, "75|13").unwrap();
        writeln!(temp_file, "53|13").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "75,47,61,53,29").unwrap();
        writeln!(temp_file, "97,61,53,29,13").unwrap();
        writeln!(temp_file, "75,29,13").unwrap();
        writeln!(temp_file, "75,97,47,61,53").unwrap();
        writeln!(temp_file, "61,13,29").unwrap();
        writeln!(temp_file, "97,13,75,29,47").unwrap();
        let formatted_input = format_input(temp_file.reopen().unwrap());
        let parsed_input = parse_raw_input(&formatted_input);
        let results = check_all_updates(&parsed_input.page_rules, &parsed_input.page_updates);
        assert_eq!(solve_part2(&results, &parsed_input.page_rules), 123);
    }
    #[test]
    fn it_will_format_the_input_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "1|2").unwrap();
        writeln!(temp_file, "2|3").unwrap();
        writeln!(temp_file, "3|4").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "1").unwrap();
        writeln!(temp_file, "2").unwrap();
        writeln!(temp_file, "3").unwrap();
        let input = format_input(temp_file.reopen().unwrap());
        assert_eq!(input.rules, vec!["1|2", "2|3", "3|4"]);
        assert_eq!(input.updates, vec!["1", "2", "3"]);
    }

    #[test]
    fn it_will_parse_rules() {
        let rules = vec![String::from("1|2"), String::from("2|3"), String::from("3|4")];
        let pages = parse_rules(rules);
        assert_eq!(pages.len(), 4);
        assert_eq!(pages.get(&1).unwrap().should_come_before, vec![2]);
        assert_eq!(pages.get(&2).unwrap().should_come_before, vec![3]);
        assert_eq!(pages.get(&3).unwrap().should_come_before, vec![4]);
        assert_eq!(pages.get(&4).unwrap().should_come_before, vec![]);
        assert_eq!(pages.get(&1).unwrap().should_come_after, vec![]);
        assert_eq!(pages.get(&2).unwrap().should_come_after, vec![1]);
        assert_eq!(pages.get(&3).unwrap().should_come_after, vec![2]);
        assert_eq!(pages.get(&4).unwrap().should_come_after, vec![3]);
    }

    #[test]
    fn it_will_parse_a_rule() {
        let rule = parse_rule(String::from("1|2"));
        assert_eq!(rule, (1, 2));
    }

    #[test]
    fn it_will_parse_updates() {
        let updates = vec![String::from("1,2,3"), String::from("4,5,6")];
        let parsed_updates = parse_updates(updates);
        assert_eq!(parsed_updates, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn it_will_parse_raw_input() {
        let raw_input = RawInput { 
            rules: vec![String::from("1|2"), String::from("2|3"), String::from("3|4")], 
            updates: vec![String::from("1,2,3"), String::from("4,5,6")]
        };
        let parsed_input = parse_raw_input(&raw_input);
        assert_eq!(parsed_input.page_rules.len(), 4);
        assert_eq!(parsed_input.page_updates, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn it_will_determine_if_a_set_of_updates_is_in_the_correct_order() {
        let rules = HashMap::from([
            (1, Page { number: 1, should_come_before: vec![2], should_come_after: vec![] }),
            (2, Page { number: 2, should_come_before: vec![3], should_come_after: vec![1] }),
            (3, Page { number: 3, should_come_before: vec![4], should_come_after: vec![2] }),
            (4, Page { number: 4, should_come_before: vec![], should_come_after: vec![3] }),
        ]);
        let update_set = vec![1, 2, 3];
        assert_eq!(check_update_set(&rules, &update_set), true);

        let update_set = vec![1, 3, 2];
        assert_eq!(check_update_set(&rules, &update_set), false);
    }

    #[test]
    fn page_will_check_if_the_update_is_correct() {
        let page = Page { number: 1, should_come_before: vec![2], should_come_after: vec![3] };
        assert_eq!(page.check_before(2), true);
        assert_eq!(page.check_before(3), false);
        assert_eq!(page.check_before(4), true); // ???
        assert_eq!(page.check_after(2), false);
        assert_eq!(page.check_after(3), true);
        assert_eq!(page.check_after(4), true);
    }

    #[test]
    fn it_will_determine_which_updates_are_in_the_correct_order() {
        let input = ParsedInput {
            page_rules: HashMap::from([
                (1, Page { number: 1, should_come_before: vec![2], should_come_after: vec![] }),
                (2, Page { number: 2, should_come_before: vec![3], should_come_after: vec![1] }),
                (3, Page { number: 3, should_come_before: vec![4], should_come_after: vec![2] }),
                (4, Page { number: 4, should_come_before: vec![], should_come_after: vec![3] }),
            ]),
            page_updates: vec![vec![1, 2, 3], vec![1, 3, 2], vec![3, 4], vec![4,3]],
        };
        let expected = CheckResults { correct: vec![vec![1,2,3], vec![3,4]], incorrect: vec![vec![1,3,2], vec![4,3]] };
        let results = check_all_updates(&input.page_rules, &input.page_updates);
        assert_eq!(results.correct, expected.correct);
        assert_eq!(results.incorrect, expected.incorrect);
    }

    #[test]
    fn it_will_sum_up_the_midpoints_of_the_correct_updates() {
        let correct_updates = vec![vec![1,2,3], vec![3,4,5]];
        assert_eq!(sum_up_midpoints(&correct_updates), 6);
    }

    #[test]
    fn it_will_correct_an_update() {
        let update = vec![1,3,2];
        let rules = HashMap::from([
            (1, Page { number: 1, should_come_before: vec![2], should_come_after: vec![] }),
            (2, Page { number: 2, should_come_before: vec![3], should_come_after: vec![1] }),
            (3, Page { number: 3, should_come_before: vec![4], should_come_after: vec![2] }),
            (4, Page { number: 4, should_come_before: vec![], should_come_after: vec![3] }),
        ]);
        assert_eq!(correct_update(&update, &rules, Vec::new()), vec![1,2,3]);
    }

    #[test]
    fn it_will_add_outsides() {
        let inside = vec![1,2,3];
        let outside = vec![4,5];
        assert_eq!(add_outsides(&inside, &outside), vec![4,1,2,3,5]);
    }
}


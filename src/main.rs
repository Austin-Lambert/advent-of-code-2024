use std::fs::{self, File};
use std::io::{self, Write};
use std::process::exit;

mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
}

fn main() {
    startup_prompt();
    let day = day_prompt();
    let input = select_input_file();
    solve_day(day, input);
}

fn startup_prompt() {
    println!(" _______  ______            _______  _       _________   _______  _______    _______  _______  ______   _______    _______  _______  _______    ___   ");
    println!("(  ___  )(  __  \\ |\\     /|(  ____ \\( (    /|\\__   __/  (  ___  )(  ____ \\  (  ____ \\(  ___  )(  __  \\ (  ____ \\  / ___   )(  __   )/ ___   )  /   )  ");
    println!("| (   ) || (  \\  )| )   ( || (    \\/|  \\  ( |   ) (     | (   ) || (    \\/  | (    \\/| (   ) || (  \\  )| (    \\/  \\/   )  || (  )  |\\/   )  | / /) |   ");
    println!("|  ___  || |   | |( (   ) )|  __)   | (\\ \\) |   | |     | |   | ||  __)     | |      | |   | || |   | ||  __)       _/   / | (/ /) |  _/   /(____   _) ");
    println!("| (   ) || |   ) | \\ \\_/ / | (      | | \\   |   | |     | |   | || (        | |      | |   | || |   ) || (         /   _/  |   / | | /   _/      ) (   ");
    println!("| )   ( || (__/  )  \\   /  | (____/\\| )  \\  |   | |     | (___) || )        | (____/\\| (___) || (__/  )| (____/\\  (   (__/\\|  (__) |(   (__/\\    | |   ");
    println!("|/     \\|(______/    \\_/   (_______/|/    )_)   )_(     (_______)|/         (_______/(_______)(______/ (_______/  \\_______/(_______)\\_______/    (_)   ");
    println!("Welcome to Austin Lambert's Advent of Code 2024!");
}

fn day_prompt() -> u32 {
    print!("Please enter the day you want to run: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse().unwrap_or_else(|_| {
            println!("Invalid input. Defaulting to day 1");
            1
        }),
        Err(e) => {
            println!("Error reading input: {}. Defaulting to day 1", e);
            1
        }
    }
}

fn select_input_file() -> File {
    let input_dir = "src/inputs";
    
    let entries = fs::read_dir(input_dir).unwrap_or_else(|e| {
        println!("Error reading directory: {}.", e);
        exit(1)
    });
    let mut files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();
    
    files.sort_by(|a, b| a.path().cmp(&b.path()));
    
    println!("\nAvailable input files:");
    for (i, file) in files.iter().enumerate() {
        println!("{}. {}", i + 1, file.path().display());
    }
    
    print!("Select a file (1-{}): ", files.len());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        println!("Error reading input: {}. Defaulting to first file", e);
        1
    });
    let selection = input.trim().parse::<usize>().unwrap_or(1) - 1;
    
    File::open(files[selection].path()).unwrap_or_else(|e| {
        println!("Error opening file: {}.", e);
        exit(1)
    })
}

fn solve_day(day: u32, input: File) {
    match day {
        1 => days::day01::solve(input),
        2 => days::day02::solve(input),
        3 => days::day03::solve(input),
        4 => days::day04::solve(input),
        5 => days::day05::solve(input),
        6 => days::day06::solve(input),
        7 => days::day07::solve(input),
        8 => days::day08::solve(input),
        9 => days::day09::solve(input),
        10 => days::day10::solve(input),
        _ => println!("Day {} not implemented yet", day),
    };
}

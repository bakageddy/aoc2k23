use std::fs;
use regex::Regex;

fn main() {
    let file = "../tests/input.txt";
    // let ans = part1(file);
    let ans = part2(file);
    println!("{}", ans);
}

fn part1(filename: &str) -> i64 {
    let contents = match fs::read_to_string(filename) {
        Ok(cont) => cont,
        Err(_) => panic!("Failed to read file!"),
    };
    let mut sum = 0;
    for i in contents.lines() {
        let (mut first, mut last) = ('0', '0');

        let raw = i.as_bytes();
        if let Some(x) = i.find(char::is_numeric) {
            if let Some(y) = i.rfind(char::is_numeric) {
                first = raw[x] as char;
                last = raw[y] as char;
            }
        }
        let num_string = format!("{}{}", first, last);
        if let Ok(x) = num_string.parse::<i64>() {
            sum += x;
        } 
    }
    sum
}

fn parse_nums(str: &str) -> char {
    match str {
        "zero" | "0"  => '0',
        "one" | "1" => '1',
        "two" | "2" => '2',
        "three" | "3" => '3',
        "four" | "4"=> '4',
        "five" | "5" => '5',
        "six" | "6" => '6',
        "seven" | "7" => '7',
        "eight" | "8" => '8',
        "nine" | "9" => '9',
        _ => '\0',
    }
}

fn extract_keywords(line: &str) -> Vec<char> {
    let pat = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut v = vec![];
    for i in pat.find_iter(line) {
        v.push(i.as_str());
    }
    v.into_iter().map(|c| {parse_nums(c)}).collect()
}

fn part2(filename: &str) -> i64 {
    let contents = match fs::read_to_string(filename) {
        Ok(cont) => cont,
        Err(_) => panic!("Failed to read file!"),
    };

    let mut sum = 0;
    for i in contents.lines() {
        let nums = extract_keywords(i);
        let (first, last) = (nums[0], nums[nums.len() - 1]);
        let num = format!("{}{}", first, last).parse::<i64>().unwrap();
        println!("{}: {}", i, num);
        sum += num;
    }
    sum
}

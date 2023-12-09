use std::fs;
use std::collections::HashMap;
use std::io::Write;

const NUMS: [&str; 18] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine","1", "2", "3", "4", "5", "6", "7", "8", "9"];

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

fn part2(filename: &str) -> u64 {
    let contents = match fs::read_to_string(filename) {
        Ok(cont) => cont,
        Err(_) => panic!("Failed to read file!"),
    };

    let mut op = fs::File::create("./debug.txt").unwrap();

    let mut sum: u64 = 0;
    for line in contents.lines() {
        let result = available_nums(line);
        let (mut first, mut last) = (line.len(), 0);
        let (mut x, mut y) = (0, 0);
        for (key, val) in result {
            if val.0 == val.1 {
                if val.0.is_none() {continue;}
                let index = val.0.unwrap();
                if index <= first {
                    first = index;
                    x = parse_num(key);
                }
                if index >= last {
                    last = index;
                    y = parse_num(key);
                }
            } else {
                if val.0.is_none() || val.1.is_none() {
                    continue;
                }
                let index1 = val.0.unwrap();
                let index2 = val.1.unwrap();
                if index1 < first {
                    first = index1;
                    x = parse_num(key);
                }
                if index1 > last {
                    last = index2;
                    y = parse_num(key);
                }
                if index2 < first {
                    first = index1;
                    x = parse_num(key);
                }
                if index2 > last {
                    last = index2;
                    y = parse_num(key);
                }
            }
        }
        let num = x * 10 + y;
        writeln!(op, "{line} : {num}");
        sum += num;
    }
    sum
}

fn parse_num(num: &str) -> u64 {
    return match num {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => num.parse::<u64>().unwrap()
    }
}

fn available_nums(str: &str) -> HashMap<&str, (Option<usize>, Option<usize>)> {
    let mut x = HashMap::new();
    for i in NUMS {
        x.insert(i, (str.find(i), str.rfind(i)));
    }
    x
}

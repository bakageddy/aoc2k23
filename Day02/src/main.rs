use std::fs;
use std::cmp::max;

#[derive(Debug)]
struct Game {
    id: i64,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct GameSet {
    red: usize,
    blue: usize,
    green: usize,
}

impl Game {
    fn new(game_line: &str) -> Self {
        let name = game_line.split(":").collect::<Vec<&str>>();
        let id = name[0].split(" ").collect::<Vec<&str>>()[1].parse::<i64>().unwrap_or_else(|_| 0);

        let sets: Vec<GameSet> = name[1].split(";").map(|set: &str| {
            GameSet::new(set)
        }).collect();

        Self {
            id,
            sets
        }
    }

    fn contains(self: &Self, set: &GameSet) -> bool {
        for i in &self.sets {
            if i.red > set.red { return false }
            if i.green > set.green { return false }
            if i.blue > set.blue { return false }
        }
        true
    }

    fn least_possible(self: &Self) -> GameSet {
        let mut max_set = GameSet::blank();
        for set in &self.sets {
            max_set.red = max(max_set.red, set.red);
            max_set.blue = max(max_set.blue, set.blue);
            max_set.green = max(max_set.green, set.green);
        }
        max_set
    }
}

impl GameSet {
    fn blank() -> Self {
        Self {
            red: 0,
            green:0,
            blue: 0,
        }
    }

    fn power(self: &Self) -> usize {
        self.red * self.green * self.blue
    }

    fn new(set: &str) -> Self {
        let mut ans = Self {blue: 0, green: 0, red: 0};
        for cubes in set.split(",") {
            let cubes = cubes.trim_start().split(" ").collect::<Vec<&str>>();
            let count = cubes[0].parse::<usize>().ok();
            match cubes[1] {
                "blue" => { ans.blue = count.unwrap_or(0); },
                "green" => { ans.green = count.unwrap_or(0); },
                "red" => { ans.red = count.unwrap_or(0); },
                _ => panic!("Wtf do i do?"),

            };
        }
        ans
    }
}


fn main() {
    let input_file = "./input.txt";
    // let possible_game_set = GameSet {
    //     red: 12,
    //     green: 13,
    //     blue: 14,
    // };
    // let ans = part1(input_file, &possible_game_set);
    let ans = part2(input_file);
    println!("{ans}");
}

fn part1(filename: &str, game_set: &GameSet) -> i64 {
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        panic!("Failed to open file!");
    });

    let mut sum = 0;
    for game in contents.lines() {
        let game = Game::new(game);
        if game.contains(game_set) {
            sum += game.id;
        }
    }
    sum
}

fn part2(filename: &str) -> usize {
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        panic!("Failed to open file!");
    });
    let mut sum = 0;
    for game in contents.lines() {
        let game = Game::new(game);
        sum += game.least_possible().power();
    }
    sum
}

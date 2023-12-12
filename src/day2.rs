use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;
use std::cmp::max;

pub fn part_one() -> Result<(), Error> {
    let mut numbers = HashMap::new();
    numbers.insert("red", 12);
    numbers.insert("green", 13);
    numbers.insert("blue", 14);

    let file_path = env::current_dir()?.join(r"src\input\day2.txt");
    let mut id_total = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let game_split: Vec<&str> = line.split(":").collect();
        let game_id: i32 = game_split[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let games = game_split[1].split(";");
        let mut is_valid = true;

        for game in games {
            for color in game.split(",") {
                let num_color: Vec<&str> = color.split(" ").filter(|x| x.len() > 0).collect();

                if is_valid {
                    is_valid = num_color[0].parse::<i32>().unwrap() <= *numbers.get(num_color[1]).unwrap();
                }
            }
        }

        if is_valid {
            id_total += game_id;
        }
    }

    println!("Id total {}", id_total.to_string());

    return Ok(());
}


pub fn part_two() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day2.txt");
    let mut power_total = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let game_split: Vec<&str> = line.split(":").collect();
        let games = game_split[1].split(";");
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for game in games {
            for color in game.split(",") {
                let num_color: Vec<&str> = color.split(" ").filter(|x| x.len() > 0).collect();
                let num = num_color[0].parse::<i32>().unwrap();
                let color = num_color[1];

                match color {
                    "red" => max_red = max(max_red, num),
                    "blue" => max_blue = max(max_blue, num),
                    "green" => max_green = max(max_green, num),
                    _ => (),
                }
            }
        }

        power_total += max_red * max_blue * max_green;
    }

    println!("Id total {}", power_total.to_string());

    return Ok(());
}
use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;

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
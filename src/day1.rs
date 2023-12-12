use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;

pub fn part_one() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day1.txt");
    let mut total = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let converted: Vec<i32> = line
            .split("")
            .map(|c| c.parse::<i32>())
            .filter(|r| !r.is_err())
            .map(|c| c.unwrap())
            .collect();

        let calib = [converted.first().unwrap().to_string(), converted.last().unwrap().to_string()].join("");
        total += calib.parse::<i32>().unwrap();
    }

    println!("Calibration total {}", total.to_string());

    return Ok(());
}

pub fn part_two() -> Result<(), Error>{
    let mut numbers = HashMap::new();
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);

    let file_path = env::current_dir()?.join(r"src\input\day1.txt");
    let mut total = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut calibration_numbers: Vec<i32> = vec![];
        let split_line: Vec<&str> = line.split("").collect();

        for (i, c) in split_line.iter().enumerate() {
            let parsed_char = c.parse::<i32>();

            if !parsed_char.is_err() {
                calibration_numbers.push(parsed_char.unwrap());
            }
            else if i < split_line.len() - 3 {
                // This block is hilarious
                let mut digit_word: Vec<&str> = vec![c];

                for n in 1..5 {
                    if i + n < split_line.len() {
                        digit_word.push(split_line[i + n]);
                        let word: String = digit_word.join("");
                        if numbers.contains_key(word.as_str()) {
                            calibration_numbers.push(*numbers.get(word.as_str()).unwrap());
                            break;
                        }
                    }
                }
            }
        }


        let calib = [calibration_numbers.first().unwrap().to_string(), calibration_numbers.last().unwrap().to_string()].join("");
        total += calib.parse::<i32>().unwrap();
    }

    println!("Calibration total {}", total.to_string());

    return Ok(());
}
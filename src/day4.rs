use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;

pub fn part_one() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day4.txt");
    let base: i64 = 2;
    let mut scratch_sum = 0;

    let file = fs::read_to_string(file_path).unwrap();

    for line in file.lines() {
        let card: Vec<&str> = line.split(':').collect();
        let card_nums: Vec<&str> = card[1].split('|').collect();
        let win_nums = card_nums[0];
        let nums = card_nums[1];
        let mut win_map = HashMap::new();

        for win_num in win_nums.split(' ').filter(|n| !n.is_empty()) {
            win_map.insert(win_num.parse::<i64>().unwrap(), win_num);
        }

        let matches: u32 = nums
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|num| if win_map.contains_key(&num.parse::<i64>().unwrap()) { 1 } else { 0 })
            .sum();

        if matches > 0 {
            scratch_sum += base.pow(matches - 1)
        }
    }

    println!("Scratch sum {}", scratch_sum);

    Ok(())
}
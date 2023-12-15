use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;
use std::convert::TryFrom;

fn get_matches(line: &str) -> u32 {
    let card: Vec<&str> = line.split(':').collect();
    let card_nums: Vec<&str> = card[1].split('|').collect();
    let win_nums = card_nums[0];
    let nums = card_nums[1];
    let mut win_map = HashMap::new();

    for win_num in win_nums.split(' ').filter(|n| !n.is_empty()) {
        win_map.insert(win_num.parse::<i64>().unwrap(), win_num);
    }

    nums
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|num| if win_map.contains_key(&num.parse::<i64>().unwrap()) { 1 } else { 0 })
        .sum()
}

pub fn part_one() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day4.txt");
    let base: i64 = 2;
    let mut scratch_sum = 0;

    let file = fs::read_to_string(file_path).unwrap();

    for line in file.lines() {
        let matches = get_matches(line);

        if matches > 0 {
            scratch_sum += base.pow(matches - 1)
        }
    }

    println!("Scratch sum {}", scratch_sum);

    Ok(())
}

pub fn part_two() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day4.txt");
    let mut scratch_sum = 0;

    let file = fs::read_to_string(file_path).unwrap();
    let mut card_queue = VecDeque::new();
    let mut card_map = HashMap::new();

    for (card_idx, line) in file.lines().enumerate() {
        let matches = usize::try_from(get_matches(line)).unwrap();

        card_queue.push_back(card_idx + 1);
        card_map.insert(card_idx + 1, matches);
    }

    while !card_queue.is_empty() {
        let idx = card_queue.pop_front().unwrap();
        let next_line_matches = card_map.get(&idx).unwrap();

        for i in idx + 1..idx + next_line_matches + 1 {
            card_queue.push_back(i);
        }

        scratch_sum += 1;
    }

    println!("Card total {}", scratch_sum);

    Ok(())
}
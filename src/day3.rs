use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;

pub fn part_one() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day3.txt");
    let mut symbol_map = HashMap::new();
    let mut part_sum = 0;

    let mut row: i64 = 0;

    let file = fs::read_to_string(file_path).unwrap();

    for line in file.lines() {
        let mut col:i64 = 0;

        for (_, c) in line.char_indices() {
            if !c.is_ascii_digit() && c != '.' {
                symbol_map.insert([row, col], c);
            }

            col += 1;
        }

        row += 1;
    }

    row = 0;

    for line in file.lines() {
        let mut current_number: Vec<char> = vec![];
        let mut current_cols = vec![];
        let mut col:i64 = 0;

        for (_, c) in line.char_indices() {
            if c.is_ascii_digit() {
                current_number.push(c);
                current_cols.push(col);
            }

            if !c.is_ascii_digit() || col == (line.len() - 1).try_into().unwrap() {
                if current_number.len() > 0 {
                    let num_string: String = current_number.into_iter().collect();
                    let num: i64 = num_string.parse::<i64>().unwrap();

                    let mut is_near_symbol = false;
                    for i in current_cols[0] - 1..current_cols[current_cols.len() - 1] + 2 {
                        for j in row - 1..row + 2 {
                            if symbol_map.contains_key(&[j, i]) {
                                is_near_symbol = true;
                                break;
                            }
                        }
                    }

                    if is_near_symbol {
                        part_sum += num;
                    }
                }

                current_number = vec![];
                current_cols = vec![];

            }

            col += 1;
        }

        row += 1;
    }

    println!("Part sum {}", part_sum.to_string());

    return Ok(());
}
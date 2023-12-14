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

    // Create map of all symbols and store their coordinates
    for line in file.lines() {
        for (col, (_, c)) in (0_i64..).zip(line.char_indices()) {
            if !c.is_ascii_digit() && c != '.' {
                symbol_map.insert([row, col], c);
            }
        }

        row += 1;
    }

    row = 0;

    for line in file.lines() {
        let mut current_number: Vec<char> = vec![];
        let mut current_cols = vec![];

        // Iterate over each character in the line
        for (col, (_, c)) in (0_i64..).zip(line.char_indices()) {
            // Construct latest number
            if c.is_ascii_digit() {
                current_number.push(c);
                current_cols.push(col);
            }

            if !c.is_ascii_digit() || col == (line.len() - 1).try_into().unwrap() {
                if !current_number.is_empty() {
                    let num_string: String = current_number.into_iter().collect();
                    let num: i64 = num_string.parse::<i64>().unwrap();

                    // Check if number is adjance to symbol by checking if any of the coordinates
                    // around the number exist in the symbol map
                    let mut is_near_symbol = false;
                    for i in current_cols[0] - 1..current_cols[current_cols.len() - 1] + 2 {
                        for j in row - 1..row + 2 {
                            if symbol_map.contains_key(&[j, i]) {
                                is_near_symbol = true;
                                break;
                            }
                        }

                        if is_near_symbol {
                            break;
                        }
                    }

                    if is_near_symbol {
                        part_sum += num;
                    }
                }

                current_number = vec![];
                current_cols = vec![];

            }
        }

        row += 1;
    }

    println!("Part sum {}", part_sum);

    Ok(())
}

pub fn part_two() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day3.txt");
    let mut symbol_map = HashMap::new();
    let mut row: i64 = 0;

    let file = fs::read_to_string(file_path).unwrap();

    // Create map of all star symbols and store their coordinates
    for line in file.lines() {
        for (col, (_, c)) in (0_i64..).zip(line.char_indices()) {
            if c == '*' {
                symbol_map.insert([row, col], c);
            }
        }

        row += 1;
    }

    let mut gear_ratios = HashMap::new();
    row = 0;

    for line in file.lines() {
        let mut current_number: Vec<char> = vec![];
        let mut current_cols = vec![];

        for (col, (_, c)) in (0_i64..).zip(line.char_indices()) {
            if c.is_ascii_digit() {
                current_number.push(c);
                current_cols.push(col);
            }

            if !c.is_ascii_digit() || col == (line.len() - 1).try_into().unwrap() {
                if !current_number.is_empty() {
                    let num_string: String = current_number.into_iter().collect();
                    let num: i64 = num_string.parse::<i64>().unwrap();

                    for i in current_cols[0] - 1..current_cols[current_cols.len() - 1] + 2 {
                        for j in row - 1..row + 2 {
                            if symbol_map.contains_key(&[j, i]) {
                                if !gear_ratios.contains_key(&[j, i]) {
                                    let adjacent_part_nums = vec![num];
                                    gear_ratios.insert([j, i], adjacent_part_nums);
                                } else {
                                    let adjacent_part_nums: &mut Vec<i64> = gear_ratios.get_mut(&[j, i]).unwrap();
                                    adjacent_part_nums.push(num);
                                }
                            }
                        }
                    }
                }

                current_number = vec![];
                current_cols = vec![];

            }
        }

        row += 1;
    }

    let mut total_gear_ratio = 0;

    for (_, v) in gear_ratios {
        if v.len() == 2 {
            total_gear_ratio += v[0] * v[1];
        }
    }

    println!("Part sum {}", total_gear_ratio);

    Ok(())
}
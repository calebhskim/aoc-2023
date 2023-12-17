
use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;
use std::cmp;
use regex::Regex;

fn get_mapped_value(val: u64, m: &HashMap<u64, (u64, u64)>) -> u64 {
    for (k, v) in m {
        if val >= *k && val < *k + v.1 {
            let (dest_start, _) = v;
            let offset = val - k;

            return dest_start + offset;
        }
    }

    val
}

pub fn part_one() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day5.txt");
    let file = fs::read_to_string(file_path).unwrap();

    let mut lines = file.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap());
    let mut next_line = lines.next();
    let map_re = Regex::new(r"(?<name>[a-z,-]+) map:").unwrap();
    let mut almanac = HashMap::from([
        ("seed-to-soil", HashMap::<u64, (u64, u64)>::new()),
        ("soil-to-fertilizer", HashMap::<u64, (u64, u64)>::new()),
        ("fertilizer-to-water", HashMap::<u64, (u64, u64)>::new()),
        ("water-to-light", HashMap::<u64, (u64, u64)>::new()),
        ("light-to-temperature", HashMap::<u64, (u64, u64)>::new()),
        ("temperature-to-humidity", HashMap::<u64, (u64, u64)>::new()),
        ("humidity-to-location", HashMap::<u64, (u64, u64)>::new()),
    ]);

    let mut current_map = "";
    let mut ready_for_new_map = true;

    while next_line.is_some() {
        let line = next_line.unwrap();

        if line.is_empty() {
            ready_for_new_map = true;
        }
        else if ready_for_new_map {
            let map_name_match = map_re.captures(line);

            if map_name_match.is_some() {
                current_map = map_name_match.unwrap().name("name").unwrap().as_str();
                ready_for_new_map = false;
            }
        }
        else {
            let mapping_range: Vec<&str> = line.trim().split(' ').collect();
            let dest_start = mapping_range[0].parse::<u64>().unwrap();
            let src_start = mapping_range[1].parse::<u64>().unwrap();
            let range_len = mapping_range[2].parse::<u64>().unwrap();

            almanac.get_mut(current_map).unwrap().insert(src_start, (dest_start, range_len));
        }

        next_line = lines.next();
    }

    let mut min_location = u64::MAX;

    for seed in seeds {
        let soil = get_mapped_value(seed, almanac.get("seed-to-soil").unwrap());
        let fertilizer = get_mapped_value(soil, almanac.get("soil-to-fertilizer").unwrap());
        let water = get_mapped_value(fertilizer, almanac.get("fertilizer-to-water").unwrap());
        let light = get_mapped_value(water, almanac.get("water-to-light").unwrap());
        let temperature = get_mapped_value(light, almanac.get("light-to-temperature").unwrap());
        let humidity = get_mapped_value(temperature, almanac.get("temperature-to-humidity").unwrap());
        let location = get_mapped_value(humidity, almanac.get("humidity-to-location").unwrap());

        min_location = cmp::min(min_location, location);
    }

    println!("Min location: {}", min_location);

    Ok(())
}
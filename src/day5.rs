
use std::env;
use std::fs;
use std::io::Error;
use std::collections::HashMap;
use std::cmp::min;
use std::cmp::Ordering;
use std::str::Lines;
use itertools::Itertools;
use regex::Regex;

fn get_mapped_value(val: u64, m: &HashMap<(u64, u64), (u64, u64)>) -> u64 {
    for (k, v) in m {
        let (k_start, k_end) = *k;
        let (v_start, _) = *v;

        if val >= k_start && val < k_end {
            let offset = val - k_start;
            return v_start + offset;
        }
    }

    val
}

fn get_range_almanac(mut lines: Lines, reverse: bool) -> HashMap<&str, HashMap<(u64, u64), (u64, u64)>> {
    let mut next_line = lines.next();
    let map_re = Regex::new(r"(?<name>[a-z,-]+) map:").unwrap();
    let mut almanac = HashMap::from([
        ("seed-to-soil", HashMap::<(u64, u64), (u64, u64)>::new()),
        ("soil-to-fertilizer", HashMap::<(u64, u64), (u64, u64)>::new()),
        ("fertilizer-to-water", HashMap::<(u64, u64), (u64, u64)>::new()),
        ("water-to-light", HashMap::<(u64, u64), (u64, u64)>::new()),
        ("light-to-temperature", HashMap::<(u64, u64), (u64, u64)>::new()),
        ("temperature-to-humidity", HashMap::<(u64, u64), (u64, u64)>::new()),
        ("humidity-to-location", HashMap::<(u64, u64), (u64, u64)>::new()),
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

            if reverse {
                almanac.get_mut(current_map).unwrap().insert((dest_start, dest_start + range_len), (src_start, src_start + range_len));
            }
            else {
                almanac.get_mut(current_map).unwrap().insert((src_start, src_start + range_len), (dest_start, dest_start + range_len));
            }
        }

        next_line = lines.next();
    }

    almanac
}

// Add in-between ranges to ranges specified in map
fn add_all_ranges(ranges: &HashMap<(u64, u64), (u64, u64)>) -> HashMap<(u64, u64), (u64, u64)> {
    let mut new_ranges = HashMap::<(u64, u64), (u64, u64)>::new();

    let mut current_start = 0;

    for range in ranges.keys().sorted() {
        let (k_start, k_end) = *range;
        let (v_start, v_end) = *ranges.get(range).unwrap();

        match k_start.cmp(&current_start) {
            Ordering::Equal => {
                current_start = k_end;
            },
            Ordering::Greater => {
                new_ranges.insert((current_start, k_start), (current_start, k_start));
            },
            Ordering::Less => (),
        };
        current_start = k_end;

        new_ranges.insert((k_start, k_end), (v_start, v_end));
    }

    new_ranges
}

fn get_min_location(seed: u64, almanac: &HashMap<&str, HashMap<(u64, u64), (u64, u64)>>) -> u64 {
    let soil = get_mapped_value(seed, almanac.get("seed-to-soil").unwrap());
    let fertilizer = get_mapped_value(soil, almanac.get("soil-to-fertilizer").unwrap());
    let water = get_mapped_value(fertilizer, almanac.get("fertilizer-to-water").unwrap());
    let light = get_mapped_value(water, almanac.get("water-to-light").unwrap());
    let temperature = get_mapped_value(light, almanac.get("light-to-temperature").unwrap());
    let humidity = get_mapped_value(temperature, almanac.get("temperature-to-humidity").unwrap());
    let location = get_mapped_value(humidity, almanac.get("humidity-to-location").unwrap());

    location
}

pub fn part_one() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day5.txt");
    let file = fs::read_to_string(file_path).unwrap();

    let mut lines = file.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let almanac = get_range_almanac(lines, false);

    let mut min_location = u64::MAX;

    for seed in seeds {
        let location = get_min_location(seed, &almanac);

        min_location = min(min_location, location);
    }

    println!("Min location: {}", min_location);

    Ok(())
}

fn find_range(
    map_list: &Vec<&str>,
    almanac: &HashMap<&str, HashMap<(u64, u64), (u64, u64)>>,
    start: u64,
    end: u64
) -> Vec<(u64, u64)> {
    if map_list.is_empty() {
        return vec![(start, end)];
    }

    let map_name = *map_list.first().unwrap();
    let ranges = almanac.get(map_name).unwrap();

    for range in ranges.keys().sorted() {
        let (dest_start, dest_end) = *range;
        let (src_start, src_end) = *ranges.get(range).unwrap();

        if start >= dest_start && start < dest_end {
            if end <= dest_end {
                let new_start = src_start + (start - dest_start);
                let new_end = src_start + (start - dest_start) + (end - start);

                return find_range(&map_list[1..].to_vec(), almanac, new_start, new_end);
            }
            else {
                let new_start = src_start + (start - dest_start);
                let new_end = src_end;

                let mut range_1 = find_range(&map_list[1..].to_vec(), almanac, new_start, new_end);
                let mut range_2 = find_range(&map_list.clone(), almanac, dest_end, end);

                range_1.append(&mut range_2);

                return range_1;
            }
        }
    }

    vec![]
}

fn find_seed_range(found_seed_ranges: &Vec<(u64, u64)>, seed_ranges_parsed: &[(u64, u64)]) -> Option<(u64, u64)> {
    for r in found_seed_ranges {
        for seed_range in seed_ranges_parsed.iter().copied() {
            if r.0 >= seed_range.0 && r.1 <= seed_range.1 {
                return Some((r.0, r.1));
            }
        }
    }

    None
}

pub fn part_two() -> Result<(), Error> {
    let file_path = env::current_dir()?.join(r"src\input\day5.txt");
    let file = fs::read_to_string(file_path).unwrap();

    let mut lines = file.lines();
    let seed_ranges: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut seed_ranges_parsed = Vec::<(u64, u64)>::new();

    for i in (0..seed_ranges.len()).step_by(2) {
        let range_start = seed_ranges[i];
        let range = seed_ranges[i + 1];
        seed_ranges_parsed.push((range_start, range_start + range));
    }

    // Create a reverse almanac mapping destination start to source start and range length
    let n_almanac = get_range_almanac(lines.clone(), false);
    let almanac = get_range_almanac(lines.clone(), true);
    let mut added_almanac = HashMap::<&str, HashMap<(u64, u64), (u64, u64)>>::new();
    for (k, v) in almanac.clone() {
        added_almanac.insert(k, add_all_ranges(&v));
    }

    let all_maps = [
        // "humidity-to-location", -> Exclude this map because it is the map we are searching through
        "temperature-to-humidity",
        "light-to-temperature",
        "water-to-light",
        "fertilizer-to-water",
        "soil-to-fertilizer",
        "seed-to-soil",
    ];
    // Map location ranges to humidity ranges
    let added_hum_ranges = added_almanac.get("humidity-to-location").unwrap();

    for l_range in added_hum_ranges.keys().sorted() {
        let h_range = added_hum_ranges.get(l_range).unwrap();
        let found = find_range(&all_maps.to_vec(), &added_almanac, h_range.0, h_range.1);

        if !found.is_empty() {
            let maybe_range = find_seed_range(&found, &seed_ranges_parsed);

            if maybe_range.is_some() {
                let (start, _) = maybe_range.unwrap();

                let mut min_location = u64::MAX; 

                // Assume you only need to check start
                min_location = min(get_min_location(start, &n_almanac), min_location);

                println!("Min location: {}", min_location);
                break;
            }
        }
    }

    Ok(())
}
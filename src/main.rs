use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
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

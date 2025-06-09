use std::env;

mod year_2024;

pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut year: Option<i32> = None;
    let mut day: Option<i32> = None;
    let mut part: Option<i32> = Some(1);
    let mut example: Option<bool> = Some(false);

    for arg in &args[1..] {
        if let Some(val) = arg.strip_prefix("--year=") {
            year = val.parse::<i32>().ok();
        } else if let Some(val) = arg.strip_prefix("--day=") {
            day = val.parse::<i32>().ok();
        } else if let Some(val) = arg.strip_prefix("--part=") {
            part = val.parse::<i32>().ok();
        } else if let Some(val) = arg.strip_prefix("--example=") {
            example = val.parse::<bool>().ok();
        }
    }

    let year = year.unwrap_or(2024);
    let day = day.unwrap_or(1);

    match year {
        2024 => match day {
            1 => year_2024::day_01::main(part, example),
            2 => year_2024::day_02::main(part, example),
            3 => year_2024::day_03::main(part, example),
            4 => year_2024::day_04::main(part, example),
            5 => year_2024::day_05::main(part, example),
            _ => Err(format!("Day {} not implemented for year {}.", day, year).into()),
        },
        _ => Err(format!("Year {} not implemented.", year).into()),
    }
}

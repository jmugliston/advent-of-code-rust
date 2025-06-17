use std::env;

use chrono::Datelike;
use clap::Parser;
use dotenv::dotenv;

mod aoc;
pub mod utils;

use clap::Subcommand;
use clap::ValueEnum;
use solvers::SOLVERS;

mod solvers;

const BASE_URL: &str = "https://adventofcode.com";
const USER_AGENT: &str = concat!(
    "github.com/jmugliston/aoc-rust/ v",
    env!("CARGO_PKG_VERSION")
);

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Part {
    One,
    Two,
}

#[derive(Parser, Debug)]
#[command(name = "CLI", version, about = "AoC CLI Tool")]
struct Cli {
    #[command(subcommand)]
    command: MainCommand,
}

#[derive(Subcommand, Debug)]
#[command(
    version, about,
    long_about = None,
)]
enum MainCommand {
    Init(CommandArgs),
    Download(CommandArgs),
    Solve(SolveArgs),
}

#[derive(Parser, Debug)]
struct CommandArgs {
    /// Year (required)
    #[arg(short, long, default_value_t = chrono::Local::now().year())]
    year: i32,

    /// Day (required)
    #[arg(short, long, default_value_t = if chrono::Local::now().month() == 12 { chrono::Local::now().day() as i32 } else { 1 })]
    day: i32,
}

#[derive(Parser, Debug)]
struct SolveArgs {
    /// Year (required)
    #[arg(short, long, default_value_t = chrono::Local::now().year())]
    year: i32,

    /// Day (required)
    #[arg(short, long, default_value_t = if chrono::Local::now().month() == 12 { chrono::Local::now().day() as i32 } else { 1 })]
    day: i32,

    /// Part (optional, defaults to 1)
    #[arg(short, long, value_parser = clap::value_parser!(i8).range(1..=2))]
    part: i8,

    /// Use the example file as input (defaults to false)
    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        MainCommand::Init(args) => {
            return init(args.year, args.day);
        }
        MainCommand::Download(args) => {
            return download(args.year, args.day);
        }
        MainCommand::Solve(args) => {
            return solve(
                args.year,
                args.day,
                match args.part {
                    1 => Part::One,
                    2 => Part::Two,
                    _ => return Err(format!("Invalid part").into()),
                },
                args.example,
            );
        }
    }
}

fn init(year: i32, day: i32) -> Result<(), Box<dyn std::error::Error>> {
    let year_folder = format!("src/aoc/year_{:04}", year);
    if !std::path::Path::new(&year_folder).exists() {
        std::fs::create_dir_all(&year_folder)?;
        let mod_rs_path = format!("{}/mod.rs", year_folder);
        if !std::path::Path::new(&mod_rs_path).exists() {
            std::fs::write(&mod_rs_path, "")?;
        }
        // Update the mod file in the year folder to import the new day
        let aoc_mod_rs_path = "src/aoc/mod.rs";
        let mod_entry = format!("pub mod year_{:04};\n", year);

        let mut mod_rs_content = if std::path::Path::new(&aoc_mod_rs_path).exists() {
            std::fs::read_to_string(&aoc_mod_rs_path)?
        } else {
            String::new()
        };

        if !mod_rs_content.contains(&mod_entry) {
            mod_rs_content.push_str(&mod_entry);
            std::fs::write(&aoc_mod_rs_path, mod_rs_content)?;
        }
    }

    let src_folder = "template/day_00";
    let dest_folder = format!("src/aoc/year_{:04}/day_{:02}", year, day);

    if !std::path::Path::new(&dest_folder).exists() {
        std::fs::create_dir_all(&dest_folder)?;
    }

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = false;
    options.content_only = true;
    fs_extra::dir::copy(src_folder, &dest_folder, &options)?;

    // Find and replace year_0000 with the year and day_00 with the day
    let mod_rs_path = format!("{}/mod.rs", dest_folder);
    if std::path::Path::new(&mod_rs_path).exists() {
        let content = std::fs::read_to_string(&mod_rs_path)?;
        let content = content
            .replace("year_0000", &format!("year_{:04}", year))
            .replace("day_00", &format!("day_{:02}", day));
        std::fs::write(&mod_rs_path, content)?;
    }

    // Update the mod file in the year folder to import the new day
    let year_mod_rs_path = format!("src/aoc/year_{:04}/mod.rs", year);
    let mod_entry = format!("pub mod day_{:02};\n", day);

    let mut mod_rs_content = if std::path::Path::new(&year_mod_rs_path).exists() {
        std::fs::read_to_string(&year_mod_rs_path)?
    } else {
        String::new()
    };

    if !mod_rs_content.contains(&mod_entry) {
        mod_rs_content.push_str(&mod_entry);
        std::fs::write(&year_mod_rs_path, mod_rs_content)?;
    }

    let mut years = Vec::new();
    let mut days = std::collections::HashMap::new();

    for entry in std::fs::read_dir("src/aoc")? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        if let Some(year_str) = file_name.strip_prefix("year_") {
            if let Ok(year_num) = year_str.parse::<i32>() {
                let mut day_list = Vec::new();
                for day_entry in std::fs::read_dir(entry.path())? {
                    let day_entry = day_entry?;
                    let day_name = day_entry.file_name().to_string_lossy().to_string();
                    if let Some(day_str) = day_name.strip_prefix("day_") {
                        if let Ok(day_num) = day_str.parse::<i32>() {
                            day_list.push(day_num);
                        }
                    }
                }
                if !day_list.is_empty() {
                    years.push(year_num);
                    days.insert(year_num, day_list);
                }
            }
        }
    }

    // Update the solvers map
    let solvers_rs_path = "src/solvers.rs";
    let solvers_rs_content = std::fs::read_to_string(solvers_rs_path)?;
    let start_marker = "// --- AUTO GENERATED MAP START ---";

    let start_idx = solvers_rs_content
        .find(start_marker)
        .ok_or("Start marker not found in solvers.rs")?;

    let mut map_entries = String::new();
    let mut years_sorted = years.clone();
    years_sorted.sort();
    for year in years_sorted {
        let mut day_list = days.get(&year).cloned().unwrap_or_default();
        day_list.sort();
        for day in day_list {
            map_entries.push_str(&format!(
                "    m.insert(({}, {}), aoc::year_{:04}::day_{:02}::main as SolveFn);\n",
                year, day, year, day
            ));
        }
    }

    let new_map = format!(
        "{start}// --- AUTO GENERATED MAP START ---
pub static SOLVERS: Lazy<HashMap<(i32, i32), SolveFn>> = Lazy::new(|| {{
    let mut m = HashMap::new();
{entries}    
    return m;
}});
// --- AUTO GENERATED MAP END ---
",
        start = &solvers_rs_content[..start_idx],
        entries = map_entries,
    );

    std::fs::write(solvers_rs_path, new_map)?;

    return download(year, day);
}

fn download(year: i32, day: i32) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/{}/day/{}/input", BASE_URL, year, day);

    let session_token = env::var("SESSION_TOKEN")?;

    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header("Cookie", format!("session={}", session_token))
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp
            .text()
            .unwrap_or_else(|_| "<failed to read body>".to_string());
        eprintln!(
            "Error: received status {} from server. Body: {}",
            status, body
        );
        return Err(format!("Failed to download input: status {}", status).into());
    }

    let resp_text = resp.text()?;

    let input_path = format!("src/aoc/year_{:04}/day_{:02}/input/input.txt", year, day);
    std::fs::write(&input_path, &resp_text)?;

    return Ok(());
}

fn solve(year: i32, day: i32, part: Part, example: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(solver) = SOLVERS.get(&(year, day)) {
        solver(part, example)
    } else {
        Err(format!("Day {} not implemented for year {}.", day, year).into())
    }
}

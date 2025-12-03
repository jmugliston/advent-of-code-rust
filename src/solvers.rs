use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::Part;

use crate::aoc;

type SolveFn = fn(Part, bool) -> Result<(), Box<dyn std::error::Error>>;

// --- AUTO GENERATED MAP START ---
pub static SOLVERS: Lazy<HashMap<(i32, i32), SolveFn>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert((2024, 1), aoc::year_2024::day_01::main as SolveFn);
    m.insert((2024, 2), aoc::year_2024::day_02::main as SolveFn);
    m.insert((2024, 3), aoc::year_2024::day_03::main as SolveFn);
    m.insert((2024, 4), aoc::year_2024::day_04::main as SolveFn);
    m.insert((2024, 5), aoc::year_2024::day_05::main as SolveFn);
    m.insert((2024, 6), aoc::year_2024::day_06::main as SolveFn);
    m.insert((2024, 7), aoc::year_2024::day_07::main as SolveFn);
    m.insert((2024, 8), aoc::year_2024::day_08::main as SolveFn);
    m.insert((2024, 9), aoc::year_2024::day_09::main as SolveFn);
    m.insert((2024, 10), aoc::year_2024::day_10::main as SolveFn);
    m.insert((2024, 11), aoc::year_2024::day_11::main as SolveFn);
    m.insert((2024, 12), aoc::year_2024::day_12::main as SolveFn);
    m.insert((2024, 13), aoc::year_2024::day_13::main as SolveFn);
    m.insert((2024, 14), aoc::year_2024::day_14::main as SolveFn);
    m.insert((2024, 15), aoc::year_2024::day_15::main as SolveFn);
    m.insert((2024, 16), aoc::year_2024::day_16::main as SolveFn);
    m.insert((2024, 17), aoc::year_2024::day_17::main as SolveFn);
    m.insert((2024, 18), aoc::year_2024::day_18::main as SolveFn);
    m.insert((2024, 19), aoc::year_2024::day_19::main as SolveFn);
    m.insert((2024, 20), aoc::year_2024::day_20::main as SolveFn);
    m.insert((2024, 21), aoc::year_2024::day_21::main as SolveFn);
    m.insert((2024, 22), aoc::year_2024::day_22::main as SolveFn);
    m.insert((2024, 23), aoc::year_2024::day_23::main as SolveFn);
    m.insert((2024, 24), aoc::year_2024::day_24::main as SolveFn);
    m.insert((2024, 25), aoc::year_2024::day_25::main as SolveFn);
    m.insert((2025, 1), aoc::year_2025::day_01::main as SolveFn);
    m.insert((2025, 2), aoc::year_2025::day_02::main as SolveFn);
    m.insert((2025, 3), aoc::year_2025::day_03::main as SolveFn);
    
    return m;
});
// --- AUTO GENERATED MAP END ---

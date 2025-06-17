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

    return m;
});
// --- AUTO GENERATED MAP END ---

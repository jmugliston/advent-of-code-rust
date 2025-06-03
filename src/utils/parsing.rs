pub fn read_lines(input: &str) -> Vec<String> {
    return input.lines().map(|line| line.to_string()).collect();
}

pub fn read_numbers(input: &str) -> Vec<i32> {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i32>().expect("Invalid number"))
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect();
}

pub fn read_lines_of_numbers(input: &str) -> Vec<Vec<i32>> {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i32>().expect("Invalid number"))
                .collect::<Vec<i32>>()
        })
        .collect();
}

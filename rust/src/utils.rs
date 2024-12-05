use std::fs;

pub fn parse_file(filename: &str) -> Vec<String> {
    let file_path = format!("./inputs/{filename}.txt");
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    contents
        .split('\n')
        .map(str::to_string)
        .filter(|line| !line.is_empty())
        .collect()
}

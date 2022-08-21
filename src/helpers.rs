use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use cli_table::{Cell, CellStruct};

pub fn read_lines(data_path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(data_path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn map_lines_to_data(lines: Lines<BufReader<File>>, num_of_col: usize) -> Vec<(usize, Vec<String>)> {
    lines
        .filter_map(|result| result.ok())
        .map(|line: String| {
            line.split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .enumerate()
        .filter(|(_, vec)| vec.len() == num_of_col && vec.iter().any(|s| !s.is_empty()))
        .collect()
}

pub fn map_data_to_cells<S: AsRef<str>>(data: &[Vec<S>]) -> Vec<Vec<CellStruct>> {
    data.iter()
        .map(|entry| entry.iter().map(|value| value.as_ref().cell()).collect())
        .collect()
}
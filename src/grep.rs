use colored::*;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

struct FoundOccurrence {
    path_str: String,
    line_number: u32,
    line_str: String,
}

impl FoundOccurrence {
    pub fn new(file_path: &Path, line_number: usize, line_content: String) -> FoundOccurrence {
        FoundOccurrence {
            path_str: file_path.to_str().unwrap().to_owned(),
            line_number: line_number as u32,
            line_str: line_content,
        }
    }
}

pub struct PathResult {
    scanned_files: u32,
    found_results: u32,
}

impl PathResult {
    fn new() -> PathResult {
        PathResult {
            scanned_files: 0,
            found_results: 0,
        }
    }

    pub fn scanned_files(&self) -> u32 {
        self.scanned_files
    }
    pub fn found_results(&self) -> u32 {
        self.found_results
    }
}

fn display_occurrence(occurrence: &FoundOccurrence) {
    println!(
        "{}::{}    {}",
        occurrence.path_str.bold().purple(),
        occurrence.line_number.to_string().green(),
        occurrence.line_str
    );
}

fn buffered_scan_file(
    file_path: &Path,
    str_to_find: &str,
) -> Result<Vec<FoundOccurrence>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .enumerate()
        .filter(|(_, line_content)| {
            if let Ok(line_content) = line_content {
                return line_content.contains(str_to_find);
            }
            false
        })
        .map(|(line_number, line_content)| (line_number, line_content.unwrap()))
        .map(|(line_number, line_content)| {
            let occurrence = FoundOccurrence::new(file_path, line_number, line_content);
            display_occurrence(&occurrence);
            occurrence
        })
        .collect::<Vec<FoundOccurrence>>())
}

pub fn search_str_in_path(path: &Path, str_to_find: &str) -> PathResult {
    let mut path_result = PathResult::new();
    if !path.is_dir() {
        path_result.scanned_files = 1;

        if let Ok(occurrences) = buffered_scan_file(path, str_to_find) {
            path_result.found_results = occurrences.len() as u32;
        }
        return path_result;
    }

    if let Ok(read_dir) = fs::read_dir(path) {
        let sub_dir_results: Vec<_> = read_dir
            .into_iter()
            .map(|entry| {
                let sub_path = entry.unwrap().path();
                search_str_in_path(&sub_path.as_path(), &str_to_find)
            })
            .collect();

        for sub_dir_result in sub_dir_results {
            path_result.scanned_files += sub_dir_result.scanned_files;
            path_result.found_results += sub_dir_result.found_results;
        }
    }

    path_result
}

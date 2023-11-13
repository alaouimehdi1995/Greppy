use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::thread;

#[derive(Debug)]
pub enum IOError {
    FileReadingError(io::Error),
    PathParsingError,
}

impl From<io::Error> for IOError {
    fn from(value: io::Error) -> Self {
        Self::FileReadingError(value)
    }
}

#[derive(Debug)]
pub struct FoundOccurrence {
    pub file_path: String,
    pub line_text: String,
    pub line_number: usize,
}

pub fn recursive_scan_path_for_target_text(
    target_text: String,
    path: &Path,
    display_function: fn(&FoundOccurrence),
) -> (u32, u32) {
    if !path.is_dir() {
        if let Ok(occurrence_count) =
            scan_file_for_target_text(&target_text, path, display_function)
        {
            return (1, occurrence_count);
        }
    }

    let Ok(read_dir) = fs::read_dir(path) else {
        return (0, 0);
    };

    let handles: Vec<_> = read_dir
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let target_text_copy = target_text.clone();
            thread::spawn(move || {
                recursive_scan_path_for_target_text(
                    target_text_copy,
                    entry.path().as_path(),
                    display_function,
                )
            })
        })
        .collect();

    let (mut scanned_files, mut occurrence_count) = (0, 0);
    for handle in handles {
        let (thread_scanned_files, thread_occurrence_count) = handle.join().unwrap_or((0, 0));
        scanned_files += thread_scanned_files;
        occurrence_count += thread_occurrence_count;
    }

    (scanned_files, occurrence_count)
}

fn scan_file_for_target_text(
    target_text: &str,
    file_path: &Path,
    display_function: fn(&FoundOccurrence),
) -> Result<u32, IOError> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let occurrence_count = reader
        .lines()
        .enumerate()
        .filter(|(_, line)| line.as_ref().is_ok_and(|l| l.contains(target_text)))
        .map(|(i, line)| {
            display_function(&FoundOccurrence {
                line_number: i,
                line_text: line?,
                file_path: file_path
                    .to_str()
                    .ok_or(IOError::PathParsingError)?
                    .to_string(),
            });
            Ok(())
        })
        .filter_map(|e: Result<(), IOError>| e.ok())
        .count();
    Ok(occurrence_count as u32)
}

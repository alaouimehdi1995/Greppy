mod grep;

use clap::Parser;
use colored::*;
use grep::FoundOccurrence;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    source_path: PathBuf,
    #[clap(short, long)]
    text_to_find: String,
}

fn display_occurrence(occurrence: &FoundOccurrence) {
    println!(
        "> {}::{}    {}",
        occurrence.file_path.bold().purple(),
        occurrence.line_number.to_string().green(),
        occurrence.line_text
    );
}

fn main() {
    let args = Args::parse();
    let now = Instant::now();
    let (scanned_files, result_count) = grep::recursive_scan_path_for_target_text(
        args.text_to_find,
        args.source_path.as_path(),
        display_occurrence,
    );
    let elapsed_time = now.elapsed();

    println!(
        "{:?} Results found in: {:.2?} (from {} files)",
        result_count, elapsed_time, scanned_files
    );
}

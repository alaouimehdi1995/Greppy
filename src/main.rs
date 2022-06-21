mod grep;
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    source_path: PathBuf,
    #[clap(short, long)]
    text_to_find: String,
}

fn main() {
    let args = Args::parse();
    let now = Instant::now();
    let results = grep::search_str_in_path(&args.source_path.as_path(), &args.text_to_find);
    let elapsed_time = now.elapsed();

    println!("{} Results found in: {:.2?} (from {} files)", results.found_results(), elapsed_time, results.scanned_files());
}

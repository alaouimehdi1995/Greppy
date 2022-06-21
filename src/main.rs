mod grep;
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, world! This is my grep program");
    let entered_path = Path::new("/home/mehdi/testenv");
    //let entered_path = Path::new("../");
    let string_to_find = "main()";

    let now = Instant::now();
    let results = grep::search_str_in_path(&entered_path, string_to_find);
    let elapsed = now.elapsed();

    println!("{} Results found in: {:.2?} (from {} files)", results.found_results(), elapsed, results.scanned_files());
}

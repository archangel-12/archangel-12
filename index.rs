use chrono::{Datelike, Utc};
use std::fs;

fn main() {
    let now = Utc::now();
    let this_year = now.year();

    let start_of_year = Utc.ymd(this_year, 1, 1).and_hms(0, 0, 0);
    let end_of_year = Utc.ymd(this_year, 12, 31).and_hms(23, 59, 59);

    let progress_of_this_year = (now.timestamp_millis() - start_of_year.timestamp_millis()) as f64
        / (end_of_year.timestamp_millis() - start_of_year.timestamp_millis()) as f64;

    let readme_path = "README.md";
    let mut readme_content = fs::read_to_string(readme_path).expect("Unable to read README.md");

    let progress_info = format!(
        "- just so you know, {} is {:.2}% complete\n",
        this_year,
        progress_of_this_year * 100.0
    );
    readme_content.push_str(&progress_info);

    fs::write(readme_path, readme_content).expect("Unable to write to README.md");
}
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let this_year = chrono::Utc::now().year();
    let start_of_year = SystemTime::from(UNIX_EPOCH)
        .checked_add(std::time::Duration::from_secs(
            (this_year as u64 - 1970) * 31_536_000,
        ))
        .expect("Invalid date");

    let seconds_in_year = 365 * 24 * 60 * 60;
    let progress_of_this_year = current_time
        .duration_since(start_of_year)
        .expect("Time calculation error")
        .as_secs() as f64
        / seconds_in_year as f64;

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
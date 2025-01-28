/* this rust code is inspired by this repo: */
/* https://github.com/liununu/liununu */
/* please leave him a star and hopefully if you see this, umm, well, gimme a star too („• ֊ •„) */
/* truth to be told, i just don't know why i use rust when you can do it in javascript :/ */
/* but anyway, this is my implementation: */

use std::fs;
use std::time::{ SystemTime, UNIX_EPOCH };

fn main() {
    let current_time = SystemTime::now();

    let current_year =
        1970 +
        current_time.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs() /
            31_536_000;

    let start_of_year = UNIX_EPOCH.checked_add(
        std::time::Duration::from_secs(((current_year as u64) - 1970) * 31_536_000)
    ).expect("invalid date");

    let is_leap_year =
        (current_year % 4 == 0 && current_year % 100 != 0) || current_year % 400 == 0;
    let seconds_in_year = if is_leap_year { 366 * 24 * 60 * 60 } else { 365 * 24 * 60 * 60 };

    let progress_of_this_year =
        (
            current_time
                .duration_since(start_of_year)
                .expect("time calculation error")
                .as_secs() as f64
        ) / (seconds_in_year as f64);

    let readme_path = "README.md";
    let readme_content = fs::read_to_string(readme_path).expect("unable to read README.md");

    let mut updated_content = String::new();
    for line in readme_content.lines() {
        if line.starts_with("- just so you know,") {
            let updated_line = format!(
                "- just so you know, {} is {:.2}% complete",
                current_year,
                progress_of_this_year * 100.0
            );
            updated_content.push_str(&updated_line);
            updated_content.push('\n');
        } else {
            updated_content.push_str(line);
            updated_content.push('\n');
        }
    }

    fs::write(readme_path, updated_content).expect("cannot overwrite README.md");
}
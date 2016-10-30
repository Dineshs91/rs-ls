extern crate colored;
extern crate chrono;

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::time::UNIX_EPOCH;
use colored::*;
use chrono::*;

fn main() {
    let files = fs::read_dir("./").unwrap();
    let files_block = fs::read_dir("./").unwrap();
    let mut file_count = 0;
    let mut file_block_count = 0;

    for f in files_block {
        file_block_count  += f.unwrap().path().metadata().unwrap().blocks();
    }

    println!("total  {}", file_block_count);

    for file in files {
        file_count += 1;
        let file_path = file.unwrap().path();

        let mut file_path_display = file_path.display().to_string();
        let mut file_size = file_path.metadata().unwrap().len().to_string();

        let file_modified = file_path.metadata().unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let file_modified_dt = Local.timestamp(file_modified as i64, 0);

        format_display(& mut file_path_display, 15);
        format_display(& mut file_size, 6);

        println!("{}   {}  {}  {}", file_count.to_string().green(),
           file_path_display.yellow().bold(),
           file_size,
           file_modified_dt.format("%h %d %H:%M")
        )
    }
}

fn format_display(text: & mut String, value: usize) {
    let actual_size = value - text.len();

    for _ in 0..actual_size {
        text.push_str(" ");
    }
}

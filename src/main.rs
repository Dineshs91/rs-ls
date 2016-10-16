extern crate colored;

use std::fs;
use colored::*;

fn main() {
    let files = fs::read_dir("./").unwrap();
    let files_count = fs::read_dir("./").unwrap().count();
    let mut file_count = 0;

    println!("total  {}", files_count);

    for file in files {
        file_count += 1;
        let file_path = file.unwrap().path();

        let mut file_path_display = file_path.display().to_string();
        let mut file_size = file_path.metadata().unwrap().len().to_string();

        let file_permissions = file_path.metadata().unwrap().permissions().readonly();
        let file_modified = file_path.metadata().unwrap().modified().unwrap();

        format_display(& mut file_path_display, 15);
        format_display(& mut file_size, 6);

        println!("{}   {}   {}   {:?}   {:?}", file_count.to_string().green(),
           file_path_display.yellow().bold(),
           file_size,
           file_permissions,
           file_modified
        )
    }
}

fn format_display(text: & mut String, value: usize) {
    let actual_size = value - text.len();

    for _ in 0..actual_size {
        text.push_str(" ");
    }
}

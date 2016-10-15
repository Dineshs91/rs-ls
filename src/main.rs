extern crate colored;

use std::fs;
use colored::*;

fn main() {
    println!("Listing files in a directory");
    let files = fs::read_dir("./").unwrap();
    let mut file_count = 0;
    for file in files {
        file_count += 1;
        let file_path = file.unwrap().path();
        let mut file_path_display = file_path.display().to_string();
        let file_display_len = file_path_display.len();

        // Lets take a value of 20
        let word_space = 20;
        let actual_space = word_space - file_display_len;
        for _ in 0..actual_space {
            file_path_display.push_str(" ");
        }

        println!("{}   {}   {:?}", file_count.to_string().green(),
         file_path_display.yellow().bold(),
         file_path.metadata().unwrap().len()
        )
    }
}

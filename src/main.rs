mod config;
mod errors;

use config::{Config, Mode};
use errors::AppError;

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    let config = Config::try_from(args)?;

    match config.mode {
        Mode::TextSearch => {
            println!(
                "Searching for text '{}' in file '{}'...\n",
                config.pattern, config.target_path
            );

            let file = File::open(&config.target_path)?;
            let reader = BufReader::new(file);
            let mut total_matches = 0;

            reader
                .lines()
                .map_while(Result::ok)
                .enumerate()
                .filter(|(_, line)| line.contains(&config.pattern))
                .for_each(|(line_num, line)| {
                    total_matches += 1;
                    println!("Line {}: {}", line_num + 1, line.trim());
                });

            println!("\nSearch complete! Total matches found: {}", total_matches);
        }
        Mode::FileFind => {
            println!(
                "Searching for files containing '{}' in directory '{}'...\n",
                config.pattern, config.target_path
            );

            let mut found_files = Vec::new();
            search_directory(Path::new(&config.target_path), &config.pattern, &mut found_files)?;

            if found_files.is_empty() {
                println!("No matching files found.");
            } else {
                for (index, file_path) in found_files.iter().enumerate() {
                    println!("{}: {}", index + 1, file_path);
                }
                println!("\nSearch complete! Total files found: {}", found_files.len());
            }
        }
    }

    Ok(())
}

/// Recursive helper function to find files by name
fn search_directory(dir: &Path, pattern: &str, results: &mut Vec<String>) -> Result<(), io::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.to_string_lossy();

            if path.is_file() && path_str.contains(pattern) {
                results.push(path_str.into_owned());
            } else if path.is_dir() {
                let _ = search_directory(&path, pattern, results); // Ignore permission errors on subfolders
            }
        }
    }
    Ok(())
}
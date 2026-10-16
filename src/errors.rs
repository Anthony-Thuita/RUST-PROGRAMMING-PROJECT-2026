use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    MissingArguments,
    IoError(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingArguments => {
                write!(
                    f,
                    "Usage:\n  Text Search: cargo run -- <pattern> <file_path>\n  File Finder: cargo run -- --find <pattern> <directory_path>"
                )
            }
            AppError::IoError(err) => write!(f, "File error: {err}"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}
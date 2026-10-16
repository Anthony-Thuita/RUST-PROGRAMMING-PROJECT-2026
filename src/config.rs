use crate::errors::AppError;

#[derive(Debug)]
pub enum Mode {
    TextSearch,
    FileFind,
}

#[derive(Debug)]
pub struct Config {
    pub mode: Mode,
    pub pattern: String,
    pub target_path: String,
}

impl TryFrom<Vec<String>> for Config {
    type Error = AppError;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
        if args.len() < 3 {
            return Err(AppError::MissingArguments);
        }

        // Check if user passed the --find flag for file name searching
        if args[1] == "--find" {
            if args.len() < 4 {
                return Err(AppError::MissingArguments);
            }
            Ok(Config {
                mode: Mode::FileFind,
                pattern: args[2].clone(),
                target_path: args[3].clone(),
            })
        } else {
            Ok(Config {
                mode: Mode::TextSearch,
                pattern: args[1].clone(),
                target_path: args[2].clone(),
            })
        }
    }
}
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "finddup",
    about = "Find duplicate files in a directory by comparing hashes"
)]

/*
Notes:

clap will infer stuff from the field attributes:
- "short" and "long" arguments from the field name.
- "default_value_t" from the field type.
- "help" from the field doc string.

"default_value" takes a string literal
"default_value_t" takes a value of the same type as the field (safer)
*/
pub struct CliArgs {
    /// The directory to scan for duplicate files
    pub path: PathBuf,

    /// Minimum file size (in bytes) to consider for duplicates
    #[arg(short, long, default_value_t = 1)]
    pub min_size: u64,

    /// Number of threads to use (0 = auto)
    #[arg(short = 't', long, default_value_t = 0)]
    pub threads: usize,

    /// Output results as JSON instead of pretty text
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

impl CliArgs {
    pub fn parse() -> Self {
        <CliArgs as Parser>::parse()
    }
}

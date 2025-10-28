mod cli;
mod models;
mod output;
mod scanner;

use cli::CliArgs;
use output::print_results;
use scanner::find_duplicates;

fn main() {
    let args = CliArgs::parse();
    let results = find_duplicates(&args.path, args.min_size, args.threads);
    print_results(&results, args.json);
}

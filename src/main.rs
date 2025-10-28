mod cli;
mod scanner;
mod models;
mod output;

use cli::CliArgs;
use scanner::find_duplicates;
use output::print_results;

fn main() {
    let args = CliArgs::parse();
    let results = find_duplicates(&args.path, args.min_size, args.threads);
    print_results(&results, args.json);
}

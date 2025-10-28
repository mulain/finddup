mod cli;
mod models;
mod output;
mod scanner;

use cli::CliArgs;
use output::print_results;
use rayon::ThreadPoolBuilder;
use scanner::find_duplicates;

fn main() {
    let args = CliArgs::parse();
    configure_threadpool(args.threads);
    let results = find_duplicates(&args.path, args.min_size);
    print_results(&results, args.json);
}

fn configure_threadpool(threads: usize) {
    if threads > 0 {
        if ThreadPoolBuilder::new()
            .num_threads(threads.max(1))
            .build_global()
            .is_ok()
        {
            println!("→ Using {threads} Rayon threads");
        } else {
            println!(
                "→ Rayon global thread pool already initialized, continuing with existing configuration"
            );
        }
    } else {
        let default_threads = rayon::current_num_threads();
        println!("→ Using default Rayon thread pool ({default_threads} threads)");
    }
}

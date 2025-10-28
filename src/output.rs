use crate::models::DuplicateGroup;
use colored::*;
use serde_json;

pub fn print_results(groups: &[DuplicateGroup], as_json: bool) {
    if as_json {
        let json = serde_json::to_string_pretty(groups).unwrap();
        println!("{}", json);
        return;
    }

    if groups.is_empty() {
        println!("{}", "No duplicates found!".green());
        return;
    }

    println!(
        "{}",
        format!("Found {} duplicate groups:", groups.len()).yellow()
    );

    for (i, g) in groups.iter().enumerate() {
        println!(
            "{}",
            format!(
                "\nGroup {} ({} files, hash: {})",
                i + 1,
                g.files.len(),
                g.hash
            )
            .blue()
            .bold()
        );
        for f in &g.files {
            println!("  {}", f.display());
        }
    }
}

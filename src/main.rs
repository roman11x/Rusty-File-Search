mod search;
mod tui;

use search::SearchConfig;
use search::parallel_search;
use std::path::PathBuf;
use std::sync::Arc;
use colored::Colorize;

fn main() {
    tui::print_header();
    loop {
        match tui::print_prompt() {
            Some((search_term, path)) => {
                let config = Arc::new(SearchConfig::new(search_term.to_string()));
                let path = PathBuf::from(path);
                match parallel_search(&path, config) {
                    Ok(results) => { tui::display_results(&results); }, //user wants to search
                    Err(e) => println!("error: {}", e.to_string().red().bold())
                }
            },
            None => {
                println!("{}", "Goodbye!".green().bold());
                break //user wants to exit
            }
        }
    }
}



mod search;
mod tui;

use search::SearchConfig;
use search::parallel_search;
use std::path::{PathBuf};
use std::sync::Arc;


fn main() {
    tui::print_header();
    let (search_term, path) = tui::print_prompt();
    let config = Arc::new(SearchConfig::new(search_term.to_string()));
    let path = PathBuf::from(path);
    let results = parallel_search(&path, config).expect("failed to search");
    tui::display_results(&results);
}



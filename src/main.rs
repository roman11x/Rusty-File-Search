mod search;
mod tui;

use search::SearchConfig;
use search::parallel_search;
use std::path::{PathBuf};
use std::sync::Arc;


fn main() {
    tui::print_header();
    loop{
        match tui::print_prompt(){
            Some((search_term, path)) => {
                let config = Arc::new(SearchConfig::new(search_term.to_string()));
                let path = PathBuf::from(path);
                let results = parallel_search(&path, config).expect("failed to search");
                tui::display_results(&results); }, //user wants to search
            None => break //user wants to exit
        }

    }

}



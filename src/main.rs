mod search;

use search::SearchConfig;
use search::search;
use std::io::stdin;
use std::path::{PathBuf};
use std::sync::Arc;

fn main() {
    println!("please enter the name of the file you would like to search for");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("failed to read line");
    let search_term = s.trim();
    let config = Arc::new(SearchConfig::new(search_term.to_string()));

    println!("please enter the path to the directory you would like to search");
    let mut s1 = String::new();
    stdin().read_line(&mut s1).expect("failed to read line");
    let path = PathBuf::from(s1.trim());
    let count  = search(&path, config).expect("failed to search");

    if count > 1{
        println!("{} files were found", count);
    }

    else {
        println!("{} file was found", count);
    }

}



use std::path::PathBuf;
use colored::Colorize;
use std::io::stdin;
use std::env::var;

pub fn display_results(results: &Vec<PathBuf>){
    for result in results {
        println!("{}", result.to_string_lossy().green().bold());
    }
}

pub fn print_header(){
    let s = r#"
   .______       __    __       _______.___________.____    ____     _______  __   __       _______         _______. _______     ___      .______        ______  __    __
   |   _  \     |  |  |  |     /       |           |\   \  /   /    |   ____||  | |  |     |   ____|       /       ||   ____|   /   \     |   _  \      /      ||  |  |  |
   |  |_)  |    |  |  |  |    |   (----`---|  |----` \   \/   /     |  |__   |  | |  |     |  |__         |   (----`|  |__     /  ^  \    |  |_)  |    |  ,----'|  |__|  |
   |      /     |  |  |  |     \   \       |  |       \_    _/      |   __|  |  | |  |     |   __|         \   \    |   __|   /  /_\  \   |      /     |  |     |   __   |
   |  |\  \----.|  `--'  | .----)   |      |  |         |  |        |  |     |  | |  `----.|  |____    .----)   |   |  |____ /  _____  \  |  |\  \----.|  `----.|  |  |  |
   | _| `._____| \______/  |_______/       |__|         |__|        |__|     |__| |_______||_______|   |_______/    |_______/__/     \__\ | _| `._____| \______||__|  |__|
  "#;
    println!("{}", s.truecolor(255,140,0).bold());
}
pub fn print_prompt() -> (String, String){
    let mut search_term = String::new();
    let mut path = String::new();
    println!("{}", "please enter the name of the file you would like to search for".red().bold());
    stdin().read_line(&mut search_term).expect("failed to read line");
    println!("{}", "please enter the path to the directory you would like to search, press enter for default home directory".red().bold());
    stdin().read_line(&mut path).expect("failed to read line");
    path = path.trim().to_string();
    if path.is_empty(){
        if cfg!(target_os = "windows") {
            path = var("USERPROFILE").unwrap_or(".".to_string());
        }
        else {
            path = var("HOME").unwrap_or(".".to_string());
        }
    }
    (search_term.trim().to_string(), path)
}

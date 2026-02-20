use std::path::{Path, PathBuf};
use colored::Colorize;
use std::io::stdin;
use std::env::var;
use std::process::Command;
pub fn display_results(results: &Vec<PathBuf>){
    for (i, result) in results.iter().enumerate() {
        println!("{}: {}", i+1, result.to_string_lossy().green().bold());
    }
    print_summary(results);
    if results.is_empty(){
        return;
    }
    else if results.len() == 1{
        println!("{}", " single file found, opening file...".blue().bold());
        open_file(&results[0]);
        return;
    }
    loop{
        println!("{}", "please enter the number of the path you would like to open, enter to skip".blue().bold());
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read line");
        if input.trim().is_empty(){
            break;
        }
        match input.trim().parse::<usize>(){
            Ok(index) if index > 0 && index <= results.len() =>{
                open_file(&results[index-1]);
            }
            _ => println!("{}", "please enter a valid number".red().bold())
        }
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
//A function that prompts the user for a search term and a path
pub fn print_prompt() -> Option<(String, String)>{
    let mut search_term = String::new();
    let mut path = String::new();
    println!("{}", "please enter the name of the file you would like to search for. Exit to exit".red().bold());
    stdin().read_line(&mut search_term).expect("failed to read line");
    if search_term.trim().is_empty(){ 
        loop {
            println!("{}", "please enter a valid search term".red().bold());
            search_term.clear();
            stdin().read_line(&mut search_term).expect("failed to read line");
            if !search_term.trim().is_empty(){
                break;
            }
        }
    }
    if search_term.trim().to_lowercase() == "exit"{
        return None;
    }

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

    Some((search_term.trim().to_string(), path))
}

fn print_summary(results: &Vec<PathBuf>){
    if results.len() > 1{
        println!("{} results found", results.len().to_string().blue().bold());
    }
   else if results.len() == 1{
       println!("{}", "1 result found".blue().bold());
   }
    else{
        println!("{}", "no results found".red().bold());
    }
}

fn open_file(path: &Path){
    let result = if cfg!(target_os = "windows") { //open the containing folder on Windows
        // explorer /select,"C:\path\to\file.txt"
        // Highlights the specific file in Explorer
        Command::new("explorer")
            .arg(format!("/select,\"{}\"", path.display()))
            .spawn()

    } else if cfg!(target_os = "macos") { //open the containing folder on macOS
        // open -R "/path/to/file.txt"
        // Reveals and highlights the file in Finder
        Command::new("open")
            .arg("-R")
            .arg(path).spawn()
    } else {  //user is on Linux, on Linux if you provide the entire path to a file, it will open the file
              //xdg-open /path/to/file.txt
        match path.parent(){
            Some(folder) => Command::new("xdg-open").arg(folder).spawn(),
            None => Err( std::io::Error::new( std::io::ErrorKind::Other, "no parent folder found"))
        }
    };
    if let Err(e) = result {
        println!("error opening file: {}", e);
    }
}

use std::fs;
use std::path::Path;

pub struct SearchConfig {
    pub search_term: String,
}

impl SearchConfig {
    pub fn new(search_term: String) -> Self {
        Self { search_term }
    }
    pub fn matches(&self, file_name: &str) -> bool {
        file_name.to_lowercase().contains(&self.search_term.to_lowercase())
    }
}

pub fn search(path: &Path, config: &SearchConfig) -> Result <usize, std::io::Error>{
    let mut counter = 0;


    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            counter += search(&entry.path(), config)?;
        }
        else if entry.file_type()?.is_file() {
            let file_name_os = entry.file_name();
            let file_name = file_name_os.to_string_lossy();
            if config.matches(&file_name){
                println!("file {} was found at {}", file_name, entry.path().display());
                counter += 1
            }
        }
    }

    Ok(counter)

}
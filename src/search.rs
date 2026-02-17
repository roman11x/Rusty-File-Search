use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::thread;

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

pub fn search(path: &Path, config: Arc<SearchConfig>) -> Result <usize, std::io::Error>{
    let mut counter = 0;
    let mut v_handles  = Vec::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let value = config.clone();
            let handle = thread::spawn(move || search(&entry.path(), value));
            v_handles.push(handle);
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

    for handle in v_handles {
        counter += handle.join().unwrap()?;
    }

    Ok(counter)

}
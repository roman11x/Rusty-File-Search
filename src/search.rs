use std::fs;
use std::path::{Path, PathBuf};
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

pub fn walk(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut v = Vec::new();
    let entries = fs::read_dir(path)?;
    v.push(path.to_path_buf());
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let sub_paths = walk(&entry.path())?;
            v.extend(sub_paths);
        }
    }

    Ok(v)

}

 fn search_dir(path: &Path, config: &SearchConfig) -> Result<usize, std::io::Error> {
    let mut counter = 0;
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_file() {
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

pub fn parallel_search(path: &Path, config: Arc<SearchConfig>) -> Result<usize, std::io::Error> {
    let num_cpus = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let mut counter = 0;
    let paths = walk(path)?;
    let chunks = paths.chunks((paths.len() + num_cpus -1) / num_cpus);
    let mut v_handles  = Vec::new();
    for chunk in chunks {
        let value = chunk.to_vec();
        let config = config.clone();
        let handle = thread::spawn(move || -> Result<usize, std::io::Error>
        {
            let mut sum = 0;
            for path in value {
                sum += search_dir(&path, &config)?;
            }
            Ok(sum)
        });
        v_handles.push(handle);
    }
    for handle in v_handles {
        counter += handle.join().unwrap()?;
    }

    Ok(counter)

}
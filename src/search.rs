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

 fn search_dir(path: &Path, config: &SearchConfig) -> Result<Vec<PathBuf>, std::io::Error> {
    let entries = fs::read_dir(path)?;
    let mut v_entries = Vec::new();
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let file_name_os = entry.file_name();
            let file_name = file_name_os.to_string_lossy();
            if config.matches(&file_name){
                v_entries.push(entry.path());
            }
        }
    }
    Ok(v_entries)
}

pub fn parallel_search(path: &Path, config: Arc<SearchConfig>) -> Result<Vec<PathBuf>, std::io::Error> {
    let num_cpus = thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let mut results = Vec::new();
    let paths = walk(path)?;
    let chunks = paths.chunks((paths.len() + num_cpus -1) / num_cpus);
    let mut v_handles  = Vec::new();
    for chunk in chunks {
        let value = chunk.to_vec();
        let config = config.clone();
        let handle = thread::spawn(move || -> Result<Vec<PathBuf>, std::io::Error>
        {
            let mut result = Vec::new();
            for path in value {
               result.extend(search_dir(&path, &config)?);
            }
            Ok(result)
        });
        v_handles.push(handle);
    }
    for handle in v_handles {
        results.extend(handle.join().unwrap()?);
    }

    Ok(results)

}
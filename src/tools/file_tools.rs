use std::path::PathBuf;

pub struct FileTools {
    pub current_dir: PathBuf,
    pub selected_files: Vec<PathBuf>,
}

impl FileTools {
    pub fn new() -> Self {
        Self {
            current_dir: Self::home_dir(),
            selected_files: Vec::new(),
        }
    }

    fn home_dir() -> PathBuf {
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"))
    }
    
    pub fn change_directory(&mut self, _path: &str) {
        // Belum diimplementasikan
    }

    pub fn bulk_rename(&mut self, _pattern: &str, _replacement: &str) {
        // Belum diimplementasikan
    }

    pub fn select_file(&mut self, filename: &str) {
        let path = self.current_dir.join(filename);
        self.selected_files.push(path);
    }
}

use std::path::PathBuf;

pub struct ReadedFiles{
    name: String,
    path: PathBuf,
    archive_path: Option<PathBuf>,
    readed_data: String
}


impl ReadedFiles {
    pub fn is_in_archive(&self) -> bool{
        self.archive_path.is_some()
    }
}
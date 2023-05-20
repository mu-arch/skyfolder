use std::path::Path;
use tokio::fs;
use crate::lib::errors::AppError;

#[derive(Debug)]
pub struct DirEntry {
    pub(crate) name: String,
    pub(crate) size: Option<u64>,
    pub(crate) is_dir: bool,
    pub(crate) is_symlink: bool,
}

pub async fn list_dir_contents(dir: &Path) -> Result<Vec<DirEntry>, AppError> {
    let mut entries = Vec::new();

    if dir.is_dir() {
        let mut dir_entries = fs::read_dir(dir).await?;

        while let Some(entry) = dir_entries.next_entry().await? {
            let path = entry.path();
            let metadata = fs::metadata(&path).await?;
            entries.push(DirEntry {
                name: String::from(path.file_name().unwrap().to_str().unwrap()),
                size: if metadata.is_file() { Some(metadata.len()) } else { None },
                is_dir: metadata.is_dir(),
                is_symlink: entry.file_type().await?.is_symlink(),
            });
        }
    }

    Ok(entries)
}

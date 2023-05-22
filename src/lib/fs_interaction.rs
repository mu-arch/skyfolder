use std::path::Path;
use tokio::fs;
use crate::lib::errors::{AppErrorExternal};
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug)]
pub struct DirEntry {
    pub(crate) name: String,
    pub(crate) size: Option<u64>,
    pub(crate) is_dir: bool,
    pub(crate) is_symlink: bool,
    pub(crate) last_modified: Option<DateTime<Utc>>, // Change this line
}

pub async fn list_dir_contents(dir: &Path) -> Result<Vec<DirEntry>, AppErrorExternal> {
    let mut entries = Vec::new();

    if dir.is_dir() {
        let mut dir_entries = fs::read_dir(dir).await?;

        while let Some(entry) = dir_entries.next_entry().await? {
            let path = entry.path();
            let metadata = fs::metadata(&path).await?;
            let last_modified = metadata.modified().ok().map(|system_time| DateTime::<Utc>::from(system_time));
            entries.push(DirEntry {
                name: String::from(path.file_name().unwrap().to_str().unwrap()),
                size: if metadata.is_file() { Some(metadata.len()) } else { None },
                is_dir: metadata.is_dir(),
                is_symlink: entry.file_type().await?.is_symlink(),
                last_modified, // Change this line
            });
        }
    }

    Ok(entries)
}

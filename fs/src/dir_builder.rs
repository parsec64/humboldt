use std::path::PathBuf;

use tokio::{fs, io};

use super::fs_node::FsNode;


pub struct DirBuilder;

impl DirBuilder {
    pub async fn from_path(path: PathBuf) -> io::Result<Vec<FsNode>> {
        let mut nodes = Vec::new();

        let mut entries = fs::read_dir(&path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let node = FsNode::from_path(entry.path()).await?;

            nodes.push(node);
        }

        Ok(nodes)
    }
}

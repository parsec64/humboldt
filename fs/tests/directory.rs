use fs::dir_builder::DirBuilder;
use std::path::PathBuf;

#[tokio::test]
async fn list_directory() {
    let path = PathBuf::from("/opt");

    let nodes = DirBuilder::from_path(path).await.unwrap();

    for node in nodes {
        println!("{}\n", node);
    }
}

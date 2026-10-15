use std::path::PathBuf;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::time::SystemTime;
use std::fs::Metadata;
use std::fmt;

use tokio::fs;
use users::{get_user_by_uid, get_group_by_gid};
use chrono::{DateTime, Local};


#[derive(Debug)]
pub struct FsNode {
    pub name: String,
    pub path: PathBuf,
    pub file_type: NodeType,

    pub node_size: u64,
    pub size: Option<u64>,
    pub item_count: Option<usize>,

    pub user: User,
    pub group: Group,
    pub permission: Permission,

    pub created: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
    pub modified: SystemTime,
}

#[derive(Debug)]
pub enum NodeType {
    Directory,
    RegularFile,
    SymLink,
    CharDevice,
    BlockDevice,
    Socket,
    Fifo,
    Unknown,
}

#[derive(Debug)]
pub struct User {
    name: String,
    uid: u32,
}

#[derive(Debug)]
pub struct Group {
    name: String,
    gid: u32,
}

#[derive(Debug)]
pub struct Permission {
    user: PermissionBits,
    group: PermissionBits,
    other: PermissionBits,

    setuid: bool,
    setgid: bool,
    sticky: bool,
}

#[derive(Debug)]
pub struct PermissionBits {
    r: bool,
    w: bool,
    x: bool,
}

impl FsNode {
    pub async fn from_path(path: PathBuf) -> std::io::Result<Self> {

        // Use symlink_metadata so that a symlink is represented
        // as a symlink rather than following it to its target.
        let metadata = fs::symlink_metadata(&path).await?;

        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        let file_type = Self::get_file_type(&metadata);

        let uid = metadata.uid();
        let gid = metadata.gid();

        let permission = Self::get_permissions(metadata.mode());

        let user = User {
            uid,
            name: Self::get_user_name(uid),
        };

        let group = Group {
            gid,
            name: Self::get_group_name(gid),
        };

        Ok(Self {
            name: name,
            path: path,
            file_type,

            node_size: metadata.len(),
            size: None,
            item_count: None,

            user,
            group,
            permission,

            created: metadata.created().ok(),
            accessed: metadata.accessed().ok(),
            modified: metadata.modified()?,
        })
    }

    fn get_file_type(metadata: &Metadata) -> NodeType {
        let file_type = metadata.file_type();

        if file_type.is_dir() {
            NodeType::Directory
        } else if file_type.is_file() {
            NodeType::RegularFile
        } else if file_type.is_symlink() {
            NodeType::SymLink
        } else if file_type.is_char_device() {
            NodeType::CharDevice
        } else if file_type.is_block_device() {
            NodeType::BlockDevice
        } else if file_type.is_fifo() {
            NodeType::Fifo
        } else if file_type.is_socket() {
            NodeType::Socket
        } else {
            NodeType::Unknown
        }
    }

    fn get_permissions(mode: u32) -> Permission {
        // Comparision with a bitwise mask
        Permission {
            user: PermissionBits {
                r: mode & 0o400 != 0,
                w: mode & 0o200 != 0,
                x: mode & 0o100 != 0,
            },

            group: PermissionBits {
                r: mode & 0o040 != 0,
                w: mode & 0o020 != 0,
                x: mode & 0o010 != 0,
            },

            other: PermissionBits {
                r: mode & 0o004 != 0,
                w: mode & 0o002 != 0,
                x: mode & 0o001 != 0,
            },

            setuid: mode & 0o4000 != 0,
            setgid: mode & 0o2000 != 0,
            sticky: mode & 0o1000 != 0,
        }
    }

    fn get_user_name(uid: u32) -> String {
        get_user_by_uid(uid)
            .map(|user| user.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string())
    }

    fn get_group_name(gid: u32) -> String {
        get_group_by_gid(gid)
            .map(|group| group.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| gid.to_string())
    }

    fn _format_time(time: SystemTime) -> String {
        let datetime: DateTime<Local> = time.into();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}



impl fmt::Display for FsNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name:         {}", self.name)?;
        writeln!(f, "Path:         {}", self.path.display())?;
        writeln!(f, "Type:         {:?}", self.file_type)?;
        writeln!(f, "Node size:    {} bytes", self.node_size)?;

        match self.size {
            Some(size) => writeln!(f, "Content size: {} bytes", size)?,
            None => writeln!(f, "Content size: N/A")?,
        }

        match self.item_count {
            Some(count) => writeln!(f, "Item count:   {}", count)?,
            None => writeln!(f, "Item count:   N/A")?,
        }

        writeln!(
            f,
            "Owner:        {} ({})",
            self.user.name, self.user.uid
        )?;

        writeln!(
            f,
            "Group:        {} ({})",
            self.group.name, self.group.gid
        )?;

        writeln!(f, "Permission:   {:?}", self.permission)?;
        writeln!(f, "Created:      {:?}", self.created)?;
        writeln!(f, "Accessed:     {:?}", self.accessed)?;
        writeln!(f, "Modified:     {:?}", self.modified)?;

        Ok(())
    }
}

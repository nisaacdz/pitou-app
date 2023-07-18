use std::{path::PathBuf, time::SystemTime};

use serde::{Deserialize /* Serializer, Deserializer*/, Serialize};

use sysinfo::{DiskExt, DiskKind};

impl From<Drive> for Pitou {
    fn from(drive: Drive) -> Self {
        Self {
            path: drive.mount_point,
        }
    }
}

impl From<&sysinfo::Disk> for Drive {
    fn from(drive: &sysinfo::Disk) -> Self {
        let mount_point = drive.mount_point().into();
        let is_removable = drive.is_removable();
        let total_space = drive.total_space();
        let free_space = drive.available_space();
        let kind = drive.kind().into();
        let name = drive.name().to_owned().into();

        Drive {
            mount_point,
            total_space,
            free_space,
            is_removable,
            kind,
            name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drive {
    pub(crate) mount_point: PathBuf,
    pub(crate) total_space: u64,
    pub(crate) free_space: u64,
    pub(crate) is_removable: bool,
    pub(crate) kind: DriveKind,
    pub(crate) name: PathBuf,
}

impl Drive {
    pub fn name(&self) -> &std::ffi::OsStr {
        &self.name.as_os_str()
    }

    pub fn is_removable(&self) -> bool {
        self.is_removable
    }

    pub fn kind(&self) -> DriveKind {
        self.kind
    }

    pub fn mount_point(&self) -> &PathBuf {
        &self.mount_point
    }

    pub fn free_space(&self) -> u64 {
        self.free_space
    }

    pub fn total_space(&self) -> u64 {
        self.total_space
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DriveKind {
    HDD,
    SSD,
    Unkown(isize),
}

impl From<sysinfo::DiskKind> for DriveKind {
    fn from(kind: sysinfo::DiskKind) -> Self {
        match kind {
            DiskKind::HDD => DriveKind::HDD,
            DiskKind::SSD => DriveKind::SSD,
            DiskKind::Unknown(val) => DriveKind::Unkown(val),
        }
    }
}

#[allow(unused)]
pub struct WithMetadata {
    pitou: Pitou,
    metadata: Metadata,
}

impl WithMetadata {
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Pitou {
    #[serde(with = "serde_path")]
    pub(crate) path: PathBuf,
}

mod serde_path {
    use super::PathBuf;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(path: &PathBuf, sz: S) -> Result<S::Ok, S::Error> {
        let mod_str = path
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
        String::serialize(&mod_str, sz)
    }

    pub fn deserialize<'d, D: Deserializer<'d>>(dz: D) -> Result<PathBuf, D::Error> {
        let de_str = String::deserialize(dz)?;
        let path = de_str.replace("/", std::path::MAIN_SEPARATOR_STR);
        Ok(path.into())
    }
}

impl Pitou {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> Option<&std::ffi::OsStr> {
        self.path().file_name()
    }
}

impl<P: AsRef<std::path::Path>> From<P> for Pitou {
    fn from(path: P) -> Self {
        let path = PathBuf::from(path.as_ref());
        Self { path }
    }
}

impl Into<PathBuf> for Pitou {
    fn into(self) -> PathBuf {
        self.path
    }
}

// backend = { path = "./backend" }
// backend = { path = "../backend" features = ["tauri"] }

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Properties {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub locked: bool,
    pub bookmark: bool,
    pub history: bool,
    pub accessed: Option<SystemTime>,
    pub modified: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub(crate) len: u64,
    pub(crate) size: u64,
    pub(crate) is_dir: bool,
    pub(crate) accessed: Option<SystemTime>,
    pub(crate) modified: Option<SystemTime>,
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn accessed(&self) -> Option<SystemTime> {
        self.accessed
    }

    pub fn modified(&self) -> Option<SystemTime> {
        self.modified
    }
}

#[derive(Clone, Copy)]
pub enum Filter {
    System,
    DotHidden,
    Locked,
}

#[derive(Clone, Copy)]
pub enum Sort {
    Name(bool),
    Modified(bool),
    Accessed(bool),
}

#[cfg(feature = "tauri")]
mod extra;

#[cfg(feature = "tauri")]
pub use extra::*;

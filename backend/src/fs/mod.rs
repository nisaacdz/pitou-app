use std::{ffi::OsString, path::PathBuf, /* ops::Deref,*/ time::SystemTime};

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
        let name = drive.name().to_owned();

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
    pub(crate) name: OsString,
}

impl Drive {
    pub fn name(&self) -> &OsString {
        &self.name
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

pub struct WithMetadata {
    pitou: Pitou,
    metadata: Metadata,
}

impl std::ops::Deref for WithMetadata {
    type Target = Pitou;
    fn deref(&self) -> &Self::Target {
        &self.pitou
    }
}

impl WithMetadata {
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Pitou {
    pub(crate) path: PathBuf,
}

impl Pitou {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> Option<&std::ffi::OsStr> {
        self.path().file_name()
    }
}

impl From<PathBuf> for Pitou {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

// backend = { path = "./backend" }
// backend = { path = "../backend" features = ["tauri"] }

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Properties {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub hidden: bool,
    pub favorite: bool,
    pub recent: bool,
    pub frequent: bool,
    pub accessed: Option<SystemTime>,
    pub modified: Option<SystemTime>,
}

/*

#[derive(Clone)]
pub struct Path {
    pub(crate) inner: std::rc::Rc<PathBuf>,
}

impl Deref for Path {
    type Target = PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        &self.inner
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        &*self.inner == &*other.inner
    }
}


impl Serialize for Path {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        PathBuf::serialize(&self.inner, sz)
    }
}

impl<'d> Deserialize<'d> for Path {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let path = PathBuf::deserialize(dz)?;
        let inner = std::rc::Rc::new(path);
        Ok(Self { inner })
    }
}


*/

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
    Hidden,
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
use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use chrono::Local;
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

    pub fn name(&self) -> String {
        if self.path().as_os_str().len() == 0 {
            String::from("Drives")
        } else {
            match self.path().file_name().unwrap().to_str() {
                Some(v) => v.to_string(),
                None => String::new(),
            }
        }
    }
}

impl From<PathBuf> for Pitou {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

impl From<&Path> for Pitou {
    fn from(value: &Path) -> Self {
        Self {
            path: PathBuf::from(value),
        }
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
    pub metadata: Metadata,
    pub locked: bool,
    pub bookmark: bool,
    pub history: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct DateTime {
    dt: SystemTime,
    cur_dt: SystemTime,
}

impl DateTime {
    pub fn format(&self) -> String {
        use chrono::Datelike;
        let ellapsed = self.cur_dt.duration_since(self.dt).unwrap_or_default();

        let cur_dt = chrono::DateTime::<Local>::from(self.cur_dt);
        let dt = chrono::DateTime::<Local>::from(self.dt);

        if ellapsed < Duration::from_secs(3600) {
            // use chrono::NaiveDate;
            // let dt = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap();
            // assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2015-09-05 23:56:04");
            // assert_eq!(dt.format("around %l %p on %b %-d").to_string(), "around 11 PM on Sep 5");
            // The resulting DelayedFormat can be formatted directly via the Display trait.

            // assert_eq!(format!("{}", dt.format("%Y-%m-%d %H:%M:%S")), "2015-09-05 23:56:04");
            // assert_eq!(format!("{}", dt.format("around %l %p on %b %-d")), "around 11 PM on Sep 5");
            if ellapsed < Duration::from_secs(60) {
                format!("few seconds ago")
            } else if ellapsed < Duration::from_secs(120) {
                format!("1 minute ago")
            } else {
                format!("{} minutes ago", ellapsed.as_secs() / 60)
            }
        } else if dt.day() == cur_dt.day() {
            dt.format("%l:%M %p today").to_string()
        } else if dt.year() == cur_dt.year() {
            dt.format("%l %p on %b %-d").to_string()
        } else {
            dt.format("%l %p on %Y-%m-%d").to_string()
            //chrono.format("%Y-%m-%d %H:%M:%S").to_string()
            //String::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PitouType {
    Directory,
    File,
    Link,
}

impl PitouType {
    pub fn is_dir(self) -> bool {
        matches!(self, PitouType::Directory)
    }

    pub fn is_file(self) -> bool {
        matches!(self, PitouType::File)
    }

    pub fn is_link(self) -> bool {
        matches!(self, PitouType::Link)
    }
}

impl std::fmt::Display for PitouType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            PitouType::Directory => "Directory",
            PitouType::File => "File",
            PitouType::Link => "Link",
        };
        write!(f, "{}", val)
    }
}

impl From<std::fs::FileType> for PitouType {
    fn from(value: std::fs::FileType) -> Self {
        if value.is_symlink() {
            PitouType::Link
        } else if value.is_file() {
            PitouType::File
        } else if value.is_dir() {
            PitouType::Directory
        } else {
            unreachable!()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Metadata {
    pub(crate) len: u64,
    pub(crate) accessed: Option<DateTime>,
    pub(crate) modified: Option<DateTime>,
    pub(crate) filetype: PitouType,
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_dir(&self) -> bool {
        self.filetype.is_dir()
    }

    pub fn accessed(&self) -> Option<DateTime> {
        self.accessed
    }

    pub fn modified(&self) -> Option<DateTime> {
        self.modified
    }

    pub fn file_type(&self) -> PitouType {
        self.filetype
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

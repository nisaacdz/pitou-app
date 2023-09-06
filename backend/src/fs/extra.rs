use super::Drive;
use crate::{DateTime, DriveKind, File, Inner, Locals, Metadata, Properties};
use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use sysinfo::{System, SystemExt};
use tokio::fs;

#[test]
fn test_m_p() {
    let mut mp = System::new();
    mp.refresh_disks_list();

    for disk in mp.disks() {
        let drive: Drive = disk.into();
        println!("{:?}", drive);
    }
}

pub fn drives() -> Vec<Drive> {
    let mut sys = System::new();
    sys.refresh_disks_list();

    let mut res = Vec::new();
    for disk in sys.disks() {
        res.push(disk.into())
    }
    res
}

#[test]
fn test_osstr() {
    let word = std::path::PathBuf::from("Hello, World");
    let word = word.file_name().unwrap();
    let mut _word = word.to_str().unwrap();
    todo!()
}

pub fn downloads_folder() -> PathBuf {
    dirs::download_dir().unwrap()
}

pub fn desktop_folder() -> PathBuf {
    dirs::desktop_dir().unwrap()
}

pub fn videos_folder() -> PathBuf {
    dirs::video_dir().unwrap()
}

pub fn pictures_folder() -> PathBuf {
    dirs::picture_dir().unwrap()
}

pub fn audios_folder() -> PathBuf {
    dirs::audio_dir().unwrap()
}

pub fn documents_folder() -> PathBuf {
    dirs::document_dir().unwrap()
}

pub(crate) trait Get {
    fn get(self) -> std::io::Result<File>;
}

impl Get for PathBuf {
    fn get(self) -> std::io::Result<File> {
        std::fs::metadata(&self).map(|m| {
            let metadata = m.into();
            File::new(self, metadata)
        })
    }
}

impl Get for Drive {
    fn get(self) -> std::io::Result<File> {
        std::fs::metadata(&self.mount_point).map(|m| {
            let metadata = m.into();
            File::new(self.mount_point, metadata)
        })
    }
}

// /// It is safe to because the use of File is carefully monitored so that it
// /// is never cloned in a separate thread will it still exists in one thread
// unsafe impl Send for File {}

impl File {
    pub fn locals() -> Locals {
        Locals {
            downloads: downloads_folder()
                .get()
                .expect("couldn't parse downloads folder"),
            videos: videos_folder().get().expect("couldnt parse videos folder"),
            pictures: pictures_folder()
                .get()
                .expect("couldn't parse pictures folder"),
            audios: audios_folder().get().expect("couldn't parse audios folder"),
            documents: documents_folder()
                .get()
                .expect("couldn't parse documents folder"),
            desktop: desktop_folder()
                .get()
                .expect("couldn't parse desktop folder"),
        }
    }

    pub fn home_directory() -> PathBuf {
        dirs::home_dir().unwrap().into()
    }

    pub async fn try_exists(&self) -> io::Result<bool> {
        fs::try_exists(self.path()).await
    }

    pub async fn children_dirs(dir: &PathBuf) -> io::Result<Vec<Self>> {
        if dir.as_os_str().len() == 0 {
            return Ok(drives()
                .into_iter()
                .map(|drive| drive.get().expect("couldn't turn drive to File"))
                .collect());
        }

        let mut read_dir = fs::read_dir(dir).await?;
        let mut res: Vec<Self> = Vec::new();
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            let metadata: Metadata = entry
                .metadata()
                .await
                .expect("couldn't get metadata")
                .into();

            if metadata.is_dir() {
                let inner = Arc::new(Inner { path, metadata });
                res.push(File { inner });
            }
        }
        Ok(res)
    }

    pub async fn children(dir: &Path) -> io::Result<Vec<Self>> {
        if dir.as_os_str().len() == 0 {
            return Ok(drives()
                .into_iter()
                .map(|drive| drive.get().expect("drive failed parse"))
                .collect());
        }

        let mut read_dir = fs::read_dir(dir).await?;
        let mut res = Vec::new();
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            let metadata: Metadata = entry
                .metadata()
                .await
                .expect("couldn't get metadata")
                .into();
            let inner = Arc::new(Inner { path, metadata });
            res.push(File { inner })
        }
        Ok(res)
    }

    pub async fn siblings(dir: &PathBuf) -> io::Result<Vec<Self>> {
        if let Some(v) = dir.parent() {
            Self::children(v).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn properties(_dir: PathBuf) -> io::Result<Properties> {
        todo!()
    }
}

#[async_recursion::async_recursion]
pub async fn directory_size<P: AsRef<std::path::Path> + Send>(path: P) -> std::io::Result<u64> {
    let mut total_size = 0;

    let mut entries = tokio::fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();

        let metadata = tokio::fs::metadata(&entry_path).await?;
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += directory_size(&entry_path).await.unwrap_or(0);
        }
    }

    Ok(total_size)
}

impl From<std::fs::Metadata> for Metadata {
    fn from(metadata: std::fs::Metadata) -> Self {
        let modified: Option<crate::DateTime> = metadata.modified().ok().map(Into::into);
        let len = metadata.len();
        let accessed = metadata.accessed().map(Into::into).ok();
        let filetype = metadata.file_type().into();
        Self {
            modified,
            len,
            accessed,
            filetype,
        }
    }
}

impl From<SystemTime> for DateTime {
    fn from(dt: SystemTime) -> Self {
        DateTime {
            dt,
            cur_dt: SystemTime::now(),
        }
    }
}

use sysinfo::{DiskExt, DiskKind};
impl From<&sysinfo::Disk> for Drive {
    fn from(drive: &sysinfo::Disk) -> Self {
        let mount_point = drive.mount_point().into();
        let is_removable = drive.is_removable();
        let total_space = drive.total_space();
        let free_space = drive.available_space();
        let kind = drive.kind().into();
        let name = drive.name().to_str().unwrap().to_owned();

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

impl From<sysinfo::DiskKind> for DriveKind {
    fn from(kind: sysinfo::DiskKind) -> Self {
        match kind {
            DiskKind::HDD => DriveKind::HDD,
            DiskKind::SSD => DriveKind::SSD,
            DiskKind::Unknown(_) => DriveKind::Unkown,
        }
    }
}

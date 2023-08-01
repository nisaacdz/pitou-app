use super::Drive;
use crate::{DateTime, Filter, Metadata, Pitou, Properties, Sort};
use std::{io, path::PathBuf, time::SystemTime};
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

macro_rules! include {
    ($val:expr) => {
        match $val {
            Some(val) => match val {
                Filter::System => unimplemented!(),
                Filter::DotHidden => |pitou: &Pitou| {
                    pitou
                        .path()
                        .file_name()
                        .map(|f| !f.to_str().unwrap().starts_with("."))
                        .unwrap_or(true)
                },
                Filter::Locked => unimplemented!(),
            },
            None => |_: &Pitou| true,
        }
    };
}

macro_rules! sort {
    ($val:expr) => {
        match $val {
            Sort::Name(asc) => move |f1: &Pitou, f2: &Pitou| {
                if asc {
                    f1.name().cmp(&f2.name())
                } else {
                    f2.name().cmp(&f1.name())
                }
            },
            Sort::Modified(_) => unimplemented!(),
            Sort::Accessed(_) => unimplemented!(),
        }
    };
}

impl Pitou {
    pub async fn try_exists(&self) -> io::Result<bool> {
        fs::try_exists(self.path()).await
    }

    pub async fn metadata(&self) -> io::Result<Metadata> {
        let metadata = fs::metadata(self.path()).await?;
        Ok(metadata.into())
    }

    pub async fn refresh(&self) -> io::Result<Metadata> {
        self.metadata().await
    }

    pub async fn children_filtered_and_sorted(
        &self,
        filter: Filter,
        sort: Sort,
    ) -> io::Result<Vec<Pitou>> {
        let mut res = self.children_filtered(filter).await?;
        res.sort_unstable_by(|a, b| sort!(sort)(a, b));
        Ok(res)
    }

    pub async fn children_filtered(&self, filter: Filter) -> io::Result<Vec<Pitou>> {
        self.children().await.map(|c| {
            c.into_iter()
                .filter(|v| include!(Some(filter))(v))
                .collect()
        })
    }

    pub async fn children(&self) -> io::Result<Vec<Pitou>> {
        if self.path().as_os_str().len() == 0 {
            return Ok(drives().into_iter().map(|drive| drive.into()).collect());
        }

        let mut read_dir = fs::read_dir(self.path()).await?;
        let mut res: Vec<Pitou> = Vec::new();
        while let Some(entry) = read_dir.next_entry().await? {
            res.push(entry.path().into())
        }
        Ok(res)
    }

    pub async fn siblings(&self) -> io::Result<Vec<Pitou>> {
        let path = match self.path().parent() {
            Some(v) => v,
            None => return Ok(Vec::new()),
        };

        let pitou: Pitou = PathBuf::from(path).into();
        pitou.children().await
    }

    pub async fn properties(&self) -> io::Result<Properties> {
        let path = self.path().clone();
        let metadata = self.metadata().await?;
        let locked = true;
        let bookmark = true;
        let history = true;

        Ok(Properties {
            path,
            locked,
            bookmark,
            history,
            metadata,
        })
    }

}

pub async fn debug_with_real_dir() -> Pitou {
    let pitou = PathBuf::from("d:/Workspace/rust");
    pitou.into()
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

impl AsRef<std::path::Path> for Pitou {
    fn as_ref(&self) -> &std::path::Path {
        &self.path()
    }
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

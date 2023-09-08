use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime},
};

use chrono::Local;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Drive {
    #[serde(with = "serde_path")]
    pub(crate) mount_point: PathBuf,
    pub(crate) total_space: u64,
    pub(crate) free_space: u64,
    pub(crate) is_removable: bool,
    pub(crate) kind: DriveKind,
    pub(crate) name: String,
}

impl Drive {
    pub fn name(&self) -> &str {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DriveKind {
    HDD,
    SSD,
    Unkown,
}

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct Path {
    inner: PathBuf,
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        &self.inner
    }
}

impl From<PathBuf> for Path {
    fn from(inner: PathBuf) -> Self {
        Self { inner }
    }
}

impl Path {
    pub fn into_inner(self) -> PathBuf {
        self.inner
    }
}

#[cfg(debug_assertions)]
mod dbg_impl {
    use super::Path;
    impl Path {
        pub fn display(&self) -> String {
            self.inner.display().to_string()
        }
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        // This is completely safe because the returned reference is only valid for as long as the lifetime on self
        // This is equivalent to having two immutable pointers to the same memory location
        unsafe { &*(self as *const PathBuf as *const Path) }
    }
}

#[test]
fn test_as_ref_of_path_for_path_buf() {
    let pb = PathBuf::from("d:/workspace/apps");
    println!("original, {}", pb.display());
    let path: &Path = pb.as_ref();
    println!("calling a function on inner PathBuf: {}", path.display());
    println!("original as ref of Path {}", path.inner.display());
}

impl Serialize for Path {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        serde_path::serialize(&self.inner, sz)
    }
}

impl<'d> Deserialize<'d> for Path {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let path = serde_path::deserialize(dz)?;
        Ok(path.into())
    }
}

#[derive(Clone)]
pub struct File {
    inner: Arc<Inner>,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}

impl Eq for File {}

impl Serialize for File {
    fn serialize<S: Serializer>(&self, sz: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(sz)
    }
}

impl<'d> Deserialize<'d> for File {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> Result<Self, D::Error> {
        let inner = Inner::deserialize(dz)?;
        Ok(Self {
            inner: Arc::new(inner),
        })
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Inner {
    #[serde(with = "serde_path")]
    pub(crate) path: PathBuf,
    pub(crate) metadata: Metadata,
}

use std::hash::{Hash, Hasher};
impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.path().hash(state)
    }
}

impl Inner {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.path.display())
    }
}

mod serde_path {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::path::{self, PathBuf};

    pub fn serialize<S: Serializer>(path: &PathBuf, sz: S) -> Result<S::Ok, S::Error> {
        let mod_str = path.to_string_lossy().replace(path::MAIN_SEPARATOR, "/");
        String::serialize(&mod_str, sz)
    }

    pub fn deserialize<'d, D: Deserializer<'d>>(dz: D) -> Result<PathBuf, D::Error> {
        let de_str = String::deserialize(dz)?;
        let mut path = de_str.replace("/", path::MAIN_SEPARATOR_STR);
        if path.len() == 2 && path.ends_with(":") {
            path.push_str(path::MAIN_SEPARATOR_STR);
        }
        Ok(path.into())
    }
}

impl File {
    pub fn new(path: PathBuf, metadata: Metadata) -> Self {
        let inner = Arc::new(Inner { path, metadata });

        Self { inner }
    }

    pub fn metadata(&self) -> &Metadata {
        self.inner.metadata()
    }

    pub fn path(&self) -> &PathBuf {
        &self.inner.path()
    }

    pub fn name(&self) -> &str {
        if self.path().as_os_str().len() == 0 {
            "Drives"
        } else {
            self.path()
                .file_name()
                .map(|v| v.to_str().map(|v| v).unwrap_or_default())
                .unwrap_or_default()
        }
    }

    pub fn name_of(path: &PathBuf) -> &str {
        if path.as_os_str().len() == 0 {
            "Drives"
        } else {
            path.file_name()
                .map(|v| v.to_str().map(|v| v).unwrap_or_default())
                .unwrap_or_default()
        }
    }

    pub fn ancestors(&self) -> impl Iterator<Item = &std::path::Path> {
        self.inner.path().ancestors()
    }
}

// impl Into<PathBuf> for Pitou {
//     fn into(self) -> PathBuf {
//         self.path
//     }
// }

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

impl Properties {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
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

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
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

impl std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f, "{:?}", self.filetype}
    }
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_dir(&self) -> bool {
        self.filetype.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.filetype.is_file()
    }

    pub fn is_link(&self) -> bool {
        self.filetype.is_link()
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

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Locals {
    pub downloads: File,
    pub audios: File,
    pub documents: File,
    pub videos: File,
    pub desktop: File,
    pub pictures: File,
}

mod error;
pub use error::*;
#[cfg(feature = "tauri")]
mod extra;

#[cfg(feature = "tauri")]
pub use extra::*;

use crate::Filter;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub enum SearchArea {
    StartsWith,
    EndsWith,
    Contains,
}

impl std::fmt::Display for SearchArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::StartsWith => String::from("Match Beginning"),
            Self::EndsWith => String::from("Match Ending"),
            Self::Contains => String::from("Match Anywhere"),
        };

        write! {f, "{}", res}
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub enum KeyType {
    Regex,
    RawSearch(SearchArea),
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            KeyType::Regex => String::from("Regex"),
            KeyType::RawSearch(_) => String::from("Standard"),
        };

        write!(f, "{}", res)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FindType {
    File,
    Folder,
    SymLink,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchOptions {
    /// false dir value implies current dir only (search in current dir only)
    pub depth: i8,
    /// regex or not
    pub keytype: KeyType,
    /// true means skip error files while false means quit when error is encountered
    pub skip_errors: bool,
    /// true means case sensitive
    pub case_sensitive: bool,
    ///
    pub filter: Filter,
}

impl SearchOptions {
    #[inline]
    pub fn new() -> Self {
        SearchOptions {
            depth: 1,
            keytype: KeyType::RawSearch(SearchArea::StartsWith),
            skip_errors: true,
            case_sensitive: true,
            filter: Filter::DEFAULT,
        }
    }

    #[inline]
    pub fn current_dir_only(mut self) -> Self {
        self.depth = 1;
        self
    }

    #[inline]
    pub fn is_standard_type(self) -> bool {
        matches!(self.keytype, KeyType::RawSearch(_))
    }

    #[inline]
    pub fn match_beginning(mut self) -> Self {
        self.keytype = KeyType::RawSearch(SearchArea::StartsWith);
        self
    }

    #[inline]
    pub fn match_ending(mut self) -> Self {
        self.keytype = KeyType::RawSearch(SearchArea::EndsWith);
        self
    }

    #[inline]
    pub fn match_anywhere(mut self) -> Self {
        self.keytype = KeyType::RawSearch(SearchArea::Contains);
        self
    }

    #[inline]
    pub fn match_regex(mut self) -> Self {
        self.keytype = KeyType::Regex;
        self
    }

    #[inline]
    pub fn skip_errored_files(mut self) -> Self {
        self.skip_errors = true;
        self
    }

    #[inline]
    pub fn quit_at_error(mut self) -> Self {
        self.skip_errors = false;
        self
    }

    #[inline]
    pub fn depth(mut self, depth: i8) -> Self {
        self.depth = depth;
        self
    }

    #[inline]
    pub fn regex(mut self) -> Self {
        self.keytype = KeyType::Regex;
        self
    }

    #[inline]
    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }

    #[inline]
    pub fn toggle_case_sensitive(mut self) -> Self {
        self.case_sensitive = !self.case_sensitive;
        self
    }

    #[inline]
    pub fn toggle_include_dirs(mut self) -> Self {
        self.filter.dir = !self.filter.dir;
        self
    }

    #[inline]
    pub fn toggle_include_files(mut self) -> Self {
        self.filter.file = !self.filter.file;
        self
    }

    #[inline]
    pub fn toggle_include_links(mut self) -> Self {
        self.filter.link = !self.filter.link;
        self
    }

    #[inline]
    pub fn case_insensitive(mut self) -> Self {
        self.case_sensitive = false;
        self
    }

    #[inline]
    pub fn include_files(mut self) -> Self {
        self.filter.file = true;
        self
    }

    #[inline]
    pub fn include_dirs(mut self) -> Self {
        self.filter.dir = true;
        self
    }

    #[inline]
    pub fn include_links(mut self) -> Self {
        self.filter.link = true;
        self
    }

    #[inline]
    pub fn key_type(mut self, keytype: KeyType) -> Self {
        self.keytype = keytype;
        self
    }
}

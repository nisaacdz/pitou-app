use std::path::{self, PathBuf, Path};

use super::Theme;

pub struct Ancestors<'a, F> {
    iter: std::path::Ancestors<'a>,
    map: F,
}

impl<'a, F> Ancestors<'a, F> {
    fn new(pitou: &'a PitouFile, map: F) -> Self where F: Fn(&Path) -> PitouFile {
        let mut iter = pitou.path().ancestors();
        iter.next().unwrap();
        Self { iter, map }
    }
}

impl<'a, F: Fn(&Path) -> PitouFile> Iterator for Ancestors<'a, F> {
    type Item = PitouFile;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&self.map)
    }
}

#[derive(Clone)]
pub struct PitouFile {
    path: PathBuf,
}

impl PartialEq for PitouFile {
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}

#[allow(unused)]
impl PitouFile {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn generate() -> Self {
        let path =
            path::PathBuf::from("home/user/documents/some random filename");
        Self { path }
    }

    pub fn path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn is_dir(&self) -> bool {
        false
    }

    pub fn ancestors(&self) -> Ancestors<impl Fn(&Path) -> PitouFile> {
        Ancestors::new(self, |path| PitouFile::from(PathBuf::from(path)))
    }

    pub fn parent(&self) -> Option<PitouFile> {
        None
    }

    pub fn name(&self) -> &std::ffi::OsStr {
        self.path().file_name().unwrap()
    }

    pub fn size(&self) -> u64 {
        448
    }
}

#[derive(yew::Properties, Clone, PartialEq)]
pub struct PitouProps {
    pub pitou: Props,
}

impl PitouProps {
    pub fn pitou(&self) -> &Props {
        &self.pitou
    }

    pub fn entries(&self) -> Vec<Props> {
        (0..5).map(|_| self.pitou().clone()).collect()
    }

    pub fn debug() -> Self {
        let file = PitouFile::generate();
        let theme = Theme::DEFAULT;

        let pitou = Props::from(file, theme);

        Self { pitou }
    }

    pub fn pitou_file(&self) -> &PitouFile {
        self.pitou().file()
    }

    pub fn theme(&self) -> &Theme {
        self.pitou().theme()
    }
}

impl From<(PitouFile, &Theme)> for Props {
    fn from((file, theme): (PitouFile, &Theme)) -> Self {
        Self::from(file, *theme)
    }
}

#[derive(Clone, PartialEq)]
pub struct Props {
    inner: std::rc::Rc<Inner>,
}

impl std::fmt::Debug for Props {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path().display())
    }
}

#[allow(dead_code)]
impl Props {
    fn from(file: PitouFile, theme: Theme) -> Self {
        let inner = std::rc::Rc::new(Inner { file, theme });
        Self { inner }
    }

    pub fn file(&self) -> &PitouFile {
        self.inner.file()
    }

    pub fn path(&self) -> &PathBuf {
        self.inner.file().path()
    }

    pub fn theme(&self) -> &Theme {
        self.inner.theme()
    }

    pub fn file_name(&self) -> &std::ffi::OsStr {
        self.path().file_name().unwrap_or_default()
    }
}

#[derive(PartialEq)]
pub struct Inner {
    file: PitouFile,
    theme: Theme,
}

impl Inner {
    pub fn file(&self) -> &PitouFile {
        &self.file
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

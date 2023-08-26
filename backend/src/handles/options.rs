use serde::{Deserialize, Serialize};

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
    pub include_files: bool,
    ///
    pub include_dirs: bool,
    ///
    pub include_links: bool,
    // skip_hidden: bool,
    // ///TODO
    // skip_system_files: bool,
}

impl SearchOptions {
    #[inline]
    pub fn new() -> Self {
        SearchOptions {
            depth: 1,
            keytype: KeyType::RawSearch(SearchArea::StartsWith),
            skip_errors: true,
            case_sensitive: true,
            include_files: true,
            include_dirs: true,
            include_links: true,
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
        self.include_dirs = !self.include_dirs;
        self
    }

    #[inline]
    pub fn toggle_include_files(mut self) -> Self {
        self.include_files = !self.include_files;
        self
    }

    #[inline]
    pub fn toggle_include_links(mut self) -> Self {
        self.include_links = !self.include_links;
        self
    }

    #[inline]
    pub fn case_insensitive(mut self) -> Self {
        self.case_sensitive = false;
        self
    }

    #[inline]
    pub fn include_files(mut self) -> Self {
        self.include_files = true;
        self
    }

    #[inline]
    pub fn include_dirs(mut self) -> Self {
        self.include_dirs = true;
        self
    }

    #[inline]
    pub fn include_links(mut self) -> Self {
        self.include_links = true;
        self
    }

    #[inline]
    pub fn key_type(mut self, keytype: KeyType) -> Self {
        self.keytype = keytype;
        self
    }
}

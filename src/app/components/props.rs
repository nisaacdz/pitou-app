use super::Properties;
use crate::app::{PitouProps, Theme};

#[derive(PartialEq, Properties)]
pub struct ThemeProp {
    pub theme: Theme,
}

impl<'a> From<&'a PitouProps> for ThemeProp {
    fn from(value: &'a PitouProps) -> ThemeProp {
        let theme = value.theme();
        ThemeProp { theme }
    }
}

impl ThemeProp {
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

use backend::Pitou;
use yew::Properties;

use super::Theme;

#[derive(Properties, Clone, PartialEq)]
pub struct PitouProps {
    pub pitou: Pitou,
    pub theme: Theme,
}

impl PitouProps {
    pub fn new(pitou: Pitou, theme: Theme) -> Self {
        Self { pitou, theme }
    }

    pub fn pitou(&self) -> &Pitou {
        &self.pitou
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }
}

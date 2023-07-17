#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    background1: Color,
    background2: Color,
    foreground1: Color,
    spare: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(u8, u8, u8);

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({},{},{})", self.0, self.1, self.2)
    }
}

#[allow(dead_code)]
impl Theme {
    pub const DEFAULT: Theme = Theme {
        background1: Color(50, 50, 50),
        foreground1: Color(255, 255, 255),
        background2: Color(80,85, 90),
        spare: Color(30, 30, 30),
    };

    pub fn get_or_default() -> Self {
        Self::DEFAULT
    }

    pub fn background1(&self) -> Color {
        self.background1
    }

    pub fn background2(&self) -> Color {
        self.background2
    }

    pub fn foreground1(&self) -> Color {
        self.foreground1
    }

    pub fn spare(&self) -> Color {
        self.spare
    }
}

#[allow(unused)]
pub struct Settings {
    view: AppView
}

impl Default for Settings {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[allow(dead_code)]
impl Settings {
    pub const DEFAULT: Settings = Settings {
        view: AppView::Content,
    };

    pub fn settings_or_default() -> Self {
        Self::DEFAULT
    }
    pub fn view(&self) -> AppView {
        self.view
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum AppView {
    Content,
    Opening,
    Settings,
}

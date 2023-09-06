use backend::Filter;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    background1: Color,
    background2: Color,
    foreground1: Color,
    spare: Color,
}

pub struct Width {
    pub value: i32,
}

impl From<i32> for Width {
    fn from(value: i32) -> Self {
        Width { value }
    }
}

pub struct Height {
    pub value: i32,
}

impl From<i32> for Height {
    fn from(value: i32) -> Self {
        Height { value }
    }
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {}px;", self.value)
    }
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "height: {}px;", self.value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    #[allow(unused)]
    pub fn width(self) -> Width {
        Width { value: self.width }
    }

    pub fn height(self) -> Height {
        Height { value: self.height }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {}px;\nheight: {}px;", self.width, self.height)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sizes {
    pub(crate) screen_width: i32,
    pub(crate) screen_height: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ApplicationContext {
    pub(crate) theme: Theme,
    pub(crate) sizes: Sizes,
    pub(crate) settings: Settings,
}

impl Sizes {
    // leftbar width and height
    pub fn screen(self) -> Rectangle {
        let width = self.screen_width;
        let height = self.screen_height;

        Rectangle { width, height }
    }

    pub fn middle_portion(self) -> Rectangle {
        let width = self.screen_width;
        let height = (87 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn menubar(self) -> Rectangle {
        let width = self.screen_width / 25;
        let height = (87 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn menu_item_icon(self) -> Rectangle {
        let width = 3 * self.screen_width / 100;
        let height = 6 * self.screen_height / 100;

        Rectangle { width, height }
    }

    pub fn menu_item(self) -> Rectangle {
        let width = self.screen_width / 25;
        let height = self.screen_height / 12;

        Rectangle { width, height }
    }

    pub fn toolbar(self) -> Rectangle {
        let width = self.screen_width;
        let height = (8 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn pane(self) -> Rectangle {
        let width = (24 * self.screen_width) / 25;
        let height = (87 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn choosedir(self) -> Width {
        let width = (3 * self.screen_width) / 200;

        width.into()
    }

    pub fn diskmeter(self) -> Width {
        let width = (3 * self.screen_width) / 25;
        width.into()
    }

    pub fn diskcmp(self) -> Rectangle {
        let width = (4 * self.screen_width) / 25;
        let height = (2 * self.screen_height) / 20;

        Rectangle { width, height }
    }

    pub fn home_local(self) -> Rectangle {
        let width = (4 * self.screen_width) / 25;
        let height = (2 * self.screen_height) / 18;

        Rectangle { width, height }
    }

    pub fn split_pane(self) -> Rectangle {
        let width = (24 * self.screen_width) / 12;
        let height = (83 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn mainpane(self) -> Rectangle {
        let width = (20 * self.screen_width) / 25;
        let height = (83 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn sidepane_possible_button(self) -> Rectangle {
        let width = (2 * self.screen_width) / 25;
        let height = (4 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn sidepane_icon(self) -> Width {
        let width = (3 * self.screen_width) / 125;
        width.into()
    }

    pub fn sidepane_filename(self) -> Width {
        let width = (15 * self.screen_width) / 125;
        width.into()
    }

    pub fn sidepane(self) -> Rectangle {
        let width = (4 * self.screen_width) / 25;
        let height = (83 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn ancestorspane(self) -> Rectangle {
        let width = (24 * self.screen_width) / 25;
        let height = (4 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn ancestorsbar(self) -> Rectangle {
        let width = (24 * self.screen_width) / 25;
        let height = (3 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn bottombar(self) -> Rectangle {
        let width = self.screen_width;
        let height = (5 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn dsc(self) -> Rectangle {
        let height = (3 * self.screen_height) / 100;
        let width = (20 * self.screen_width) / 25;

        Rectangle { width, height }
    }

    pub fn search_input(self) -> Rectangle {
        let height = (4 * self.screen_height) / 100;
        let width = (3 * self.screen_width) / 25;

        Rectangle { width, height }
    }

    pub fn row(self) -> Height {
        let height = (6 * self.screen_height) / 100;

        height.into()
    }

    pub fn row_icon(self) -> Width {
        let width = self.screen_width / 25;
        width.into()
    }

    pub fn row_namefield(self) -> Width {
        let width = (9 * self.screen_width) / 25;
        width.into()
    }

    pub fn row_checkbox(self) -> Width {
        let width = self.screen_width / 25;
        width.into()
    }

    pub fn row_typefield(self) -> Width {
        let width = (4 * self.screen_width) / 25;
        width.into()
    }

    pub fn row_sparefield(self) -> Width {
        let width = (5 * self.screen_width) / 25;
        width.into()
    }

    pub fn toolbar_item(self) -> Rectangle {
        let width = self.screen_width / 25;
        let height = (8 * self.screen_height) / 25;

        Rectangle { width, height }
    }

    pub fn toolbar_icon(self) -> Rectangle {
        let width = self.screen_width / 25;
        let height = (6 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn toolbar_icon_img(self) -> Rectangle {
        let width = self.screen_width / 25;
        let height = (5 * self.screen_height) / 100;

        Rectangle { width, height }
    }

    pub fn toolbar_namefield(self) -> Rectangle {
        let width = self.screen_width / 25;
        let height = (2 * self.screen_height) / 100;

        Rectangle { width, height }
    }
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
        background2: Color(80, 85, 90),
        spare: Color(30, 30, 30),
    };
    // RICHMOND PICKS THIS ONE
    pub const LIGHTDEFAULT: Theme = Theme {
        background1: Color(222, 184, 135),
        foreground1: Color(0, 0, 0),
        background2: Color(245, 245, 220),
        spare: Color(210, 105, 30),
    };

    pub const MAINLIGHTDEFAULT: Theme = Theme {
        background1: Color(245, 245, 245),
        foreground1: Color(0, 0, 0),
        background2: Color(51, 153, 204),
        spare: Color(255, 255, 255),
    };

    pub const MAINGPT: Theme = Theme {
        //background1: Color(210, 210, 210),
        background1: Color(180, 180, 180),
        foreground1: Color(41, 41, 41),
        background2: Color(235, 235, 255),
        spare: Color(135, 206, 235),
    };

    pub const MAINGPTDARK: Theme = Theme {
        background1: Color(41, 41, 41),
        foreground1: Color(255, 255, 255),
        background2: Color(28, 51, 85),
        //spare: Color(173, 216, 230)
        spare: Color(0, 0, 0),
    };

    pub const LIGHTDEFAULT2: Theme = Theme {
        background1: Color(255, 230, 180),
        foreground1: Color(0, 0, 0),
        background2: Color(245, 245, 210),
        spare: Color(210, 105, 30),
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

#[derive(Clone, Copy, PartialEq)]
pub struct Settings {
    pub(crate) view: AppView,
    pub(crate) filter: Filter,
    /// How many times to refresh in 1 minute max = 255
    pub(crate) refresh_rate: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[allow(dead_code)]
impl Settings {
    pub const DEFAULT: Settings = Settings {
        view: AppView::Explorer,
        refresh_rate: 250,
        filter: Filter::DEFAULT,
    };

    pub fn settings_or_default() -> Self {
        Self::DEFAULT
    }
    pub fn view(&self) -> AppView {
        self.view
    }
}

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Explorer,
    Home,
    Settings,
    Search,
    History,
    Bookmarks,
    Locked,
    Cloud,
}

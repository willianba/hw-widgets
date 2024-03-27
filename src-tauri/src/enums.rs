pub enum WindowId {
    Main,
    Settings,
}

impl WindowId {
    pub fn as_str(&self) -> &str {
        match self {
            WindowId::Main => "main",
            WindowId::Settings => "settings",
        }
    }
}

pub enum WindowTitle {
    Settings,
}

impl WindowTitle {
    pub fn as_str(&self) -> &str {
        match self {
            WindowTitle::Settings => "Settings",
        }
    }
}

pub enum MenuItemId {
    Toggle,
    Quit,
    Settings,
}

impl MenuItemId {
    pub fn as_str(&self) -> &str {
        match self {
            MenuItemId::Settings => "settings",
            MenuItemId::Toggle => "toggle",
            MenuItemId::Quit => "quit",
        }
    }
}

pub enum MenuItemTitle {
    Show,
    Hide,
    Quit,
    Settings,
}

impl MenuItemTitle {
    pub fn as_str(&self) -> &str {
        match self {
            MenuItemTitle::Settings => "Settings",
            MenuItemTitle::Show => "Show",
            MenuItemTitle::Hide => "Hide",
            MenuItemTitle::Quit => "Quit",
        }
    }
}

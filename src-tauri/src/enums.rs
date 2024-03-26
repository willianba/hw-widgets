pub enum WindowId {
    Main,
}

impl WindowId {
    pub fn as_str(&self) -> &str {
        match self {
            WindowId::Main => "main",
        }
    }
}
pub enum MenuItemId {
    Toggle,
    Quit,
}

impl MenuItemId {
    pub fn as_str(&self) -> &str {
        match self {
            MenuItemId::Toggle => "toggle",
            MenuItemId::Quit => "quit",
        }
    }
}

pub enum MenuItemTitle {
    Show,
    Hide,
    Quit,
}

impl MenuItemTitle {
    pub fn as_str(&self) -> &str {
        match self {
            MenuItemTitle::Show => "Show",
            MenuItemTitle::Hide => "Hide",
            MenuItemTitle::Quit => "Quit",
        }
    }
}


#[derive(Clone, Debug)]
pub enum Session {
    LoggedIn(String),
    Guest,
}

impl<'a> Default for Session {
    fn default() -> Self {
        Session::Guest
    }
}

impl<'a> Session {
    pub fn new(viewer: Option<String>) -> Self {
        match viewer {
            Some(viewer) => Session::LoggedIn(viewer),
            None => Session::Guest,
        }
    }
    pub fn viewer(&self) -> Option<&String> {
        match self {
            Session::LoggedIn(viewer) => Some(viewer),
            Session::Guest => None,
        }
    }
}
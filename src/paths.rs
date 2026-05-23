use std::fmt;
use std::path::PathBuf;

pub struct OPath(Option<PathBuf>);

impl OPath {
    pub fn new(path: PathBuf) -> Self {
        Self(Some(path))
    }

    pub fn empty() -> Self {
        Self(None)
    }

    pub fn ok(&self) -> bool {
        self.0.is_some()
    }

    pub fn value(&self) -> &PathBuf {
        self.0.as_ref().expect("no path found")
    }
}

impl fmt::Display for OPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(path) => write!(f, "<{}>", path.display()),
            None => write!(f, "<no path found>"),
        }
    }
}

use std::path::PathBuf;

pub trait PathUtil {
    fn appending(self: &Self, name: &str) -> Self;
}

impl PathUtil for PathBuf {
    fn appending(self: &PathBuf, name: &str) -> Self {
        let mut x = self.clone();
        x.push(name);
        x
    }
}


use std::path::Path;
use std::path::PathBuf;

pub trait StringUtil {
    fn appending(self: &Self, part: &str) -> String;
}
impl StringUtil for str {
    fn appending(self: &str, part: &str) -> String {
        let mut s = String::from(self);
        s.push_str(part);
        return s;
    }
}








pub trait PathUtil {
    fn appending(self: &Self, name: &str) -> PathBuf;
}
impl PathUtil for Path {
    fn appending(self: &Path, name: &str) -> PathBuf {
        let mut x = self.to_path_buf();
        x.push(name);
        x
    }
}
// impl PathUtil for PathBuf {
//     fn appending(self: &PathBuf, name: &str) -> PathBuf {
//         let mut x = self.clone();
//         x.push(name);
//         x
//     }
// }


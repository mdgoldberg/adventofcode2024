use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> String {
    std::fs::read_to_string(path).unwrap()
}

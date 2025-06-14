pub fn preview_file(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
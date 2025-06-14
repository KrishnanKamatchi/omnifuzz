pub fn open_file(path: &str) {
    if let Err(e) = opener::open(path) {
        eprintln!("Failed to open file: {}", e);
    }
}

use std::path::PathBuf;

pub struct AppConfig {
    pub search_dir: PathBuf,
    pub search_term: Option<String>,
}

impl AppConfig {
    pub fn new(args: &[String]) -> Self {
        let search_dir = args.get(1).map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap());
        let search_term = args.get(2).cloned();

        AppConfig {
            search_dir,
            search_term,
        }
    }
}

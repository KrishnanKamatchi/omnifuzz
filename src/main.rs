use omnifuzz::config::AppConfig;
use omnifuzz::ui::run_ui;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = AppConfig::new(&args);

    if let Err(e) = run_ui(config) {
        eprintln!("Application error: {}", e);
    }
}
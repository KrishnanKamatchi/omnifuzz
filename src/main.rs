use omnifuzz::config::AppConfig;
use omnifuzz::finder::run_finder;
use omnifuzz::ui::run_ui;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = AppConfig::new(&args);

    if let Err(e) = run_finder(&config).and_then(|results| Ok(run_ui(results, &config))) {
        eprintln!("Application error: {}", e);
    }
}

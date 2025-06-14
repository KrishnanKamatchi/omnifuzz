
use crate::config::AppConfig;
use walkdir::WalkDir;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct SearchResult {
    pub path: String,
    pub score: i64,
}

pub fn run_finder(config: &AppConfig) -> Result<Vec<SearchResult>, String> {
    let matcher = SkimMatcherV2::default();
    let mut results = vec![];

    for entry in WalkDir::new(&config.search_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path().display().to_string();
        let file_name = entry.file_name().to_string_lossy();

        if let Some(ref term) = config.search_term {
            if let Some(score) = matcher.fuzzy_match(&file_name, term) {
                results.push(SearchResult { path, score });
            }
        } else {
            results.push(SearchResult { path, score: 0 });
        }
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));
    Ok(results)
}

use chrono::{DateTime, TimeZone, Utc};
use git2::{Commit, DiffStats, Repository};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

/// Open a Git repository and return it.
pub(crate) fn get_repo(repo_path: &str) -> Repository {
    Repository::open(repo_path).expect("Failed to open the repository")
}

/// Get a vector of commits based on the specified range.
pub(crate) fn get_commits<'repo>(
    repo: &'repo Repository,
    after: &str,
    before: &str,
) -> Result<Vec<Commit<'repo>>, Box<dyn Error + 'static>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    if !before.is_empty() && !after.is_empty() {
        let before_oid = repo.revparse_single(before)?.id();
        let after_oid = repo.revparse_single(after)?.id();
        revwalk.push_range(format!("{before_oid}..{after_oid}").as_str())?;
    } else if !after.is_empty() {
        let after_oid = repo.revparse_single(after)?.id();
        revwalk.push_range(format!("..{after_oid}").as_str())?;
    } else if !before.is_empty() {
        let before_oid = repo.revparse_single(before)?.id();
        revwalk.push_range(format!("{before_oid}..").as_str())?;
    }

    let commits: Vec<Commit> = revwalk
        .filter_map(|id| repo.find_commit(id.ok()?).ok())
        .collect();

    Ok(commits)
}

/// Get commit statistics for a vector of commits by a specific user.
pub(crate) fn get_commit_stats<'repo>(
    repo: &'repo Repository,
    commits: &[Commit<'repo>],
    user_name: &str,
) -> Vec<Result<DiffStats, Box<dyn Error>>> {
    commits
        .iter()
        .filter(|commit| commit.author().name().unwrap_or("") == user_name)
        .map(|commit| get_commit_stats_for_commit(repo, commit))
        .collect()
}

/// Get commit statistics for a specific commit.
fn get_commit_stats_for_commit<'repo>(
    repo: &'repo Repository,
    commit: &Commit<'repo>,
) -> Result<DiffStats, Box<dyn Error + 'static>> {
    let old_tree = if commit.parent_count() > 0 {
        Some(commit.parent(0)?.tree()?)
    } else {
        None
    };
    let diff = repo.diff_tree_to_tree(old_tree.as_ref(), Some(&commit.tree()?), None)?;

    diff.stats().map_err(Into::into)
}

/// Display commit statistics.
pub(crate) fn show_commit_stats(stats: &[Result<DiffStats, Box<dyn Error>>], user_name: &String) {
    if stats.is_empty() {
        println!("Warning: The user \"{user_name}\" did not contribute to this repository.");
        return;
    }

    let (total_files_changed, total_insertions, total_deletions) =
        stats.iter().fold((0, 0, 0), |acc, commit_stats| {
            if let Ok(stats) = commit_stats {
                (
                    acc.0 + stats.files_changed(),
                    acc.1 + stats.insertions(),
                    acc.2 + stats.deletions(),
                )
            } else {
                acc
            }
        });

    println!("Commit statistics for user \"{user_name}\":");
    println!("Files changed: {total_files_changed}");
    println!("Insertions: {total_insertions}");
    println!("Deletions: {total_deletions}");
}

/// Display a message about coding habits.
pub(crate) fn show_coding_habits(commits: &Vec<Commit>) {
    let mut commit_messages = Vec::new();
    let mut commit_times = Vec::new();

    for commit in commits {
        commit_messages.push(commit.message().unwrap_or("").to_string());
        commit_times.push(commit.time().seconds());
    }

    if commit_messages.is_empty() {
        println!("No commit data available for analysis.");
        return;
    }

    // Simple analysis: Counting word occurrences in commit messages
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let word_regex = Regex::new(r"\b\w+\b").unwrap();

    for message in &commit_messages {
        for word in word_regex.find_iter(message.to_lowercase().as_str()) {
            let word_entry = word_counts.entry(word.as_str().to_owned()).or_insert(0);
            *word_entry += 1;
        }
    }

    println!("Commit message word occurrences:");
    for (word, count) in word_counts.iter().sorted_by(|a, b| b.1.cmp(a.1)).take(10) {
        println!("{word}: {count}");
    }

    let commit_date_times: Vec<DateTime<Utc>> = commit_times
        .into_iter()
        .map(|timestamp| Utc.timestamp_opt(timestamp, 0).unwrap())
        .collect();

    println!("\nCommit activity:");

    let mut commit_activity: HashMap<String, usize> = HashMap::new();
    for date_time in &commit_date_times {
        let date = date_time.format("%d-%m-%Y").to_string();
        let count = commit_activity.entry(date).or_insert(0);
        *count += 1;
    }

    let (most_active_day, most_active_day_count) =
        commit_activity.iter().max_by_key(|x| x.1).unwrap();
    println!("Most active day: {most_active_day} with {most_active_day_count} commits");

    let mut commit_activity_hour: HashMap<String, usize> = HashMap::new();
    for date_time in &commit_date_times {
        let hour = date_time.format("%H").to_string();
        let count = commit_activity_hour.entry(hour).or_insert(0);
        *count += 1;
    }

    let (most_active_hour, most_active_hour_count) =
        commit_activity_hour.iter().max_by_key(|x| x.1).unwrap();

    println!("Most active hour: {most_active_hour} with {most_active_hour_count} commits");
}

pub(crate) fn show_top_committers(max: usize, commits: &Vec<Commit>) {
    let mut commit_counts: HashMap<String, usize> = HashMap::new();

    for commit in commits {
        if let Some(author_name) = commit.author().name() {
            *commit_counts.entry(author_name.to_string()).or_insert(0) += commit.parent_count();
        }
    }

    let mut top_committers: Vec<(&String, &usize)> = commit_counts.iter().collect();

    top_committers.sort_by(|a, b| b.1.cmp(a.1));

    println!("Top {max} committers:");
    for (name, count) in top_committers.iter().take(max) {
        println!("{name}: {count}");
    }
}

/// Get the user name from the Git configuration.
pub(crate) fn get_user_name() -> String {
    git2::Config::open_default()
        .and_then(|config| config.get_string("user.name"))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::analyzer::{get_commits, get_repo, get_user_name};

    #[test]
    fn show_commit_stats() {
        let repo = get_repo(".");
        println!("Repository: {}", repo.path().to_string_lossy());
        assert!(repo.is_empty().is_ok(), "Failed to get repository");

        let commits = get_commits(&repo, "", "");
        println!("Commits: {commits:?}");
        assert!(commits.is_ok(), "Failed to get commits");

        let user = get_user_name();
        println!("User: {user}");
        assert!(!user.is_empty(), "Failed to get user name");
    }
}

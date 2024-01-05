use git2::{Commit, DiffStats, Repository};
use std::error::Error;

pub(crate) fn get_repo(repo_path: &str) -> Repository {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    return repo;
}

pub(crate) fn get_commits<'repo>(
    repo: &'repo Repository,
    time_range: &str,
) -> Result<Vec<Commit<'repo>>, Box<dyn Error + 'static>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    // Optional: Parse and apply time range filter
    if !time_range.is_empty() {
        let time_range_spec = format!("HEAD --since={}", time_range);
        revwalk.push_range(time_range_spec.as_str())?;
    }

    let commits: Vec<Commit> = revwalk
        .filter_map(|id| {
            let id = match id {
                Ok(id) => id,
                Err(_) => return None,
            };

            repo.find_commit(id).ok()
        })
        .collect();

    return Ok(commits);
}

pub(crate) fn get_commit_stats<'repo>(
    repo: &'repo Repository,
    commit: &Vec<Commit<'repo>>,
    user_name: &str
) -> Vec<Result<DiffStats, Box<dyn Error>>> {
    let mut stats = Vec::new();
    for commit in commit.iter() {
        if commit.parent_count() == 0 || commit.author().name().unwrap() != user_name {
            continue;
        }

        let commit_stats = get_commit_stats_for_commit(repo, commit);
        stats.push(commit_stats);
    }

    return stats;
}

fn get_commit_stats_for_commit<'repo>(
    repo: &'repo Repository,
    commit: &Commit<'repo>,
) -> Result<DiffStats, Box<dyn Error + 'static>> {
    let parent = commit.parent(0)?;
    let diff = repo.diff_tree_to_tree(
        Some(&parent.tree()?),
        Some(&commit.tree()?),
        None,
    )?;

    let stats = diff.stats()?;
    return Ok(stats);
}

pub(crate) fn show_commit_stats(stats: &Vec<Result<DiffStats, Box<dyn Error>>>) {
    let mut total_files_changed = 0;
    let mut total_insertions = 0;
    let mut total_deletions = 0;

    for commit_stats in stats.iter() {
        let commit_stats = commit_stats.as_ref().ok();
        total_files_changed += commit_stats.map_or(0, |stats| stats.files_changed());
        total_insertions += commit_stats.map_or(0, |stats| stats.insertions());
        total_deletions += commit_stats.map_or(0, |stats| stats.deletions());
    }

    println!("Files changed: {}", total_files_changed);
    println!("Insertions: {}", total_insertions);
    println!("Deletions: {}", total_deletions);
}

pub(crate) fn show_coding_habits() {
    println!("Coding habits: ");
}

pub(crate) fn get_user_name() -> String {
    let user_name = match git2::Config::open_default() {
        Ok(config) => config.get_string("user.name").unwrap(),
        Err(_) => String::from(""),
    };
    return user_name;
}
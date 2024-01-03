use git2::{Commit, Repository};
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
) {
    for commit in commit.iter() {
        if commit.parent_count() == 0 {
            continue;
        }

        let stats = get_commit_stats_for_commit(repo, commit);
        show_commit_stats(&stats.unwrap());
    }
}

fn get_commit_stats_for_commit<'repo>(
    repo: &'repo Repository,
    commit: &Commit<'repo>,
) -> Result<git2::DiffStats, Box<dyn Error + 'static>> {
    let parent = commit.parent(0)?;
    let diff = repo.diff_tree_to_tree(
        Some(&parent.tree()?),
        Some(&commit.tree()?),
        None,
    )?;

    let stats = diff.stats()?;
    return Ok(stats);
}

pub(crate) fn show_commit_stats(commit_stats: &git2::DiffStats) {
    println!("Files changed: {}", commit_stats.files_changed());
    println!("Insertions: {}", commit_stats.insertions());
    println!("Deletions: {}", commit_stats.deletions());
}

pub(crate) fn show_coding_habits() {
    println!("Coding habits: ");
}
use git2::{Commit, DiffStats, Repository};
use std::error::Error;

pub(crate) fn get_repo(repo_path: &str) -> Repository {
    match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

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
        revwalk.push_range(format!("{}..{}", before_oid, after_oid).as_str())?;
    } else if !after.is_empty() {
        let after_oid = repo.revparse_single(after)?.id();
        revwalk.push_range(format!("..{}", after_oid).as_str())?;
    } else if !before.is_empty() {
        let before_oid = repo.revparse_single(before)?.id();
        revwalk.push_range(format!("{}..", before_oid).as_str())?;
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

    Ok(commits)
}

pub(crate) fn get_commit_stats<'repo>(
    repo: &'repo Repository,
    commit: &[Commit<'repo>],
    user_name: &str,
) -> Vec<Result<DiffStats, Box<dyn Error>>> {
    let mut stats = Vec::new();
    for commit in commit.iter() {
        if commit.parent_count() == 0 || commit.author().name().unwrap() != user_name {
            continue;
        }

        let commit_stats = get_commit_stats_for_commit(repo, commit);
        stats.push(commit_stats);
    }

    stats
}

fn get_commit_stats_for_commit<'repo>(
    repo: &'repo Repository,
    commit: &Commit<'repo>,
) -> Result<DiffStats, Box<dyn Error + 'static>> {
    let parent = commit.parent(0)?;
    let diff = repo.diff_tree_to_tree(Some(&parent.tree()?), Some(&commit.tree()?), None)?;

    let stats = diff.stats()?;
    Ok(stats)
}

pub(crate) fn show_commit_stats(stats: &[Result<DiffStats, Box<dyn Error>>]) {
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
    match git2::Config::open_default() {
        Ok(config) => config.get_string("user.name").unwrap(),
        Err(_) => String::from(""),
    }
}

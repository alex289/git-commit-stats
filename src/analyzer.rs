use git2::{Commit, DiffStats, Repository};
use std::error::Error;

/// Open a Git repository and return it.
pub(crate) fn get_repo(repo_path: &str) -> Repository {
    Repository::open(repo_path).expect("Failed to open the repository")
}

/// Get a vector of commits based on the specified range and user name.
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
        .filter(|commit| {
            commit.parent_count() > 0 && commit.author().name().unwrap_or("") == user_name
        })
        .map(|commit| get_commit_stats_for_commit(repo, commit))
        .collect()
}

/// Get commit statistics for a specific commit.
fn get_commit_stats_for_commit<'repo>(
    repo: &'repo Repository,
    commit: &Commit<'repo>,
) -> Result<DiffStats, Box<dyn Error + 'static>> {
    let parent = commit.parent(0)?;
    let diff = repo.diff_tree_to_tree(Some(&parent.tree()?), Some(&commit.tree()?), None)?;

    diff.stats().map_err(Into::into)
}

/// Display commit statistics.
pub(crate) fn show_commit_stats(stats: &[Result<DiffStats, Box<dyn Error>>]) {
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

    println!("Files changed: {}", total_files_changed);
    println!("Insertions: {}", total_insertions);
    println!("Deletions: {}", total_deletions);
}

/// Display a message about coding habits.
pub(crate) fn show_coding_habits() {
    println!("Coding habits: ");
}

/// Get the user name from the Git configuration.
pub(crate) fn get_user_name() -> String {
    git2::Config::open_default()
        .and_then(|config| config.get_string("user.name"))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::analyzer::{get_commit_stats, get_commits, get_repo, get_user_name};

    #[test]
    fn show_commit_stats() {
        let repo = get_repo(".");
        println!("Repository: {}", repo.path().to_string_lossy());
        assert!(repo.is_empty().is_ok(), "Failed to get repository");

        let commits = get_commits(&repo, "", "");
        println!("Commits: {:?}", commits);
        assert!(commits.is_ok(), "Failed to get commits");

        let user = get_user_name();
        println!("User: {}", user);
        assert!(!user.is_empty(), "Failed to get user name");

        let stats = get_commit_stats(&repo, &commits.unwrap(), &user);
        println!("Commit Stats: {:?}", stats);
        assert!(!stats.is_empty(), "Failed to get commit stats");
    }
}

mod analyzer;

use clap::Parser;
use std::{env, fs, process};

/// Struct to define command-line arguments using Clap.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Git repository [default = current directory]
    #[arg(short, long)]
    repo_path: Option<String>,

    /// Commit hash which commits should be analyzed [default = all]
    #[arg(short, long)]
    after: Option<String>,

    /// Commit hash before which commits should be analyzed [default = all]
    #[arg(short, long)]
    before: Option<String>,

    /// User name for commit analysis [default = `git config user.name`]
    #[arg(short, long)]
    user: Option<String>,

    /// Amount of of top committers to show [default = 3]
    #[arg(short, long)]
    top_committers: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let current_dir = match args.repo_path {
        Some(repo_path) => repo_path.clone(),
        None => env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string(),
    };

    if !check_directory_and_git(&current_dir) {
        process::exit(1);
    }

    let user_name = match args.user {
        Some(user) => user.clone(),
        None => analyzer::get_user_name(),
    };

    if user_name.is_empty() {
        eprintln!("Error: Failed to get user name.");
        process::exit(1);
    }

    let repo = analyzer::get_repo(&current_dir);

    if repo.is_empty().is_err() {
        eprintln!("Error: Failed to get repository.");
        process::exit(1);
    }

    let commits = analyzer::get_commits(
        &repo,
        &args.after.unwrap_or_default(),
        &args.before.unwrap_or_default(),
    );

    if commits.is_err() {
        eprintln!("Error: Failed to get commits.");
        process::exit(1);
    }

    if commits.as_ref().unwrap().is_empty() {
        eprintln!("No commits found.");
        process::exit(0);
    }

    let commits_vec = commits.unwrap();
    let stats = analyzer::get_commit_stats(&repo, &commits_vec, &user_name);

    analyzer::show_commit_stats(&stats, &user_name);
    println!();
    analyzer::show_coding_habits(&commits_vec);
    println!();
    analyzer::show_top_committers(args.top_committers.unwrap_or(3), &commits_vec);
}

/// Check if the specified path is a directory and a Git repository.
fn check_directory_and_git(directory_path: &str) -> bool {
    match fs::metadata(directory_path) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                eprintln!("Error: The specified path is not a directory.");
                return false;
            }
        }
        Err(_) => {
            eprintln!("Error: Directory does not exist.");
            return false;
        }
    }

    let git_path = format!("{}/.git", directory_path);
    if fs::metadata(git_path).is_err() {
        println!("Error: Directory is not a Git repository.");
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::{check_directory_and_git, main};

    #[test]
    fn test_check_directory_and_git() {
        assert!(
            check_directory_and_git("."),
            "Failed to check current directory"
        );
        assert!(
            !check_directory_and_git("/invalid/path"),
            "Failed to check non-existing directory"
        );
        assert!(
            !check_directory_and_git("/Users"),
            "Failed to check non-git directory"
        );
    }

    #[test]
    fn run_main_successfully() {
        main();
    }
}

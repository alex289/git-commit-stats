mod analyzer;

use clap::Parser;
use std::{fs, process};

/*
2. Functionality:

Analyze commit data, including frequently modified files, coding patterns, and commit frequencies.
Generate insights and statistics based on the analysis.

3. Output:
Display a summary of coding habits and patterns.
Provide suggestions for more efficient commits.

4. User Interaction:
Provide an option to export the analysis results for further review.
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Git repository (default = current directory)
    #[arg(short, long, default_value = "")]
    repo_path: String,

    /// Commit hash which commits should be analyzed (default = all)
    #[arg(short, long, default_value = "")]
    after: String,

    /// Commit hash before which commits should be analyzed (default = all)
    #[arg(short, long, default_value = "")]
    before: String,

    /// User name for commit analysis (default = git config user.name)
    #[arg(short, long, default_value = "")]
    user_name: String,
}

fn main() {
    let args = Args::parse();

    let current_dir = if !args.repo_path.is_empty() {
        args.repo_path
    } else {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        current_dir.to_string_lossy().to_string()
    };

    let success = check_directory_and_git(&current_dir);

    if !success {
        process::exit(1);
    }

    let user_name = if args.user_name.is_empty() {
        analyzer::get_user_name()
    } else {
        args.user_name
    };

    let repo = analyzer::get_repo(&current_dir);
    let commits = analyzer::get_commits(&repo, &args.after, &args.before);
    let stats = analyzer::get_commit_stats(&repo, &commits.unwrap(), &user_name);

    analyzer::show_commit_stats(&stats);
    println!();
    analyzer::show_coding_habits();
}

fn check_directory_and_git(directory_path: &str) -> bool {
    let metadata = match fs::metadata(directory_path) {
        Ok(metadata) => metadata,
        Err(_) => {
            eprintln!("Error: Directory does not exist.");
            return false;
        }
    };

    if !metadata.is_dir() {
        eprintln!("Error: The specified path is not a directory.");
        return false;
    }

    let git_path = format!("{}/.git", directory_path);
    if fs::metadata(git_path).is_err() {
        println!("Error: Directory is not a Git repository.");
        return false;
    }

    true
}

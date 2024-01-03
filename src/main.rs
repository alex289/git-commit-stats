mod analyzer;

use clap::Parser;

/*
1. User Input:

Specify the Git repository path.
Choose the time range for commit analysis.

2. Functionality:

Retrieve Git commit history using Git commands or a Git API.
Analyze commit data, including frequently modified files, coding patterns, and commit frequencies.
Generate insights and statistics based on the analysis.

3. Output:
Display a summary of coding habits and patterns.
Provide suggestions for more efficient commits.
Optionally, offer visualizations or charts to represent the data.

4. User Interaction:
Allow users to customize the analysis by specifying additional parameters.
Provide an option to export the analysis results for further review.
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Git repository
    #[arg(short, long)]
    repo_path: String,

    /// Time range for commit analysis
    #[arg(short, long, default_value = "")]
    time_range: String,
}

fn main() {
    let args = Args::parse();

    let repo = analyzer::get_repo(&args.repo_path);
    let commits = analyzer::get_commits(&repo, &args.time_range);
    let stats = analyzer::get_commit_stats(&repo, &commits.unwrap());

    analyzer::show_commit_stats(&stats);
    analyzer::show_coding_habits();
}

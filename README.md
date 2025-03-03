# git-commit-stats

![Crates.io Version](https://img.shields.io/crates/v/git-commit-stats?style=flat)
![Crates.io Total Downloads](https://img.shields.io/crates/d/git-commit-stats)

git-commit-stats is a command-line tool designed to provide insightful analysis of Git repositories. It analyzes commit histories, providing users with valuable information about coding habits and patterns. The tool aims to enhance your understanding of code changes over time.

```bash
> git-commit-stats

Commit statistics:
Files changed: 86
Insertions: 2067
Deletions: 429

Commit message word occurrences:
github: 139
clap: 136
com: 127
https: 104
dependencies: 89
1: 85
dependency: 84
chore: 79
group: 78
update: 77

Commit activity:
Most active day: 05-01-2024 with 33 commits
Most active hour: 14 with 19 commits

Top 3 committers:
Alex: 88
alex289: 36
github-actions[bot]: 23
```

## Features

- **Commit Analysis:** git-commit-stats extracts commit messages and performs analysis, including word frequency and coding patterns.
- **Insights:** Gain insights into frequently modified files, commit frequencies, and coding habits.
- **User-Friendly:** Simple command-line interface for ease of use.
- **Customization:** Tailor the analysis by specifying parameters such as the repository path and time range.

## Getting Started

### Prerequisites

- Ensure you have cargo installed on your machine.

### Installation

1. Install the cli with cargo:

```bash
cargo install git-commit-stats
```

2. Run the cli:

```bash
> git-commit-stats --help

A tool to analyze git commits

Usage: git-commit-stats [OPTIONS]

Options:
  -r, --repo-path <REPO_PATH>
          Path to the Git repository [default = current directory]
  -a, --after <AFTER>
          Commit hash which commits should be analyzed [default = all]
  -b, --before <BEFORE>
          Commit hash before which commits should be analyzed [default = all]
  -u, --user <USER>
          User name for commit analysis [default = `git config user.name`]
  -t, --top-committers <TOP_COMMITTERS>
          Amount of of top committers to show [default = 3]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Contributing

We welcome contributions! If you find any issues or have ideas for improvements, please open an issue or submit a pull request.

### License

This project is licensed under the MIT License. Feel free to use, modify, and distribute it as needed.

### Acknowledgments

- Git2 for interacting with Git repositories.
- Clap for command-line argument parsing.
- Regex for parsing commit messages.
- Itertools for efficient iteration.
- Chrono for date and time parsing.

# git-commit-stats

![Crates.io Version](https://img.shields.io/crates/v/git-commit-stats?style=flat)
![Crates.io Total Downloads](https://img.shields.io/crates/d/git-commit-stats)

git-commit-stats is a command-line tool designed to provide insightful analysis of Git repositories. It analyzes commit histories, providing users with valuable information about coding habits and patterns. The tool aims to enhance your understanding of code changes over time.

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
git-commit-stats --help
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

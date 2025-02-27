# Tavily Search CLI

A command-line interface for searching the web using the Tavily AI-powered search engine.

## Features

- Search the web using natural language queries
- Get AI-generated answers for your questions
- Extract content from specific URLs
- Customize search depth and result count
- Simple and intuitive command-line interface

## Installation

### Prerequisites

- Rust and Cargo installed
- Tavily API key (get one at [tavily.com](https://tavily.com/))

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/tavily-search.git
cd tavily-search

# Build the project
cargo build --release

# The binary will be available at ./target/release/tavily-search
```

### Setting up your API key

Create a `.env` file in the project root or set an environment variable:

```
TAVILY_API_KEY=tvly-your-api-key-here
```

## Usage

### Basic search

```bash
tavily-search "What is quantum computing?"
```

### Search without AI-generated answer

By default, searches include AI-generated answers. To disable this feature:

```bash
tavily-search --no-answer "How does blockchain work?"
```

### Control search depth

```bash
tavily-search --depth advanced "Latest developments in AI"
```

### Limit number of results

```bash
tavily-search --max-results 3 "Best hiking trails in California"
```

### Extract content from URLs

```bash
tavily-search extract https://example.com https://another-site.com
```

### Search and extract in one command

Perfect for automated tools and code agents:

```bash
tavily-search --extract-top 2 "How to parse JSON in Rust"
```

This performs a search and then automatically extracts full content from the top 2 results.

### Control extract content length

Adjust the amount of content shown for each extraction:

```bash
tavily-search --extract-top 2 --extract-limit 1000 "Rust programming language"
```

This displays up to 1000 characters from each extracted source (default is 800).

## Development

### Continuous Integration

This project uses GitHub Actions for continuous integration and deployment. The following workflows are available:

- **CI**: Runs on every pull request and push to main/master
  - Builds and tests the application
  - Runs clippy for linting
  - Checks code formatting with rustfmt

- **Cross-Platform Tests**: Ensures the application works on multiple platforms
  - Tests on Ubuntu, Windows, and macOS
  - Tests with minimum supported Rust version

- **Security Audit**: Scans dependencies for known vulnerabilities
  - Runs weekly and on dependency changes
  - Uses cargo-audit for vulnerability detection

- **Release**: Automates the release process
  - Builds binaries for multiple platforms
  - Uploads artifacts to GitHub Releases
  - Optional publishing to crates.io

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Created With Claude Code

This project was developed with assistance from [Claude Code](https://docs.anthropic.com/en/docs/agents-and-tools/claude-code/overview), Anthropic's AI coding assistant. Claude Code helped with:

- Initial project structure and implementation
- Feature development
- Testing and debugging
- CI/CD workflow setup

## License

MIT
use clap::builder::styling::{AnsiColor, Color, Styles};
use clap::{Command, CommandFactory, Parser, Subcommand};
use std::sync::OnceLock;

use crate::commands::Shell;

/// Custom styles for help output - matches worktrunk's color scheme
fn help_styles() -> Styles {
    Styles::styled()
        .header(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .usage(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .placeholder(anstyle::Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
}

/// Default command name for worktrunk
const DEFAULT_COMMAND_NAME: &str = "wt";

/// Help template for commands
const HELP_TEMPLATE: &str = "\
{before-help}{name} - {about-with-newline}\
Usage: {usage}

{all-args}{after-help}";

/// Build a clap Command for Cli with the shared help template applied recursively.
pub fn build_command() -> Command {
    apply_help_template_recursive(Cli::command(), DEFAULT_COMMAND_NAME)
}

fn apply_help_template_recursive(mut cmd: Command, path: &str) -> Command {
    cmd = cmd.help_template(HELP_TEMPLATE).display_name(path);

    for sub in cmd.get_subcommands_mut() {
        let sub_cmd = std::mem::take(sub);
        let sub_path = format!("{} {}", path, sub_cmd.get_name());
        let sub_cmd = apply_help_template_recursive(sub_cmd, &sub_path);
        *sub = sub_cmd;
    }
    cmd
}

fn version_str() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        let git_version = env!("VERGEN_GIT_DESCRIBE");
        let cargo_version = env!("CARGO_PKG_VERSION");

        if git_version.contains("IDEMPOTENT") {
            cargo_version.to_string()
        } else {
            git_version.to_string()
        }
    })
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    /// Human-readable table format
    Table,
    /// JSON output
    Json,
}

#[derive(Parser)]
#[command(name = "wt")]
#[command(about = "Git worktree management", long_about = None)]
#[command(version = version_str())]
#[command(disable_help_subcommand = true)]
#[command(styles = help_styles())]
#[command(arg_required_else_help = true)]
#[command(
    after_long_help = r#"See `wt config --help` for configuration file locations and setup."#
)]
pub struct Cli {
    /// Working directory for this command
    #[arg(
        short = 'C',
        global = true,
        value_name = "path",
        display_order = 100,
        help_heading = "Global Options"
    )]
    pub directory: Option<std::path::PathBuf>,

    /// User config file path
    #[arg(
        long,
        global = true,
        value_name = "path",
        display_order = 101,
        help_heading = "Global Options"
    )]
    pub config: Option<std::path::PathBuf>,

    /// Show commands and debug info
    #[arg(
        long,
        short = 'v',
        global = true,
        display_order = 102,
        help_heading = "Global Options"
    )]
    pub verbose: bool,

    /// Shell wrapper mode
    #[arg(long, global = true, hide = true)]
    pub internal: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum ConfigShellCommand {
    /// Generate shell integration code
    #[command(after_long_help = r#"## Manual Setup

Add one line to the shell config:

Bash (~/.bashrc):
```console
eval "$(wt config shell init bash)"
```

Fish (~/.config/fish/config.fish):
```fish
wt config shell init fish | source
```

Zsh (~/.zshrc):
```zsh
eval "$(wt config shell init zsh)"
```

## Auto Setup

Use `wt config shell install` to add to the shell config automatically."#)]
    Init {
        /// Shell to generate code for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Write shell integration to config files
    #[command(after_long_help = r#"## Auto Setup

Detects existing shell config files and adds integration:
```console
wt config shell install
```

Install for specific shell only:
```console
wt config shell install zsh
```

Shows proposed changes and waits for confirmation before modifying any files.
Use --force to skip confirmation."#)]
    Install {
        /// Shell to install (default: all)
        #[arg(value_enum)]
        shell: Option<Shell>,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Remove shell integration from config files
    #[command(after_long_help = r#"## Removal

Removes shell integration lines from config files:
```console
wt config shell uninstall
```

Remove from specific shell only:
```console
wt config shell uninstall zsh
```

Skip confirmation prompt:
```console
wt config shell uninstall --force
```

## Version Tolerance

Detects various forms of the integration pattern regardless of:
- Command prefix (wt, worktree, etc.)
- Minor syntax variations between versions"#)]
    Uninstall {
        /// Shell to uninstall (default: all)
        #[arg(value_enum)]
        shell: Option<Shell>,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum ApprovalsCommand {
    /// Store approvals in config
    #[command(
        after_long_help = r#"Prompts for approval of all project commands and saves them to user config.

By default, shows only unapproved commands. Use `--all` to review all commands
including previously approved ones. Use `--force` to approve without prompts."#
    )]
    Add {
        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,

        /// Show all commands
        #[arg(long)]
        all: bool,
    },

    /// Clear approved commands from config
    #[command(
        after_long_help = r#"Removes saved approvals, requiring re-approval on next command run.

By default, clears approvals for the current project. Use `--global` to clear
all approvals across all projects."#
    )]
    Clear {
        /// Clear global approvals
        #[arg(short, long)]
        global: bool,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Shell integration setup
    Shell {
        #[command(subcommand)]
        action: ConfigShellCommand,
    },

    /// Create user configuration file
    #[command(
        after_long_help = concat!(
            "Creates `~/.config/worktrunk/config.toml` with the following content:\n\n```\n",
            include_str!("../dev/config.example.toml"),
            "```"
        )
    )]
    Create,

    /// Show configuration files & locations
    #[command(
        after_long_help = r#"Shows location and contents of user config (`~/.config/worktrunk/config.toml`)
and project config (`.config/wt.toml`).

If a config file doesn't exist, shows defaults that would be used.

## Doctor Mode

Use `--doctor` to test commit generation with a synthetic diff:

```console
wt config show --doctor
```

This verifies that the LLM command is configured correctly and can generate
commit messages."#
    )]
    Show {
        /// Test commit generation pipeline
        #[arg(long)]
        doctor: bool,
    },

    /// Manage caches (CI status, default branch)
    Cache {
        #[command(subcommand)]
        action: CacheCommand,
    },

    /// Get or set runtime variables (stored in git config)
    #[command(
        after_long_help = r#"Variables are runtime values stored in git config, separate from
configuration files. Use `wt config show` to view file-based configuration.

## Available Variables

- **default-branch**: The repository's default branch (read-only, cached)
- **marker**: Custom status marker for a branch (shown in `wt list`)

## Examples

Get the default branch:
```console
wt config var get default-branch
```

Set a marker for current branch:
```console
wt config var set marker "🚧 WIP"
```

Clear markers:
```console
wt config var clear marker --all
```"#
    )]
    Var {
        #[command(subcommand)]
        action: VarCommand,
    },

    /// Manage command approvals
    #[command(after_long_help = r#"## How Approvals Work

Commands from project hooks (.config/wt.toml) and LLM configuration require
approval on first run. This prevents untrusted projects from running arbitrary
commands.

**Approval flow:**
1. Command is shown with expanded template variables
2. User approves or denies
3. Approved commands are saved to user config under `[projects."project-id"]`

**When re-approval is required:**
- Command template changes (not just variable values)
- Project ID changes (repository moves)

**Bypassing prompts:**
- `--force` flag on individual commands (e.g., `wt merge --force`)
- Useful for CI/automation where prompts aren't possible

## Examples

Pre-approve all commands for current project:
```console
wt config approvals add
```

Clear approvals for current project:
```console
wt config approvals clear
```

Clear global approvals:
```console
wt config approvals clear --global
```"#)]
    Approvals {
        #[command(subcommand)]
        action: ApprovalsCommand,
    },
}

#[derive(Subcommand)]
pub enum CacheCommand {
    /// Show cached data
    #[command(after_long_help = r#"Shows all cached data including:

- **Default branch**: Cached result of querying remote for default branch
- **CI status**: Cached GitHub/GitLab CI status per branch (30s TTL)

CI cache entries show status, age, and the commit SHA they were fetched for."#)]
    Show,

    /// Clear cached data
    Clear {
        /// Cache type: 'ci' or 'default-branch' (omit for all)
        #[arg(value_parser = ["ci", "default-branch"])]
        cache_type: Option<String>,
    },

    /// Refresh default branch from remote
    #[command(
        after_long_help = r#"Queries the remote to determine the default branch and caches the result.

Use when the remote default branch has changed. The cached value is used by
`wt merge`, `wt list`, and other commands that reference the default branch."#
    )]
    Refresh,
}

#[derive(Subcommand)]
pub enum VarCommand {
    /// Get a variable value
    #[command(after_long_help = r#"Variables:

- **default-branch**: The repository's default branch (main, master, etc.)
- **marker**: Custom status marker for a branch (shown in `wt list`)
- **ci-status**: CI/PR status for a branch (passed, running, failed, conflicts, noci)

## Examples

Get the default branch:
```console
wt config var get default-branch
```

Force refresh from remote:
```console
wt config var get default-branch --refresh
```

Get marker for current branch:
```console
wt config var get marker
```

Get marker for a specific branch:
```console
wt config var get marker --branch=feature
```

Get CI status for current branch:
```console
wt config var get ci-status
```

Force refresh CI status (bypass cache):
```console
wt config var get ci-status --refresh
```"#)]
    Get {
        /// Variable: 'default-branch', 'marker', or 'ci-status'
        #[arg(value_parser = ["default-branch", "marker", "ci-status"])]
        key: String,

        /// Force refresh (for cached variables)
        #[arg(long)]
        refresh: bool,

        /// Target branch (for branch-scoped variables)
        #[arg(long, add = crate::completion::branch_value_completer())]
        branch: Option<String>,
    },

    /// Set a variable value
    #[command(after_long_help = r#"Variables:

- **marker**: Custom status marker displayed in `wt list` output

## Examples

Set marker for current branch:
```console
wt config var set marker "🚧 WIP"
```

Set marker for a specific branch:
```console
wt config var set marker "✅ ready" --branch=feature
```"#)]
    Set {
        /// Variable: 'marker'
        #[arg(value_parser = ["marker"])]
        key: String,

        /// Value to set
        value: String,

        /// Target branch (defaults to current)
        #[arg(long, add = crate::completion::branch_value_completer())]
        branch: Option<String>,
    },

    /// Clear a variable value
    #[command(after_long_help = r#"Variables:

- **marker**: Custom status marker for a branch

## Examples

Clear marker for current branch:
```console
wt config var clear marker
```

Clear marker for a specific branch:
```console
wt config var clear marker --branch=feature
```

Clear all markers:
```console
wt config var clear marker --all
```"#)]
    Clear {
        /// Variable: 'marker'
        #[arg(value_parser = ["marker"])]
        key: String,

        /// Target branch (defaults to current)
        #[arg(long, add = crate::completion::branch_value_completer(), conflicts_with = "all")]
        branch: Option<String>,

        /// Clear all values
        #[arg(long)]
        all: bool,
    },
}

/// Workflow building blocks
#[derive(Subcommand)]
pub enum StepCommand {
    /// Commit changes with LLM commit message
    Commit {
        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,

        /// Skip pre-commit hooks
        #[arg(long = "no-verify", action = clap::ArgAction::SetFalse, default_value_t = true)]
        verify: bool,

        /// What to stage before committing [default: all]
        #[arg(long)]
        stage: Option<crate::commands::commit::StageMode>,
    },

    /// Squash commits with LLM commit message
    Squash {
        /// Target branch
        ///
        /// Defaults to default branch.
        #[arg(add = crate::completion::branch_value_completer())]
        target: Option<String>,

        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,

        /// Skip pre-commit hooks
        #[arg(long = "no-verify", action = clap::ArgAction::SetFalse, default_value_t = true)]
        verify: bool,

        /// What to stage before committing [default: all]
        #[arg(long)]
        stage: Option<crate::commands::commit::StageMode>,
    },

    /// Push changes to local target branch
    ///
    /// Automatically stashes non-conflicting edits in the target worktree before
    /// the push and restores them afterward so other agents' changes stay intact.
    Push {
        /// Target branch
        ///
        /// Defaults to default branch.
        #[arg(add = crate::completion::branch_value_completer())]
        target: Option<String>,

        /// Allow merge commits
        #[arg(long)]
        allow_merge_commits: bool,
    },

    /// Rebase onto target
    Rebase {
        /// Target branch
        ///
        /// Defaults to default branch.
        #[arg(add = crate::completion::branch_value_completer())]
        target: Option<String>,
    },

    /// Run post-create hook
    PostCreate {
        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,
    },

    /// Run post-start hook
    PostStart {
        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,
    },

    /// Run pre-commit hook
    PreCommit {
        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,
    },

    /// Run pre-merge hook
    PreMerge {
        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,
    },

    /// Run post-merge hook
    PostMerge {
        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,
    },
}

/// Subcommands for `wt list`
#[derive(Subcommand)]
pub enum ListSubcommand {
    /// Single-line status for shell prompts
    ///
    /// Format: `branch  status  ±working  commits  upstream  ci`
    ///
    /// Designed for shell prompts, starship, or editor integrations.
    /// Uses same collection infrastructure as `wt list`.
    Statusline {
        /// Claude Code mode: read context from stdin, add directory and model
        ///
        /// Reads JSON from stdin with `.workspace.current_dir` and `.model.display_name`.
        /// Output: `dir  branch  status  ±working  commits  upstream  ci  | model`
        #[arg(long)]
        claude_code: bool,
    },
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage configuration and shell integration
    #[command(
        about = "Manage configuration and shell integration",
        after_long_help = r#"Manages configuration, shell integration, and runtime settings. The command provides subcommands for setup, inspection, and cache management.

## Examples

Install shell integration (required for directory switching):

```console
wt config shell install
```

Create user config file with documented examples:

```console
wt config create
```

Show current configuration and file locations:

```console
wt config show
```

## Shell Integration

Shell integration allows Worktrunk to change the shell's working directory after `wt switch`. Without it, commands run in a subprocess and directory changes don't persist.

The `wt config shell install` command adds integration to the shell's config file. Manual installation:

```console
# For bash: add to ~/.bashrc
eval "$(wt config shell init bash)"

# For zsh: add to ~/.zshrc
eval "$(wt config shell init zsh)"

# For fish: add to ~/.config/fish/config.fish
wt config shell init fish | source
```

## Configuration Files

**User config** — `~/.config/worktrunk/config.toml` (or `$WORKTRUNK_CONFIG_PATH`):

Personal settings like LLM commit generation, path templates, and default behaviors. The `wt config create` command generates a file with documented examples.

**Project config** — `.config/wt.toml` in repository root:

Project-specific hooks: post-create, post-start, pre-commit, pre-merge, post-merge. See [Hooks](/hooks/) for details.

## LLM Commit Messages

Worktrunk can generate commit messages using an LLM. Enable in user config:

```toml
[commit-generation]
command = "llm"
```

See [LLM Commits](/llm-commits/) for installation, provider setup, and customization.
"#
    )]
    Config {
        #[command(subcommand)]
        action: ConfigCommand,
    },

    /// Workflow building blocks
    #[command(
        name = "step",
        after_long_help = r#"Individual workflow steps for scripting and automation. Each subcommand performs one step of the `wt merge` pipeline — commit, squash, rebase, push, or run hooks — allowing custom workflows or manual intervention between steps.

## Examples

Commit with an LLM-generated message:

```console
wt step commit
```

Squash all branch commits into one:

```console
wt step squash
```

Run pre-merge hooks (tests, lints):

```console
wt step pre-merge
```

Rebase onto main:

```console
wt step rebase
```

## Use Cases

**Custom merge workflow** — Run steps individually when `wt merge` doesn't fit, such as adding manual review between squash and rebase:

```console
wt step commit
wt step squash
# manual review here
wt step rebase
wt step pre-merge
wt step push
```

**CI integration** — Run hooks explicitly in CI environments:

```console
wt step pre-merge --force  # skip approval prompts
```

## Subcommands

| Command | Description |
|---------|-------------|
| `commit` | Commits uncommitted changes with an [LLM-generated message](/llm-commits/) |
| `squash` | Squashes all branch commits into one with an [LLM-generated message](/llm-commits/) |
| `rebase` | Rebases the branch onto the target (default: main) |
| `push` | Pushes changes to the local target branch |
| `post-create` | Runs post-create hooks |
| `post-start` | Runs post-start hooks |
| `pre-commit` | Runs pre-commit hooks |
| `pre-merge` | Runs pre-merge hooks |
| `post-merge` | Runs post-merge hooks |
"#
    )]
    Step {
        #[command(subcommand)]
        action: StepCommand,
    },

    /// Interactive worktree selector
    ///
    /// Toggle preview tabs with 1/2/3 keys. Toggle preview visibility with alt-p.
    #[cfg(unix)]
    #[command(
        after_long_help = r#"Interactive worktree picker with live preview. The selector shows worktree state at a glance — diff stats, commit history, and working tree status — without switching directories first.

## Examples

Open the interactive selector:

```console
wt select
```

## The Interface

The selector displays a two-panel layout: a worktree list on the left and a preview panel on the right. The preview updates automatically when navigating between worktrees.

**Preview tabs** — toggled with number keys:

1. **Diff** — Changes relative to main branch
2. **Log** — Recent commits on the branch
3. **Status** — Working tree status (staged, modified, untracked)

## Keybindings

| Key | Action |
|-----|--------|
| `↑`/`↓` or `j`/`k` | Navigate worktree list |
| `Enter` | Switch to selected worktree |
| `Esc` or `q` | Cancel |
| `/` | Filter worktrees |
| `1`/`2`/`3` | Switch preview tab |
| `Alt+p` | Toggle preview panel |
"#
    )]
    Select,

    /// List worktrees and optionally branches
    #[command(
        after_long_help = r#"Show all worktrees with their status at a glance. The table includes uncommitted changes, divergence from main and remote, and optional CI status.

## Examples

List all worktrees:

```console
wt list
```

Include CI status and conflict detection:

```console
wt list --full
```

Include branches that don't have worktrees:

```console
wt list --branches
```

Output as JSON for scripting:

```console
wt list --format=json
```

## Status Symbols

The Status column shows a compact summary. Symbols appear in this order:

| Symbol | Meaning |
|--------|---------|
| `+` | Staged files (ready to commit) |
| `!` | Modified files (unstaged changes) |
| `?` | Untracked files |
| `✖` | Merge conflicts (fix before continuing) |
| `⊘` | Would conflict if merged to main |
| `≡` | Matches main (identical contents) |
| `_` | No commits (empty branch) |
| `↻` | Rebase in progress |
| `⋈` | Merge in progress |
| `↑` | Ahead of main |
| `↓` | Behind main |
| `↕` | Diverged from main |
| `⇡` | Ahead of remote |
| `⇣` | Behind remote |
| `⇅` | Diverged from remote |
| `⎇` | Branch without worktree |
| `⌫` | Prunable (directory missing) |
| `⊠` | Locked worktree |

Rows are dimmed when there's no marginal contribution (`≡` matches main or `_` no commits).

## Columns

| Column | Description |
|--------|-------------|
| **Branch** | Branch name |
| **Status** | Compact symbols (see above) |
| **HEAD±** | Uncommitted changes: `+added` `-deleted` lines |
| **main↕** | Commits ahead↑/behind↓ relative to main |
| **main…±** | Line diffs in commits ahead of main (`--full` only) |
| **Path** | Worktree directory |
| **Remote⇅** | Commits ahead⇡/behind⇣ vs tracking branch |
| **CI** | Pipeline status (`--full` only) |
| **Commit** | Short hash (8 chars) |
| **Age** | Time since last commit |
| **Message** | Last commit message (truncated) |

### CI Status

The CI column (`--full`) shows pipeline status from GitHub/GitLab:

- `●` green — All checks passed
- `●` blue — Checks running
- `●` red — Checks failed
- `●` yellow — Merge conflicts with base
- `●` gray — No checks configured
- blank — No PR/MR found
- dimmed — Stale (unpushed local changes)

## JSON Output

The `--format=json` flag outputs structured data for scripting:

```console
# Find worktrees with conflicts
wt list --format=json | jq '.[] | select(.status.branch_state == "Conflicts")'

# Find worktrees with uncommitted changes
wt list --format=json | jq '.[] | select(.status.working_tree.modified)'

# Get current worktree
wt list --format=json | jq '.[] | select(.is_current == true)'

# Find branches ahead of main
wt list --format=json | jq '.[] | select(.status.main_divergence == "Ahead")'
```

**Status fields:**

- `working_tree`: `{untracked, modified, staged, renamed, deleted}` booleans
- `branch_state`: `""` | `"Conflicts"` | `"MergeTreeConflicts"` | `"MatchesMain"` | `"NoCommits"`
- `git_operation`: `""` | `"Rebase"` | `"Merge"`
- `main_divergence`: `""` | `"Ahead"` | `"Behind"` | `"Diverged"`
- `upstream_divergence`: `""` | `"Ahead"` | `"Behind"` | `"Diverged"`

**Position fields:**

- `is_main`: boolean — is the main worktree
- `is_current`: boolean — is the current directory
- `is_previous`: boolean — is the previous worktree from `wt switch`
"#
    )]
    #[command(args_conflicts_with_subcommands = true)]
    List {
        #[command(subcommand)]
        subcommand: Option<ListSubcommand>,

        /// Output format (table, json)
        #[arg(long, value_enum, default_value = "table", hide_possible_values = true)]
        format: OutputFormat,

        /// Include branches without worktrees
        #[arg(long)]
        branches: bool,

        /// Include remote branches
        #[arg(long)]
        remotes: bool,

        /// Show CI, conflicts, diffs
        #[arg(long)]
        full: bool,

        /// Show fast info immediately, update with slow info
        ///
        /// Displays local data (branches, paths, status) first, then updates
        /// with remote data (CI, upstream) as it arrives. Auto-enabled for TTY.
        #[arg(long, overrides_with = "no_progressive")]
        progressive: bool,

        /// Force buffered rendering
        #[arg(long = "no-progressive", overrides_with = "progressive", hide = true)]
        no_progressive: bool,
    },

    /// Switch to a worktree
    #[command(
        after_long_help = r#"Navigate between worktrees or create new ones. Switching to an existing worktree is just a directory change. With `--create`, a new branch and worktree are created, and hooks run.

## Examples

Switch to an existing worktree:

```console
wt switch feature-auth
```

Create a new worktree for a fresh branch:

```console
wt switch --create new-feature
```

Create from a specific base branch:

```console
wt switch --create hotfix --base production
```

Switch to the previous worktree (like `cd -`):

```console
wt switch -
```

## Creating Worktrees

The `--create` flag (or `-c`) creates a new branch from the default branch (or `--base`), sets up a worktree at `../{repo}.{branch}`, runs [post-create hooks](/hooks/#post-create) synchronously, then spawns [post-start hooks](/hooks/#post-start) in the background before switching to the new directory.

```console
# Create from main (default)
wt switch --create api-refactor

# Create from a specific branch
wt switch --create emergency-fix --base release-2.0

# Create and open in editor
wt switch --create docs --execute "code ."

# Skip all hooks
wt switch --create temp --no-verify
```

## Shortcuts

Special symbols for common targets:

| Shortcut | Meaning |
|----------|---------|
| `-` | Previous worktree (like `cd -`) |
| `@` | Current branch's worktree |
| `^` | Default branch (main/master) |

```console
wt switch -                              # Go back to previous worktree
wt switch ^                              # Switch to main worktree
wt switch --create bugfix --base=@       # Branch from current HEAD
```

## Hooks

When creating a worktree (`--create`), hooks run in this order:

1. **post-create** — Blocking, sequential. Typically: `npm install`, `cargo build`
2. **post-start** — Background, parallel. Typically: dev servers, file watchers

See [Hooks](/hooks/) for configuration details.

## How Arguments Are Resolved

Arguments resolve using **path-first lookup**:

1. Compute the expected path for the argument (using the configured path template)
2. If a worktree exists at that path, switch to it (regardless of what branch it's on)
3. Otherwise, treat the argument as a branch name

**Example**: If `repo.foo/` exists but is on branch `bar`:

- `wt switch foo` switches to `repo.foo/` (the `bar` branch worktree)
- `wt switch bar` also works (falls back to branch lookup)
"#
    )]
    Switch {
        /// Branch or worktree name
        ///
        /// Shortcuts: '^' (main), '-' (previous), '@' (current)
        #[arg(add = crate::completion::worktree_branch_completer())]
        branch: String,

        /// Create a new branch
        #[arg(short = 'c', long)]
        create: bool,

        /// Base branch
        ///
        /// Defaults to default branch.
        #[arg(short = 'b', long, add = crate::completion::branch_value_completer())]
        base: Option<String>,

        /// Command to run after switch
        #[arg(short = 'x', long)]
        execute: Option<String>,

        /// Skip approval prompts
        #[arg(short = 'f', long)]
        force: bool,

        /// Skip all project hooks
        #[arg(long = "no-verify", action = clap::ArgAction::SetFalse, default_value_t = true)]
        verify: bool,
    },

    /// Remove worktree and branch
    #[command(
        after_long_help = r#"Cleans up finished work by removing worktrees and their branches. Without arguments, removes the current worktree and returns to the main worktree.

## Examples

Remove current worktree and branch:

```console
wt remove
```

Remove a specific worktree:

```console
wt remove feature-branch
```

Keep the branch after removing the worktree:

```console
wt remove --no-delete-branch feature-branch
```

Remove multiple worktrees:

```console
wt remove old-feature another-branch
```

Force-delete an unmerged branch:

```console
wt remove -D experimental
```

## Branch Deletion

By default, branches are deleted only when their content is already integrated into the target branch (typically main). This works correctly with squash-merge and rebase workflows where commit ancestry isn't preserved but the file changes are.

The `-D` flag overrides this safety check and force-deletes unmerged branches. The `--no-delete-branch` flag prevents branch deletion entirely.

## Background Removal

Removal runs in the background by default — the command returns immediately so work can continue. Logs are written to `.git/wt-logs/{branch}-remove.log`.

The `--no-background` flag runs removal in the foreground (blocking).

## How Arguments Are Resolved

Arguments resolve using **path-first lookup**:

1. Compute the expected path for the argument (using the configured path template)
2. If a worktree exists at that path, use it (regardless of what branch it's on)
3. Otherwise, treat the argument as a branch name

**Example**: If `repo.foo/` exists but is on branch `bar`:

- `wt remove foo` removes `repo.foo/` and the `bar` branch
- `wt remove bar` also works (falls back to branch lookup)

**Shortcuts**: `@` (current worktree), `-` (previous worktree), `^` (main worktree)
"#
    )]
    Remove {
        /// Worktree or branch (@ for current)
        #[arg(add = crate::completion::worktree_branch_completer())]
        worktrees: Vec<String>,

        /// Keep branch after removal
        #[arg(long = "no-delete-branch", action = clap::ArgAction::SetFalse, default_value_t = true)]
        delete_branch: bool,

        /// Delete unmerged branches
        #[arg(short = 'D', long = "force-delete")]
        force_delete: bool,

        /// Run removal in foreground
        #[arg(long = "no-background", action = clap::ArgAction::SetFalse, default_value_t = true)]
        background: bool,
    },

    /// Merge worktree into target branch
    #[command(
        after_long_help = r#"Integrates the current branch into the target branch (default: main) and removes the worktree. All steps — commit, squash, rebase, push, cleanup — run automatically.

## Examples

Merge current worktree into main:

```console
wt merge
```

Keep the worktree after merging:

```console
wt merge --no-remove
```

Preserve commit history (no squash):

```console
wt merge --no-squash
```

Skip hooks and approval prompts:

```console
wt merge --no-verify --force
```

## The Pipeline

`wt merge` runs these steps in order:

1. **Commit** — Stages and commits uncommitted changes with an LLM-generated message. The `--stage` flag controls what gets staged: `all` (default), `tracked`, or `none`.

2. **Squash** — Combines multiple commits into one (like GitHub's "Squash and merge") with an LLM-generated message. The `--no-squash` flag preserves individual commits. A backup ref is saved to `refs/wt-backup/<branch>`.

3. **Rebase** — Rebases onto the target branch. Conflicts abort the merge immediately.

4. **Pre-merge hooks** — Project-defined commands run after rebase. Failures abort the merge.

5. **Push** — Fast-forward push to the local target branch. Non-fast-forward pushes are rejected.

6. **Cleanup** — Removes the worktree and branch. The `--no-remove` flag keeps the worktree.

7. **Post-merge hooks** — Project-defined commands run after cleanup. Failures are logged but don't abort.

## Hooks

When merging, hooks run in this order:

1. **pre-merge** — After rebase, before push. Failures abort the merge.
2. **post-merge** — After cleanup. Failures are logged but don't abort.

The `--no-verify` flag skips all hooks. See [Hooks](/hooks/) for configuration details.
"#
    )]
    Merge {
        /// Target branch
        ///
        /// Defaults to default branch.
        #[arg(add = crate::completion::branch_value_completer())]
        target: Option<String>,

        /// Force commit squashing
        #[arg(long, overrides_with = "no_squash", hide = true)]
        squash: bool,

        /// Skip commit squashing
        #[arg(long = "no-squash", overrides_with = "squash")]
        no_squash: bool,

        /// Force commit, squash, and rebase
        #[arg(long, overrides_with = "no_commit", hide = true)]
        commit: bool,

        /// Skip commit, squash, and rebase
        #[arg(long = "no-commit", overrides_with = "commit")]
        no_commit: bool,

        /// Force worktree removal after merge
        #[arg(long, overrides_with = "no_remove", hide = true)]
        remove: bool,

        /// Keep worktree after merge
        #[arg(long = "no-remove", overrides_with = "remove")]
        no_remove: bool,

        /// Force running project hooks
        #[arg(long, overrides_with = "no_verify", hide = true)]
        verify: bool,

        /// Skip all project hooks
        #[arg(long = "no-verify", overrides_with = "verify")]
        no_verify: bool,

        /// Skip approval prompts
        #[arg(short, long)]
        force: bool,

        /// What to stage before committing [default: all]
        #[arg(long)]
        stage: Option<crate::commands::commit::StageMode>,
    },
}

+++
title = "wt list"
weight = 11

[extra]
group = "Commands"
+++

Show all worktrees with their status at a glance. The table includes uncommitted changes, divergence from main and remote, and optional CI status.

## Examples

List all worktrees:

```bash
wt list
```

Include CI status and conflict detection:

```bash
wt list --full
```

Include branches that don't have worktrees:

```bash
wt list --branches
```

Output as JSON for scripting:

```bash
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

```bash
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

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt list --help-page` — edit cli.rs to update -->

```bash
wt list - List worktrees and optionally branches
Usage: wt list [OPTIONS]
       wt list <COMMAND>

Commands:
  statusline  Single-line status for shell prompts

Options:
      --format <FORMAT>
          Output format (table, json)

          [default: table]

      --branches
          Include branches without worktrees

      --remotes
          Include remote branches

      --full
          Show CI, conflicts, diffs

      --progressive
          Show fast info immediately, update with slow info

          Displays local data (branches, paths, status) first, then updates with remote data (CI, upstream) as it arrives. Auto-enabled for TTY.

  -h, --help
          Print help (see a summary with '-h')

Global Options:
  -C <path>
          Working directory for this command

      --config <path>
          User config file path

  -v, --verbose
          Show commands and debug info
```

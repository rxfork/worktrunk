<!-- markdownlint-disable MD014 MD024 MD033 -->

# Worktrunk

<!-- User badges -->

[![Crates.io](https://img.shields.io/crates/v/worktrunk?style=for-the-badge&logo=rust)](https://crates.io/crates/worktrunk)
[![License: MIT](https://img.shields.io/badge/LICENSE-MIT-blue?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![GitHub CI Status](https://img.shields.io/github/actions/workflow/status/max-sixty/worktrunk/ci.yaml?event=push&branch=main&logo=github&style=for-the-badge)](https://github.com/max-sixty/worktrunk/actions?query=branch%3Amain+workflow%3Aci)

<!-- Dev badges (uncomment when repo is public and has traction) -->
<!-- [![Downloads](https://img.shields.io/crates/d/worktrunk?style=for-the-badge&logo=rust)](https://crates.io/crates/worktrunk) -->
<!-- [![Stars](https://img.shields.io/github/stars/max-sixty/worktrunk?style=for-the-badge&logo=github)](https://github.com/max-sixty/worktrunk/stargazers) -->

Worktrunk is a CLI for Git worktree management, designed for parallel AI agent
workflows. Git worktrees give each agent an isolated branch and directory;
Worktrunk adds branch-based navigation, unified status, and lifecycle hooks. The
goal is to make spinning up a new AI "developer" for a task feel as routine as
`git switch`.

## December 2025 Project Status

I've been using Worktrunk as my daily driver, and am releasing it as Open Source
this week. It's built with love (there's no slop!). If social proof is helpful:
I also created [PRQL](https://github.com/PRQL/prql) (10k stars) and am a
maintainer of [Xarray](https://github.com/pydata/xarray) (4k stars),
[Insta](https://github.com/mitsuhiko/insta), &
[Numbagg](https://github.com/numbagg/numbagg).

I'd recommend:

- **starting with Worktrunk as a simpler & better `git worktree`**: create / navigate /
  list / clean up git worktrees with ease
- **later using the more advanced features if you find they resonate**: there's
  lots for the more ambitious, such as [LLM commit
  messages](#llm-commit-messages), or [local merging of worktrees gated on
  CI-like checks](#local-merging-with-wt-merge), or [fzf-like selector +
  preview](#interactive-worktree-picker). And QoL features, such as listing the
  CI status & the Claude Code status for all branches, or a great [Claude Code
  statusline](#statusline-integration). But they're not required to get value
  from the tool.

## Demo

![Worktrunk Demo](dev/wt-demo/out/wt-demo.gif)

## Quick Start

### 1. Install

**Homebrew (macOS):**

```bash
$ brew install max-sixty/worktrunk/wt
$ wt config shell install  # allows commands to change directories
```

**Cargo:**

```bash
$ cargo install worktrunk
$ wt config shell install
```

### 2. Create a worktree

<!-- ⚠️ AUTO-GENERATED from tests/integration_tests/snapshots/integration__integration_tests__shell_wrapper__tests__readme_example_simple_switch.snap — edit source to update -->

```bash
$ wt switch --create fix-auth
✅ Created new worktree for fix-auth from main at ../repo.fix-auth
```

<!-- END AUTO-GENERATED -->

This creates `../repo.fix-auth` on branch `fix-auth`.

### 3. Switch between worktrees

<!-- ⚠️ AUTO-GENERATED from tests/integration_tests/snapshots/integration__integration_tests__shell_wrapper__tests__readme_example_switch_back.snap — edit source to update -->

```bash
$ wt switch feature-api
✅ Switched to worktree for feature-api at ../repo.feature-api
```

<!-- END AUTO-GENERATED -->

### 4. List worktrees

<!-- ⚠️ AUTO-GENERATED from tests/snapshots/integration__integration_tests__list__readme_example_simple_list.snap — edit source to update -->

```bash
$ wt list
  Branch       Status         HEAD±    main↕  Path                Remote⇅  Commit    Age   Message
@ feature-api  +   ↑⇡      +36  -11   ↑4      ./repo.feature-api   ⇡3      b1554967  30m   Add API tests
^ main             ^⇣                         ./repo                   ⇣1  b834638e  1d    Initial commit
+ fix-auth        _                           ./repo.fix-auth              b834638e  1d    Initial commit

⚪ Showing 3 worktrees, 1 with changes, 1 ahead
```

<!-- END AUTO-GENERATED -->

`--full` adds CI status and conflicts. `--branches` includes all branches.

### 5. Clean up

Say we merged via CI, our changes are on main, and we're finished with the worktree:

<!-- ⚠️ AUTO-GENERATED from tests/integration_tests/snapshots/integration__integration_tests__shell_wrapper__tests__readme_example_remove.snap — edit source to update -->

```bash
$ wt remove
🔄 Removing feature-api worktree & branch in background (already in main)
```

<!-- END AUTO-GENERATED -->

## Why git worktrees?

We have a few options for working with multiple agents:

- one working tree with many branches — agents step on each other, can't use git
  for staging & committing
- multiple clones — slow to set up, drift out of sync
- git worktrees — multiple directories backed by a single `.git` directory

So we use git worktrees! But then...

## Why Worktrunk?

Git's built-in `worktree` commands require remembering worktrees' locations, and
composing git & `cd` commands together. In contrast, Worktrunk bundles creation,
navigation, status, and cleanup into simple commands. A few examples:

<table>
<tr>
<th>Task</th>
<th>Worktrunk</th>
<th>Plain git</th>
</tr>
<tr>
<td>Switch worktrees</td>
<td><pre lang="bash">wt switch feature</pre></td>
<td><pre lang="bash">cd ../repo.feature</pre></td>
</tr>
<tr>
<td>Create + start Claude</td>
<td><pre lang="bash">wt switch -c -x claude feature</pre>
...or with an <a href="#alias">alias</a>: <code>wsc feature</code>
</td>
<td><pre lang="bash">git worktree add -b feature ../repo.feature main
cd ../repo.feature
claude</pre></td>
</tr>

<tr>
<td>Clean up</td>
<td><pre lang="bash">wt remove</pre></td>
<td><pre lang="bash">cd ../repo
git worktree remove ../repo.feature
git branch -d feature</pre></td>
</tr>
<tr>
<td>List</td>
<td><pre lang="bash">wt list</pre>
...including diffstats & status
</td>
<td><pre lang="bash">git worktree list</pre>
...just branch names & paths
</td>
</tr>
<tr>
<td>List with CI status</td>
<td><pre lang="bash">wt list --full</pre>
...including CI status & diffstat downstream of <code>main</code>. Optionally add <code>--branches</code> or <code>--remotes</code>.
</td>
<td>N/A</td>
</tr>
</table>

...and check out examples below for more advanced workflows.

## Advanced

Many Worktrunk users will just use the commands above. For more:

### LLM commit messages

Worktrunk can invoke external commands to generate commit messages.
[llm](https://llm.datasette.io/) from [**@simonw**](https://github.com/simonw) is recommended.

Add to user config (`~/.config/worktrunk/config.toml`):

```toml
[commit-generation]
command = "llm"
args = ["-m", "claude-haiku-4-5-20251001"]
```

`wt merge` generates commit messages automatically or `wt step commit` runs just the commit step.

For custom prompt templates: `wt config --help`.

### Project hooks

Automate setup and validation at worktree lifecycle events:

| Hook            | When                                | Example                      |
| --------------- | ----------------------------------- | ---------------------------- |
| **post-create** | After worktree created              | `cp -r .cache`, `ln -s`      |
| **post-start**  | After worktree created (background) | `npm install`, `cargo build` |
| **pre-commit**  | Before creating any commit          | `pre-commit run`             |
| **pre-merge**   | After squash, before push           | `cargo test`, `pytest`       |
| **post-merge**  | After successful merge              | `cargo install --path .`     |

Project commands require approval on first run; use `--force` to skip prompts
or `--no-verify` to skip hooks entirely. Configure in `.config/wt.toml`:

```toml
# Install dependencies, build setup
[post-create]
"install" = "uv sync"

# Dev servers, file watchers (runs in background)
[post-start]
"dev" = "uv run dev"

# Tests and lints before merging (blocks on failure)
[pre-merge]
"lint" = "uv run ruff check"
"test" = "uv run pytest"
```

Example output:

<!-- ⚠️ AUTO-GENERATED from tests/integration_tests/snapshots/integration__integration_tests__shell_wrapper__tests__readme_example_hooks_post_create.snap — edit source to update -->

```bash
$ wt switch --create feature-x
🔄 Running post-create install:
   uv sync

  Resolved 24 packages in 145ms
  Installed 24 packages in 1.2s
✅ Created new worktree for feature-x from main at ../repo.feature-x
🔄 Running post-start dev:
   uv run dev
```

<!-- END AUTO-GENERATED -->

### Local merging with `wt merge`

`wt merge` handles the full merge workflow: stage, commit, squash, rebase,
merge, cleanup. Includes [LLM commit messages](#llm-commit-messages),
[project hooks](#project-hooks), and [config](#wt-config)/[flags](#wt-merge)
for skipping steps.

<table>
<tr>
<th>Task</th>
<th>Worktrunk</th>
<th>Plain git</th>
</tr>
<tr>
<td>Merge + clean up</td>
<td><pre lang="bash">wt merge</pre></td>
<td><pre lang="bash">git add -A
git reset --soft $(git merge-base HEAD main)
git diff --staged | llm "Write a commit message based on this diff" | git commit -F -
git rebase main
# pre-merge hook
cargo test
cd ../repo && git merge --ff-only feature
git worktree remove ../repo.feature
git branch -d feature
# post-merge hook
cargo install --path .  </pre></td>
</tr>
</table>

<!-- ⚠️ AUTO-GENERATED from tests/snapshots/integration__integration_tests__merge__readme_example_complex.snap — edit source to update -->

```bash
$ wt merge
🔄 Squashing 3 commits into a single commit (3 files, +33)...
🔄 Generating squash commit message...
   feat(auth): Implement JWT authentication system

   Add comprehensive JWT token handling including validation, refresh logic,
   and authentication tests. This establishes the foundation for secure
   API authentication.

   - Implement token refresh mechanism with expiry handling
   - Add JWT encoding/decoding with signature verification
   - Create test suite covering all authentication flows
✅ Squashed @ 95c3316
🔄 Running pre-merge test:
   cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/worktrunk-abc123)

running 18 tests
test auth::tests::test_jwt_decode ... ok
test auth::tests::test_jwt_encode ... ok
test auth::tests::test_token_refresh ... ok
test auth::tests::test_token_validation ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s
🔄 Running pre-merge lint:
   cargo clippy
    Checking worktrunk v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
🔄 Merging 1 commit to main @ 95c3316 (no rebase needed)
   * 95c3316 feat(auth): Implement JWT authentication system
    auth.rs      |  8 ++++++++
    auth_test.rs | 17 +++++++++++++++++
    jwt.rs       |  8 ++++++++
    3 files changed, 33 insertions(+)
✅ Merged to main (1 commit, 3 files, +33)
🔄 Removing feature-auth worktree & branch in background (already in main)
🔄 Running post-merge install:
   cargo install --path .
  Installing worktrunk v0.1.0
   Compiling worktrunk v0.1.0
    Finished release [optimized] target(s) in 2.34s
  Installing ~/.cargo/bin/wt
   Installed package `worktrunk v0.1.0` (executable `wt`)
```

<!-- END AUTO-GENERATED -->

### Claude Code Status Tracking

The Worktrunk plugin adds Claude Code session tracking to `wt list`:

<!-- ⚠️ AUTO-GENERATED from tests/snapshots/integration__integration_tests__list__with_user_marker.snap — edit source to update -->

```bash
$ wt list
  Branch       Status         HEAD±    main↕  Path                Remote⇅  Commit    Age   Message
@ main             ^                          ./repo                       b834638e  1d    Initial commit
+ feature-api      ↑  🤖              ↑1      ./repo.feature-api           9606cd0f  1d    Add REST API endpoints
+ review-ui      ? ↑  💬              ↑1      ./repo.review-ui             afd3b353  1d    Add dashboard component
+ wip-docs       ?_                           ./repo.wip-docs              b834638e  1d    Initial commit

⚪ Showing 4 worktrees, 2 ahead
```

<!-- END AUTO-GENERATED -->

- `🤖` — Claude is working
- `💬` — Claude is waiting for input

**Install the plugin:**

```bash
claude plugin marketplace add max-sixty/worktrunk
claude plugin install worktrunk@worktrunk
```

<details>
<summary>Manual status markers</summary>

Set status markers manually for any workflow:

```bash
wt config var set marker "🚧"                   # Current branch
wt config var set marker "✅" --branch feature  # Specific branch
git config worktrunk.marker.feature "💬"        # Direct git config
```

</details>

### Interactive Worktree Picker

`wt select` opens a fzf-like fuzzy-search worktree picker with diff preview. Unix only.

Preview tabs (toggle with `1`/`2`/`3`):

- **Tab 1**: Working tree changes (uncommitted)
- **Tab 2**: Commit history (commits not on main highlighted)
- **Tab 3**: Branch diff (changes ahead of main)

### Statusline Integration

`wt list statusline` outputs a single-line status for shell prompts, starship,
or editor integrations[^1].

[^1]:
    Currently this grabs CI status, so is too slow to use in synchronous
    contexts. If a faster version would be helpful, please add an Issue.

**Claude Code** (`--claude-code`): Reads workspace context from stdin, outputs
directory, branch status, and model.

```
~/w/myproject.feature-auth  !🤖  ±+42 -8  ↑3  ⇡1  ●  | Opus
```

<details>
<summary>Claude Code configuration</summary>

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "wt list statusline --claude-code"
  }
}
```

</details>

## Tips & patterns

<a id="alias"></a>**Alias for new worktree + agent:**

```bash
alias wsc='wt switch --create --execute=claude'
wsc new-feature  # Creates worktree, runs hooks, launches Claude
```

**Eliminate cold starts** — `post-create` hooks install deps and copy caches.
See [`.config/wt.toml`](.config/wt.toml) for an example using copy-on-write.

**Local CI gate** — `pre-merge` hooks run before merging. Failures abort the
merge.

**Track agent status** — Custom emoji markers show agent state in `wt list`.
Claude Code hooks can set these automatically. See [Claude Code Status
Tracking](#claude-code-status-tracking).

**Monitor CI across branches** — `wt list --full --branches` shows PR/CI status
for all branches, including those without worktrees. CI column links to PR pages
in terminals with hyperlink support.

**JSON API** — `wt list --format=json` for dashboards, statuslines, scripts.

**Task runners** — Reference Taskfile/Justfile/Makefile in hooks:

```toml
[post-create]
"setup" = "task install"

[pre-merge]
"validate" = "just test lint"
```

**Shortcuts** — `^` = default branch, `@` = current branch, `-` = previous
worktree. Example: `wt switch --create hotfix --base=@` branches from current
HEAD.

## Commands Reference

<details>
<summary><strong><code>wt switch [branch]</code></strong> - Switch to existing worktree or create a new one</summary>

<!-- ⚠️ AUTO-GENERATED from `wt switch --help-md` — edit source to update -->

```
wt switch — Switch to a worktree
Usage: wt switch [OPTIONS] <BRANCH>

Arguments:
  <BRANCH>
          Branch or worktree name

          Shortcuts: '^' (main), '-' (previous), '@' (current)

Options:
  -c, --create
          Create a new branch

  -b, --base <BASE>
          Base branch

          Defaults to default branch.

  -x, --execute <EXECUTE>
          Command to run after switch

  -f, --force
          Skip approval prompts

      --no-verify
          Skip all project hooks

  -h, --help
          Print help (see a summary with '-h')

Global Options:
  -C <path>
          Working directory for this command

      --config <path>
          User config file path

  -v, --verbose
          Show commands and debug info

Navigate between worktrees or create new ones. Switching to an existing worktree is just a directory change. With `--create`, a new branch and worktree are created, and hooks run.

```

### Examples

Switch to an existing worktree:

```bash
wt switch feature-auth
```

Create a new worktree for a fresh branch:

```bash
wt switch --create new-feature
```

Create from a specific base branch:

```bash
wt switch --create hotfix --base production
```

Switch to the previous worktree (like `cd -`):

```bash
wt switch -
```

### Creating Worktrees

The `--create` flag (or `-c`) creates a new branch from the default branch (or `--base`), sets up a worktree at `../{repo}.{branch}`, runs [post-create hooks](/hooks/#post-create) synchronously, then spawns [post-start hooks](/hooks/#post-start) in the background before switching to the new directory.

```bash
# Create from main (default)
wt switch --create api-refactor

# Create from a specific branch
wt switch --create emergency-fix --base release-2.0

# Create and open in editor
wt switch --create docs --execute "code ."

# Skip all hooks
wt switch --create temp --no-verify
```

### Shortcuts

Special symbols for common targets:

| Shortcut | Meaning |
|----------|---------|
| `-` | Previous worktree (like `cd -`) |
| `@` | Current branch's worktree |
| `^` | Default branch (main/master) |

```bash
wt switch -                              # Go back to previous worktree
wt switch ^                              # Switch to main worktree
wt switch --create bugfix --base=@       # Branch from current HEAD
```

### Hooks

When creating a worktree (`--create`), hooks run in this order:

1. **post-create** — Blocking, sequential. Typically: `npm install`, `cargo build`
2. **post-start** — Background, parallel. Typically: dev servers, file watchers

See [Hooks](/hooks/) for configuration details.

### How Arguments Are Resolved

Arguments resolve using **path-first lookup**:

1. Compute the expected path for the argument (using the configured path template)
2. If a worktree exists at that path, switch to it (regardless of what branch it's on)
3. Otherwise, treat the argument as a branch name

**Example**: If `repo.foo/` exists but is on branch `bar`:

- `wt switch foo` switches to `repo.foo/` (the `bar` branch worktree)
- `wt switch bar` also works (falls back to branch lookup)

<!-- END AUTO-GENERATED -->

</details>

<details id="wt-merge">
<summary><strong><code>wt merge [target]</code></strong> - Merge, push, and cleanup</summary>

<!-- ⚠️ AUTO-GENERATED from `wt merge --help-md` — edit source to update -->

```
wt merge — Merge worktree into target branch
Usage: wt merge [OPTIONS] [TARGET]

Arguments:
  [TARGET]
          Target branch

          Defaults to default branch.

Options:
      --no-squash
          Skip commit squashing

      --no-commit
          Skip commit, squash, and rebase

      --no-remove
          Keep worktree after merge

      --no-verify
          Skip all project hooks

  -f, --force
          Skip approval prompts

      --stage <STAGE>
          What to stage before committing [default: all]

          Possible values:
          - all:     Stage everything: untracked files + unstaged tracked changes
          - tracked: Stage tracked changes only (like git add -u)
          - none:    Stage nothing, commit only what's already in the index

  -h, --help
          Print help (see a summary with '-h')

Global Options:
  -C <path>
          Working directory for this command

      --config <path>
          User config file path

  -v, --verbose
          Show commands and debug info

Integrates the current branch into the target branch (default: main) and removes the worktree. All steps — commit, squash, rebase, push, cleanup — run automatically.

```

### Examples

Merge current worktree into main:

```bash
wt merge
```

Keep the worktree after merging:

```bash
wt merge --no-remove
```

Preserve commit history (no squash):

```bash
wt merge --no-squash
```

Skip hooks and approval prompts:

```bash
wt merge --no-verify --force
```

### The Pipeline

`wt merge` runs these steps in order:

1. **Commit** — Stages and commits uncommitted changes with an LLM-generated message. The `--stage` flag controls what gets staged: `all` (default), `tracked`, or `none`.

2. **Squash** — Combines multiple commits into one (like GitHub's "Squash and merge") with an LLM-generated message. The `--no-squash` flag preserves individual commits. A backup ref is saved to `refs/wt-backup/<branch>`.

3. **Rebase** — Rebases onto the target branch. Conflicts abort the merge immediately.

4. **Pre-merge hooks** — Project-defined commands run after rebase. Failures abort the merge.

5. **Push** — Fast-forward push to the local target branch. Non-fast-forward pushes are rejected.

6. **Cleanup** — Removes the worktree and branch. The `--no-remove` flag keeps the worktree.

7. **Post-merge hooks** — Project-defined commands run after cleanup. Failures are logged but don't abort.

### Hooks

When merging, hooks run in this order:

1. **pre-merge** — After rebase, before push. Failures abort the merge.
2. **post-merge** — After cleanup. Failures are logged but don't abort.

The `--no-verify` flag skips all hooks. See [Hooks](/hooks/) for configuration details.

<!-- END AUTO-GENERATED -->

</details>

<details>
<summary><strong><code>wt remove [worktree]</code></strong> - Remove worktree and branch</summary>

<!-- ⚠️ AUTO-GENERATED from `wt remove --help-md` — edit source to update -->

```
wt remove — Remove worktree and branch
Usage: wt remove [OPTIONS] [WORKTREES]...

Arguments:
  [WORKTREES]...
          Worktree or branch (@ for current)

Options:
      --no-delete-branch
          Keep branch after removal

  -D, --force-delete
          Delete unmerged branches

      --no-background
          Run removal in foreground

  -h, --help
          Print help (see a summary with '-h')

Global Options:
  -C <path>
          Working directory for this command

      --config <path>
          User config file path

  -v, --verbose
          Show commands and debug info

Cleans up finished work by removing worktrees and their branches. Without arguments, removes the current worktree and returns to the main worktree.

```

### Examples

Remove current worktree and branch:

```bash
wt remove
```

Remove a specific worktree:

```bash
wt remove feature-branch
```

Keep the branch after removing the worktree:

```bash
wt remove --no-delete-branch feature-branch
```

Remove multiple worktrees:

```bash
wt remove old-feature another-branch
```

Force-delete an unmerged branch:

```bash
wt remove -D experimental
```

### Branch Deletion

By default, branches are deleted only when their content is already integrated into the target branch (typically main). This works correctly with squash-merge and rebase workflows where commit ancestry isn't preserved but the file changes are.

The `-D` flag overrides this safety check and force-deletes unmerged branches. The `--no-delete-branch` flag prevents branch deletion entirely.

### Background Removal

Removal runs in the background by default — the command returns immediately so work can continue. Logs are written to `.git/wt-logs/{branch}-remove.log`.

The `--no-background` flag runs removal in the foreground (blocking).

### How Arguments Are Resolved

Arguments resolve using **path-first lookup**:

1. Compute the expected path for the argument (using the configured path template)
2. If a worktree exists at that path, use it (regardless of what branch it's on)
3. Otherwise, treat the argument as a branch name

**Example**: If `repo.foo/` exists but is on branch `bar`:

- `wt remove foo` removes `repo.foo/` and the `bar` branch
- `wt remove bar` also works (falls back to branch lookup)

**Shortcuts**: `@` (current worktree), `-` (previous worktree), `^` (main worktree)

<!-- END AUTO-GENERATED -->

</details>

<details id="wt-list">
<summary><strong><code>wt list</code></strong> - Show all worktrees and branches</summary>

<!-- ⚠️ AUTO-GENERATED from `wt list --help-md` — edit source to update -->

```
wt list — List worktrees and optionally branches
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

Show all worktrees with their status at a glance. The table includes uncommitted changes, divergence from main and remote, and optional CI status.

```

### Examples

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

### Status Symbols

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

### Columns

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

#### CI Status

The CI column (`--full`) shows pipeline status from GitHub/GitLab:

- `●` green — All checks passed
- `●` blue — Checks running
- `●` red — Checks failed
- `●` yellow — Merge conflicts with base
- `●` gray — No checks configured
- blank — No PR/MR found
- dimmed — Stale (unpushed local changes)

### JSON Output

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

<!-- END AUTO-GENERATED -->

</details>

<details id="wt-config">
<summary><strong><code>wt config</code></strong> - Manage configuration</summary>

<!-- ⚠️ AUTO-GENERATED from `wt config --help-md` — edit source to update -->

```
wt config — Manage configuration and shell integration
Usage: wt config [OPTIONS] <COMMAND>

Commands:
  shell      Shell integration setup
  create     Create user configuration file
  show       Show configuration files & locations
  cache      Manage caches (CI status, default branch)
  var        Get or set runtime variables (stored in git config)
  approvals  Manage command approvals

Options:
  -h, --help
          Print help (see a summary with '-h')

Global Options:
  -C <path>
          Working directory for this command

      --config <path>
          User config file path

  -v, --verbose
          Show commands and debug info

Manages configuration, shell integration, and runtime settings. The command provides subcommands for setup, inspection, and cache management.

```

### Examples

Install shell integration (required for directory switching):

```bash
wt config shell install
```

Create user config file with documented examples:

```bash
wt config create
```

Show current configuration and file locations:

```bash
wt config show
```

### Shell Integration

Shell integration allows Worktrunk to change the shell's working directory after `wt switch`. Without it, commands run in a subprocess and directory changes don't persist.

The `wt config shell install` command adds integration to the shell's config file. Manual installation:

```bash
# For bash: add to ~/.bashrc
eval "$(wt config shell init bash)"

# For zsh: add to ~/.zshrc
eval "$(wt config shell init zsh)"

# For fish: add to ~/.config/fish/config.fish
wt config shell init fish | source
```

### Configuration Files

**User config** — `~/.config/worktrunk/config.toml` (or `$WORKTRUNK_CONFIG_PATH`):

Personal settings like LLM commit generation, path templates, and default behaviors. The `wt config create` command generates a file with documented examples.

**Project config** — `.config/wt.toml` in repository root:

Project-specific hooks: post-create, post-start, pre-commit, pre-merge, post-merge. See [Hooks](/hooks/) for details.

### LLM Commit Messages

Worktrunk can generate commit messages using an LLM. Enable in user config:

```toml
[commit-generation]
command = "llm"
```

See [LLM Commits](/llm-commits/) for installation, provider setup, and customization.

<!-- END AUTO-GENERATED -->

</details>

<details>
<summary><strong><code>wt step</code></strong> - Building blocks for workflows</summary>

<!-- ⚠️ AUTO-GENERATED from `wt step --help-md` — edit source to update -->

```
wt step — Workflow building blocks
Usage: wt step [OPTIONS] <COMMAND>

Commands:
  commit       Commit changes with LLM commit message
  squash       Squash commits with LLM commit message
  push         Push changes to local target branch
  rebase       Rebase onto target
  post-create  Run post-create hook
  post-start   Run post-start hook
  pre-commit   Run pre-commit hook
  pre-merge    Run pre-merge hook
  post-merge   Run post-merge hook

Options:
  -h, --help
          Print help (see a summary with '-h')

Global Options:
  -C <path>
          Working directory for this command

      --config <path>
          User config file path

  -v, --verbose
          Show commands and debug info

Individual workflow steps for scripting and automation. Each subcommand performs one step of the `wt merge` pipeline — commit, squash, rebase, push, or run hooks — allowing custom workflows or manual intervention between steps.

```

### Examples

Commit with an LLM-generated message:

```bash
wt step commit
```

Squash all branch commits into one:

```bash
wt step squash
```

Run pre-merge hooks (tests, lints):

```bash
wt step pre-merge
```

Rebase onto main:

```bash
wt step rebase
```

### Use Cases

**Custom merge workflow** — Run steps individually when `wt merge` doesn't fit, such as adding manual review between squash and rebase:

```bash
wt step commit
wt step squash
# manual review here
wt step rebase
wt step pre-merge
wt step push
```

**CI integration** — Run hooks explicitly in CI environments:

```bash
wt step pre-merge --force  # skip approval prompts
```

### Subcommands

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

<!-- END AUTO-GENERATED -->

</details>

## FAQ

<details>
<summary><strong>What commands does Worktrunk execute?</strong></summary>

Worktrunk executes commands in three contexts:

1. **Project hooks** (project config: `.config/wt.toml`) - Automation for worktree lifecycle
2. **LLM commands** (user config: `~/.config/worktrunk/config.toml`) - Commit message generation
3. **--execute flag** - Commands provided explicitly

Commands from project hooks and LLM configuration require approval on first run. Approved commands are saved to user config under the project's configuration. If a command changes, Worktrunk requires new approval.

**Example approval prompt:**

<!-- ⚠️ AUTO-GENERATED from tests/integration_tests/snapshots/integration__integration_tests__shell_wrapper__tests__readme_example_approval_prompt.snap — edit source to update -->

```
🟡 repo needs approval to execute 3 commands:

⚪ post-create install:
   echo 'Installing dependencies...'

⚪ post-create build:
   echo 'Building project...'

⚪ post-create test:
   echo 'Running tests...'

💡 Allow and remember? [y/N]
```

<!-- END AUTO-GENERATED -->

Use `--force` to bypass prompts (useful for CI/automation).

</details>

<details>
<summary><strong>How does Worktrunk compare to alternatives?</strong></summary>

### vs. Branch Switching

Branch switching uses one directory, so only one agent can work at a time.
Worktrees give each agent its own directory.

### vs. Plain `git worktree`

Git's built-in worktree commands work but require manual lifecycle management:

```bash
# Plain git worktree workflow
git worktree add -b feature-branch ../myapp-feature main
cd ../myapp-feature
# ...work, commit, push...
cd ../myapp
git merge feature-branch
git worktree remove ../myapp-feature
git branch -d feature-branch
```

Worktrunk automates the full lifecycle:

```bash
wt switch --create feature-branch  # Creates worktree, runs setup hooks
# ...work...
wt merge                            # Squashes, merges, removes worktree
```

What `git worktree` doesn't provide:

- Consistent directory naming and cleanup validation
- Project-specific automation (install dependencies, start services)
- Unified status across all worktrees (commits, CI, conflicts, changes)

Worktrunk adds path management, lifecycle hooks, and `wt list --full` for viewing all worktrees—branches, uncommitted changes, commits ahead/behind, CI status, and conflicts—in a single view.

### vs. git-machete / git-town

Different scopes:

- **git-machete**: Branch stack management in a single directory
- **git-town**: Git workflow automation in a single directory
- **worktrunk**: Multi-worktree management with hooks and status aggregation

These tools can be used together—run git-machete or git-town inside individual worktrees.

### vs. Git TUIs (lazygit, gh-dash, etc.)

Git TUIs operate on a single repository. Worktrunk manages multiple worktrees,
runs automation hooks, and aggregates status across branches. TUIs work inside
each worktree directory.

</details>

<details>
<summary><strong>Installation fails with C compilation errors</strong></summary>

Errors related to tree-sitter or C compilation (C99 mode, `le16toh` undefined)
can be avoided by installing without syntax highlighting:

```bash
cargo install worktrunk --no-default-features
```

This disables bash syntax highlighting in command output but keeps all core functionality. The syntax highlighting feature requires C99 compiler support and can fail on older systems or minimal Docker images.

</details>

<details>
<summary><strong>How can I contribute?</strong></summary>

- Star the repo
- Try it out and [open an issue](https://github.com/max-sixty/worktrunk/issues) with feedback
- Send to a friend
- Post about it — [X](https://twitter.com/intent/tweet?text=Worktrunk%20%E2%80%94%20CLI%20for%20git%20worktree%20management&url=https%3A%2F%2Fgithub.com%2Fmax-sixty%2Fworktrunk) · [Reddit](https://www.reddit.com/submit?url=https%3A%2F%2Fgithub.com%2Fmax-sixty%2Fworktrunk&title=Worktrunk%20%E2%80%94%20CLI%20for%20git%20worktree%20management) · [LinkedIn](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fgithub.com%2Fmax-sixty%2Fworktrunk)

Thanks in advance!

</details>

<details>
<summary><strong>Any notes for developing this crate?</strong></summary>

### Running Tests

**Quick tests (no external dependencies):**

```bash
cargo test --lib --bins           # Unit tests (~200 tests)
cargo test --test integration     # Integration tests without shell tests (~300 tests)
```

**Full integration tests (requires bash, zsh, fish):**

```bash
cargo test --test integration --features shell-integration-tests
```

**Dependencies for shell integration tests:**

- bash, zsh, fish shells
- Quick setup: `./dev/setup-claude-code-web.sh` (installs shells on Linux)

### Releases

Use [cargo-release](https://github.com/crate-ci/cargo-release) to publish new versions:

```bash
cargo install cargo-release

# Bump version, update Cargo.lock, commit, tag, and push
cargo release patch --execute   # 0.1.0 -> 0.1.1
cargo release minor --execute   # 0.1.0 -> 0.2.0
cargo release major --execute   # 0.1.0 -> 1.0.0
```

This updates Cargo.toml and Cargo.lock, creates a commit and tag, then pushes to GitHub. The tag push triggers GitHub Actions to build binaries, create the release, and publish to crates.io.

Run without `--execute` to preview changes first.

### Updating Homebrew Formula

After `cargo release` completes and the GitHub release is created, update the [homebrew-worktrunk](https://github.com/max-sixty/homebrew-worktrunk) tap:

```bash
./dev/update-homebrew.sh
```

This script fetches the new tarball, computes the SHA256, updates the formula, and pushes to homebrew-worktrunk.

</details>

+++
title = "wt remove"
weight = 13

[extra]
group = "Commands"
+++

Cleans up finished work by removing worktrees and their branches. Without arguments, removes the current worktree and returns to the main worktree.

## Examples

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

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt remove --help-page` — edit cli.rs to update -->

```bash
wt remove - Remove worktree and branch
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
```

+++
title = "wt merge"
weight = 12

[extra]
group = "Commands"
+++

Integrates the current branch into the target branch (default: main) and removes the worktree. All steps — commit, squash, rebase, push, cleanup — run automatically.

## Examples

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

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt merge --help-page` — edit cli.rs to update -->

```bash
wt merge - Merge worktree into target branch
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
```

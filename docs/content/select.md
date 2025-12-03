+++
title = "wt select"
weight = 14

[extra]
group = "Commands"
+++

Interactive worktree picker with live preview. The selector shows worktree state at a glance — diff stats, commit history, and working tree status — without switching directories first.

## Examples

Open the interactive selector:

```bash
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

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt select --help-page` — edit cli.rs to update -->

```bash
wt select - Interactive worktree selector

Toggle preview tabs with 1/2/3 keys. Toggle preview visibility with alt-p.
Usage: wt select [OPTIONS]

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
```

+++
title = "wt step"
weight = 16

[extra]
group = "Commands"
+++

Individual workflow steps for scripting and automation. Each subcommand performs one step of the `wt merge` pipeline — commit, squash, rebase, push, or run hooks — allowing custom workflows or manual intervention between steps.

## Examples

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

## Use Cases

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

---

## Command Reference

<!-- ⚠️ AUTO-GENERATED from `wt step --help-page` — edit cli.rs to update -->

```bash
wt step - Workflow building blocks
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
```

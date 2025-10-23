function git-llm-message -d "Generate a commit message using LLM from diff and provided custom instruction"
    # This function uses an XML format to structure data for the LLM:
    #  - <git-info> contains branch and recent commit metadata
    #  - <git-diff> contains the staged changes
    # The LLM uses this structured format to generate a well-formatted commit message.

    argparse debug -- $argv
    or return # Exit if argparse finds an invalid option

    # Show informative message
    set files_count (git diff --cached --name-only | count)
    set stat_summary (git diff --cached --shortstat)

    # IMPORTANT: Using inline color codes in echo statements that are already
    # redirected to stderr ensures that color codes never contaminate stdout
    echo (set_color cyan)"🔄 Processing $files_count files. $stat_summary. Generating commit message..."(set_color normal) >&2

    # Create a temporary file that will be automatically cleaned up when the function exits
    set --local input_file (mktemp)

    # Ensure temp file is removed when function exits (unless --debug is passed)
    if set -q _flag_debug
        # When debugging, tell the user where to find the temp file
        echo (set_color -d)"💬 === Debug: Temporary file will be preserved at: $input_file ==="(set_color normal) >&2
    else
        function __cleanup --on-event fish_exit --inherit-variable input_file
            rm -f $input_file
        end
    end

    # Set default system instruction if empty
    # $argv now contains non-option arguments after argparse processes them.
    set --local actual_system_instruction
    if test (count $argv) -gt 0
        set actual_system_instruction $argv[1] # Use the first non-option arg as system instruction
    end
    set --local user_instruction (test -n "$actual_system_instruction" && echo "$actual_system_instruction" || echo "Write a concise, clear git commit message based on the provided diff.")

    # ----- Prepare LLM Input File (Instructions + Git Info + Diff) -----

    # Write detailed instructions (formerly part of system prompt) to input_file
    printf "%s\n" "Format
- First line: <50 chars, present tense, describes WHAT and WHY (not HOW).
- Blank line after first line.
- Optional details with proper line breaks explaining context. Commits with more substantial changes should have more details.
- Return ONLY the formatted message without quotes, code blocks, or preamble.

Style
- Do not give normative statements or otherwise speculate on why the change was made.
- Broadly match the style of the previous commit messages.
  - For example, if they're in conventional commit format, use conventional commits; if they're not, don't use conventional commits.

The context contains:
- <git-diff> with the staged changes. This is the ONLY content you should base your message on.
- <git-info> with branch name and recent commit message titles for style reference ONLY. DO NOT use their content to inform your message.

---
The following is the context for your task:
---
" >$input_file

    # printf "<git-diff>\n```diff" >>$input_file
    printf "<git-diff>\n```" >>$input_file
    git --no-pager diff --staged >>$input_file
    printf "\n```\n</git-diff>\n" >>$input_file

    # Add git context information
    printf "<git-info>\n" >>$input_file

    # Try to get current branch name
    set current_branch_name (git rev-parse --abbrev-ref HEAD 2>/dev/null)
    if test $status -eq 0; and test -n "$current_branch_name"
        printf "  <current-branch>%s</current-branch>\n" $current_branch_name >>$input_file
    end

    # Try to get recent commit messages
    set recent_commits_list (git log --pretty='format:%s' -n 5 --no-merges 2>/dev/null)
    if test $status -eq 0; and test (count $recent_commits_list) -gt 0
        printf "  <previous-commit-message-titles>\n" >>$input_file
        for commit_msg in $recent_commits_list
            printf "    <previous-commit-message-title>%s</previous-commit-message-title>\n" $commit_msg >>$input_file
        end
        printf "  </previous-commit-message-titles>\n" >>$input_file
    end

    printf "</git-info>\n\n" >>$input_file

    # Debug output if requested
    if set -q _flag_debug
        set input_size (stat -f %z $input_file)

        echo (set_color -d)"💬 === Debug: Temporary Files ==="(set_color normal) >&2
        echo "💬 Input file (instructions + git info + diff): $input_file ($input_size bytes)" >&2
        echo (set_color -d)"💬 === Debug: System Instruction (passed directly to LLM) ==="(set_color normal) >&2
        echo "💬 $user_instruction" >&2
        echo (set_color -d)"💬 === Debug: Input File ==="(set_color normal) >&2
        cat $input_file >&2
    end

    # Execute the command (will be traced if in debug mode)
    # Enable command tracing if debug mode is active
    if set -q _flag_debug
        echo (set_color -d)"💬 === Debug: Executing LLM Command ==="(set_color normal) >&2
        # Set local fish_trace that will automatically go out of scope
        set -l fish_trace 1
    end

    # Use different LLM configurations based on the OS
    switch (uname)
        case Darwin
            # macOS-specific model and settings
            cat $input_file | llm prompt -m gemini-2.5-flash \
                -o temperature 0 \
                -o google_search 0 \
                -o thinking_budget 0 \
                --no-log \
                --system "$user_instruction" \
                | read -z msg \
                || return 1
        case Linux '*'
            # Linux and any other OS uses default configs
            cat $input_file | llm prompt \
                --no-log \
                --system "$user_instruction" \
                | read -z msg \
                || return 1
    end

    # Show generated message to user with colors
    # Keep all color codes on stderr so they never contaminate stdout
    echo (set_color green)"🔍 Generated message:"(set_color normal) >&2

    # Display first line in bold
    set -l all_lines (string split \n -- "$msg")
    set -l first_line $all_lines[1]
    echo (set_color --bold)"$first_line"(set_color normal) >&2

    # Display the rest if it exists (preserving all formatting)
    if test (count $all_lines) -gt 1
        set -l remaining_lines $all_lines[2..]
        printf "%s\n" $remaining_lines >&2
    end
    echo "" >&2

    # Return just the message to stdout for piping to other tools
    # No color code cleanup needed because we never let them touch stdout
    # Use echo to preserve newlines properly
    echo -n "$msg"
end

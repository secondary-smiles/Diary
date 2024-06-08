# Diary

A simple and extensible `cli` tool built in rust to manage a diary.

Diary entries are created every day, and each addition is called a *snippet*. Adding notes to your daily entry will create a small collection of notes for each day.

## Usage
`diary` comes with a few subcommands to do different tasks.

```
Usage: diary <COMMAND>

Commands:
  add    Add a new snippet to today's entry [aliases: a]
  view   View an entry from today or in the past [aliases: v]
  build  Convert a diary entry to HTML [aliases: b]
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuration
`diary`'s configuration file can be found at `$XDG_CONFIG_HOME/diary/default-config.toml` (typically `~/.config/diary/default-config.toml`).

```toml
location = "/home/user/diary" # Absolute path to base location of diary entries. Defaults to $HOME/diary.
pager = "less" # Pager to use when viewing diary entries, defaults to $PAGER.
editor = "nvim" # Editor to use when writing snippets, defaults to $EDITOR.

[entry.frontmatter] # Content to prepend to a diary entry.
content = ""
cmd = '''printf "# %s's Diary" "$(whoami)"''' # Command to be invoked with sh -c the stdout will be captured and inserted into after entry.frontmatter.content

[snippet.frontmatter] # Content to prepend to a snippet.
content = "\n---\n"
cmd = '''printf "## At %s" "$(date +%R)"'''

[snippet.endmatter] # Content to append to a snippet.
content = "\n---\n"
cmd = ""

[build] # Options for when building entries to html files.
css = ["./path/to/style.css"] # A list of paths to css files to include.
script = ["./path/to/script.js"] # A list of paths to script files to include.

[build.frontmatter] # Content to add to the <head> tag in the built html.
content = ""
cmd = ""

[build.endmatter] # Content to add right before the closing </body> tag in the built html.
content = ""
cmd = ""
```


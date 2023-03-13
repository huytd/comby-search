# Comby Search

Comby Search is a convenience wrapper around [Comby](https://github.com/comby-tools/comby) to provide the code searching functionality.

![](metadata/screenshot.gif)

## Why Comby Search? Why Comby?

Because Comby by default provided the code rewrite function and does not have a pretty output when doing `-match-only`. Comby Search 
wrapped around Comby and parse the `-match-only` output to make it easier to search and see the result.

And why using Comby? See here for more details: https://github.com/comby-tools/comby#isnt-a-regex-approach-like-sed-good-enough

But TL;DR: Comby provides a better search comparing to Regex. It handles whitespaces automatically (if you ever tried search multilines of codes in Regex, you'll know), context aware (for example, it will not match commented code),...

## Dependencies

Comby Search rely on these tools, you should have them installed first:

- Comby (https://github.com/comby-tools/comby)
- Ripgrep (https://github.com/BurntSushi/ripgrep)

## Install

You can install via `cargo` with:

```
cargo install comby-search
```

Or compile it from source.

## Usage

To start a new search, run the command below. See [Comby's Documentation](https://comby.dev/docs/basic-usage) for more information about the search syntax.

```
$ cb '<search query>'
```

For example:

```
$ cb 'struct :[A] { :[B] }'
```

You can also limit the scope of the search to some specific files with:

```
$ cb 'struct :[A] { :[B] }' '*.ts,*.java,!*.js'
```

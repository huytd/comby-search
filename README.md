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

```
Usage: cb [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Your code search query

Options:
  -f, --filter <PATTERN>      File filter pattern, for example: '*.ts,!*.spec.ts'
  -l, --line-margin <MARGIN>  Number of lines to display before and after the matched search result
  -h, --help                  Print help
```

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
$ cb -f '*.ts,*.java,!*.js' 'struct :[A] { :[B] }'
```

By default, the matched result will be displayed with a margin of 3 lines before and after, you can change this with the `--line-margin` or `-l` option:

```
$cb -l 10 'foo'
```

## Examples

Here are some example search with `cb`.

1. Find all arrow functions in TSX files

```
$ cb -f '*.tsx,*.jsx' 'const :[1] = (:[2]) => { :[3] }'
```

2. Is there any call to the `superFoo` function with an empty string as argument?

```
$ cb 'superFoo(:[2] "" :[3])'
```

3. Find all for loop in all Java files, but not in test files

```
$ cb -f '!*Test.java' 'for ([:1]) { :[2] }'
```

4. Find all Rust functions that return a String

```
$ cb -f '*.rs' 'fn :[1] (:[2]) -> String'
```

5. Find Rust traits (can you tell the different?)

```
$ cb 'trait :[1] { fn :[2](:[3]); }'

# and

$ cb 'trait :[1] { fn :[2](:[3]) -> :[4]; }'
```

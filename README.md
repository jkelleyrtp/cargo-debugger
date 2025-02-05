# cdb: easily launch rust executables with vscode's debugger

`cdb` is a simple CLI tool that launches your cargo build with the vscode debugger attached.

![demo.gif](/assets/demo.gif)

## Installation

```sh
cargo install cargo-debugger
```

## Usage
cdb is an alias to `cargo rustc --message-format json-diagnostic-rendered-ansi ...` - so simply pass normal cargo arguments to cdb.

Any extra args after `--` will be passed to the executable under debug.

```sh
cdb --bin dioxus-cli -- serve --verbose --experimental-bundle-split --trace --release
```

This will launch your cargo build with the vscode debugger attached.

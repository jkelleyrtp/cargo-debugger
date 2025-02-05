# cargo-debugger: easily launch rust executables with vscode's debugger

`cargo-debugger` is a simple CLI tool that launches your cargo build with the vscode debugger attached.

![demo.gif](/assets/demo.gif)

## Installation

```sh
cargo install cargo-debugger
```

## Usage
cdb is an alias to `cargo rustc --message-format json-diagnostic-rendered-ansi ...` - so simply pass normal cargo arguments to cdb.

Any extra args after `--` will be passed to the executable under debug.

```sh
cargo debugger --bin dioxus-cli -- serve --verbose --experimental-bundle-split --trace --release
```

This will launch your cargo build with the vscode debugger attached.

## Tips

- Make sure your target executable has debug symbols. Release builds won't have them. Some custom profiles won't either.
- You can create aliases to cargo-debugger configurations your `.zshrc` or `.bashrc` to make it easier to launch your executables.

## License

MIT

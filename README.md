# cargo-debugger: easily launch rust executables with vscode's debugger

`cargo-debugger` is a simple CLI tool that launches your cargo build with the vscode debugger attached.

![demo.gif](https://github.com/jkelleyrtp/cargo-debugger/raw/main/assets/demo.gif)

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

Currently, we only support `cargo rustc` equivalents. We also only launch with the code-lldb debugger for vscode. If you'd like to use another editor or debugger, feel free to contribute!

## Tips

- Make sure your target executable has debug symbols. Release builds won't have them. Some custom profiles won't either.
- You can create aliases to cargo-debugger configurations your `.zshrc` or `.bashrc` to make it easier to launch your executables.
- We use the format outlined here for deep-link urls. https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#debugging-externally-launched-code

## Future Ideas

- Support more cargo commands (test, bench, etc)
- Attach to a running process (https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#attaching-debugger-to-the-current-process-c)
- WASM support
- More editors and debuggers

## License

MIT

use anyhow::Context;
use std::{env::current_dir, process::Stdio};
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

// cargo debugger --package dioxus-cli --bin dioxus-bin -- serve --verbose --experimental-bundle-split --trace --release

/// Run any of your workspace's binaries with the debugger attached.
///
/// cdb serve
async fn run() -> anyhow::Result<()> {
    let mut all_args: Vec<String> = std::env::args().collect();

    // if running as cargo debugger, then remove the debugger arg
    if all_args.get(1) == Some(&"debugger".to_string()) {
        all_args.remove(1);
    }

    let mut parsing_cargo_args = true;
    let mut parsing_env_args = true;
    let mut cargo_args = vec![];
    let mut process_env_args = vec![];
    let mut rest_args = vec![];

    println!("all args: {:?}", all_args);

    for arg in all_args.iter().skip(1) {
        // Switch to parsing the rest of the args
        if arg == "--" && parsing_cargo_args {
            parsing_cargo_args = false;
            continue;
        }

        // Attempt to parse env pairs
        if parsing_env_args && !parsing_cargo_args {
            if let Some((left, right)) = arg.split_once('=') {
                if left.is_empty() || right.is_empty() {
                    return Err(anyhow::anyhow!("Invalid argument: {}", arg));
                }

                process_env_args.push((left.to_string(), right.to_string()));
            } else {
                parsing_env_args = false;
                rest_args.push(arg.to_string());
            }
            continue;
        }

        // Attempt to parse cargo args
        if parsing_cargo_args {
            cargo_args.push(arg.to_string());
            continue;
        }

        // Attempt to parse rest args
        rest_args.push(arg.to_string());
    }

    let mut child = tokio::process::Command::new("cargo")
        .arg("rustc")
        .args(cargo_args)
        .arg("--message-format")
        .arg("json-diagnostic-rendered-ansi")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn cargo process")?;

    let stdout = tokio::io::BufReader::new(child.stdout.take().unwrap());
    let stderr = tokio::io::BufReader::new(child.stderr.take().unwrap());
    let mut output_location = None;
    let mut stdout = stdout.lines();
    let mut stderr = stderr.lines();

    loop {
        use cargo_metadata::Message;

        let line = tokio::select! {
            Ok(Some(line)) = stdout.next_line() => line,
            Ok(Some(line)) = stderr.next_line() => line,
            else => break,
        };

        let Some(Ok(message)) = Message::parse_stream(std::io::Cursor::new(line)).next() else {
            continue;
        };

        match message {
            Message::CompilerArtifact(artifact) => match artifact.executable {
                Some(i) => output_location = Some(i),
                None => {}
            },
            Message::CompilerMessage(compiler_message) => {
                if let Some(rendered) = compiler_message.message.rendered {
                    println!("{rendered}");
                }
            }
            Message::BuildScriptExecuted(_build_script) => {}
            Message::BuildFinished(build_finished) => {
                if !build_finished.success {
                    return Err(anyhow::anyhow!(
                            "Cargo build failed, signaled by the compiler. Toggle tracing mode (press `t`) for more information."
                        )
                        .into());
                }
            }
            Message::TextLine(word) => println!("{word}"),
            _ => {}
        }
    }

    let output_location =
        output_location.context("Failed to find output location. Build must've failed.")?;

    let cur_dir = current_dir().context("Failed to get current directory")?;

    let args = rest_args
        .iter()
        .map(|arg| format!("'{}'", urlencoding::encode(arg)))
        .collect::<Vec<_>>()
        .join(", ");

    let env = process_env_args
        .iter()
        .map(|(k, v)| format!("'{}': '{}'", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join(", ");

    let url = format!(
        "vscode://vadimcn.vscode-lldb/launch/config?{{ 'cwd': {cwd}, 'program': {program}, 'args': [{args}], 'env': {{ {env} }} }}",
        cwd = cur_dir.canonicalize()?.to_str().unwrap(),
        program = output_location,
        args = args,
        env = env
    );

    tokio::process::Command::new("code")
        .args(&["--open-url", &url])
        .output()
        .await
        .context("Failed to launch code")?;

    Ok(())
}

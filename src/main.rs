mod api;
mod state;

use anyhow::{Context, Result};
use rhai::Engine;
use state::paths::SHEBANG_PREFIX;

const HELP_TEXT: &str = "\
rhai — Rhai scripting runtime

USAGE:
    rhai <script.rhai> [args...]
    rhai --help

EXAMPLES:
    rhai build.rhai
    rhai deploy.rhai --env prod
    rhai fetch.rhai https://api.github.com
";

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();

    let mut cli_args: Vec<String> = std::env::args().skip(1).collect();

    if cli_args.is_empty() || cli_args[0] == "--help" || cli_args[0] == "-h" {
        print!("{HELP_TEXT}");
        return Ok(());
    }

    let script_path = cli_args.remove(0);
    let script_args = cli_args;

    let source = std::fs::read_to_string(&script_path)
        .with_context(|| format!("Cannot read script: {script_path}"))?;

    let source = strip_shebang(source);

    let mut engine = Engine::new();
    api::fs::register(&mut engine);
    api::env::register(&mut engine);
    api::http::register(&mut engine);
    api::shell::register(&mut engine, script_args);

    engine.on_print(|s| println!("{s}"));
    engine.on_debug(|s, _, _| eprintln!("[debug] {s}"));

    engine
        .run(&source)
        .with_context(|| format!("Script error in {script_path}"))?;

    Ok(())
}

fn strip_shebang(source: String) -> String {
    if source.starts_with(SHEBANG_PREFIX) {
        source
            .lines()
            .skip(1)
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        source
    }
}

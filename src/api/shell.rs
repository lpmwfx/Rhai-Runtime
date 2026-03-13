use rhai::{Dynamic, EvalAltResult, Map};

type RhaiResult = Result<Dynamic, Box<EvalAltResult>>;

fn rhai_err(msg: impl ToString) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(msg.to_string().into(), rhai::Position::NONE).into()
}

pub fn sh(cmd: &str) -> RhaiResult {
    let output = std::process::Command::new("sh")
        .args(["-c", cmd])
        .output()
        .map_err(|e| rhai_err(format!("sh: {e}")))?;

    let mut map = Map::new();
    map.insert("stdout".into(), Dynamic::from(String::from_utf8_lossy(&output.stdout).into_owned()));
    map.insert("stderr".into(), Dynamic::from(String::from_utf8_lossy(&output.stderr).into_owned()));
    map.insert("code".into(),   Dynamic::from(output.status.code().unwrap_or(-1) as i64));
    map.insert("ok".into(),     Dynamic::from(output.status.success()));
    Ok(Dynamic::from(map))
}

pub fn sh_ok(cmd: &str) -> RhaiResult {
    let output = std::process::Command::new("sh")
        .args(["-c", cmd])
        .output()
        .map_err(|e| rhai_err(format!("sh_ok: {e}")))?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(output.status.code().unwrap_or(1));
    }
    Ok(Dynamic::UNIT)
}

pub fn exit(code: i64) {
    std::process::exit(code as i32);
}

pub fn register(engine: &mut rhai::Engine, script_args: Vec<String>) {
    engine.register_fn("sh",     sh);
    engine.register_fn("sh_ok",  sh_ok);
    engine.register_fn("exit",   exit);
    engine.register_fn("args",   move || -> Dynamic {
        Dynamic::from(
            script_args
                .iter()
                .map(|s| Dynamic::from(s.clone()))
                .collect::<rhai::Array>(),
        )
    });
}

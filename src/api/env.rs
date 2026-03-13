use rhai::{Dynamic, EvalAltResult, Map};

type RhaiResult = Result<Dynamic, Box<EvalAltResult>>;

fn rhai_err(msg: impl ToString) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(msg.to_string().into(), rhai::Position::NONE).into()
}

pub fn env(key: &str) -> RhaiResult {
    std::env::var(key)
        .map(Dynamic::from)
        .map_err(|_| rhai_err(format!("env: '{key}' is not set")))
}

pub fn env_or(key: &str, default: &str) -> Dynamic {
    Dynamic::from(std::env::var(key).unwrap_or_else(|_| default.to_owned()))
}

pub fn set_env(key: &str, val: &str) {
    unsafe { std::env::set_var(key, val) };
}

pub fn env_map() -> Dynamic {
    let map: Map = std::env::vars()
        .map(|(k, v)| (k.into(), Dynamic::from(v)))
        .collect();
    Dynamic::from(map)
}

pub fn register(engine: &mut rhai::Engine) {
    engine.register_fn("env",     env);
    engine.register_fn("env_or",  env_or);
    engine.register_fn("set_env", set_env);
    engine.register_fn("env_map", env_map);
}

use rhai::{Dynamic, EvalAltResult};

type RhaiResult = Result<Dynamic, Box<EvalAltResult>>;

fn rhai_err(msg: impl ToString) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(msg.to_string().into(), rhai::Position::NONE).into()
}

pub fn http_get(url: &str) -> RhaiResult {
    ureq::get(url)
        .call()
        .map_err(|e| rhai_err(format!("http_get: {e}")))?
        .into_string()
        .map(Dynamic::from)
        .map_err(|e| rhai_err(format!("http_get read: {e}")))
}

pub fn http_get_json(url: &str) -> RhaiResult {
    let body: serde_json::Value = ureq::get(url)
        .call()
        .map_err(|e| rhai_err(format!("http_get_json: {e}")))?
        .into_json()
        .map_err(|e| rhai_err(format!("http_get_json parse: {e}")))?;

    json_to_dynamic(body)
}

pub fn http_post(url: &str, body: &str) -> RhaiResult {
    ureq::post(url)
        .set("Content-Type", "application/json")
        .send_string(body)
        .map_err(|e| rhai_err(format!("http_post: {e}")))?
        .into_string()
        .map(Dynamic::from)
        .map_err(|e| rhai_err(format!("http_post read: {e}")))
}

fn json_to_dynamic(val: serde_json::Value) -> RhaiResult {
    match val {
        serde_json::Value::Null        => Ok(Dynamic::UNIT),
        serde_json::Value::Bool(b)     => Ok(Dynamic::from(b)),
        serde_json::Value::Number(n)   => {
            if let Some(i) = n.as_i64() {
                Ok(Dynamic::from(i))
            } else {
                Ok(Dynamic::from(n.as_f64().unwrap_or(0.0)))
            }
        }
        serde_json::Value::String(s)   => Ok(Dynamic::from(s)),
        serde_json::Value::Array(arr)  => {
            let items: Result<rhai::Array, _> = arr.into_iter().map(json_to_dynamic).collect();
            Ok(Dynamic::from(items?))
        }
        serde_json::Value::Object(obj) => {
            let map: Result<rhai::Map, _> = obj
                .into_iter()
                .map(|(k, v)| json_to_dynamic(v).map(|d| (k.into(), d)))
                .collect();
            Ok(Dynamic::from(map?))
        }
    }
}

pub fn register(engine: &mut rhai::Engine) {
    engine.register_fn("http_get",      http_get);
    engine.register_fn("http_get_json", http_get_json);
    engine.register_fn("http_post",     http_post);
}

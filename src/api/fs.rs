use rhai::{Dynamic, EvalAltResult};
use std::path::Path;
use crate::state::sizes::READ_DIR_CAPACITY;

type RhaiResult = Result<Dynamic, Box<EvalAltResult>>;

fn rhai_err(msg: impl ToString) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(msg.to_string().into(), rhai::Position::NONE).into()
}

pub fn read_file(path: &str) -> RhaiResult {
    std::fs::read_to_string(path)
        .map(Dynamic::from)
        .map_err(|e| rhai_err(format!("read_file: {e}")))
}

pub fn write_file(path: &str, content: &str) -> RhaiResult {
    std::fs::write(path, content)
        .map(|_| Dynamic::UNIT)
        .map_err(|e| rhai_err(format!("write_file: {e}")))
}

pub fn append_file(path: &str, content: &str) -> RhaiResult {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|e| rhai_err(format!("append_file open: {e}")))?;
    file.write_all(content.as_bytes())
        .map(|_| Dynamic::UNIT)
        .map_err(|e| rhai_err(format!("append_file write: {e}")))
}

pub fn remove_file(path: &str) -> RhaiResult {
    let p = Path::new(path);
    let result = if p.is_dir() {
        std::fs::remove_dir_all(p)
    } else {
        std::fs::remove_file(p)
    };
    result
        .map(|_| Dynamic::UNIT)
        .map_err(|e| rhai_err(format!("remove_file: {e}")))
}

pub fn copy_file(src: &str, dst: &str) -> RhaiResult {
    std::fs::copy(src, dst)
        .map(|_| Dynamic::UNIT)
        .map_err(|e| rhai_err(format!("copy_file: {e}")))
}

pub fn move_file(src: &str, dst: &str) -> RhaiResult {
    std::fs::rename(src, dst)
        .map(|_| Dynamic::UNIT)
        .map_err(|e| rhai_err(format!("move_file: {e}")))
}

pub fn make_dir(path: &str) -> RhaiResult {
    std::fs::create_dir_all(path)
        .map(|_| Dynamic::UNIT)
        .map_err(|e| rhai_err(format!("make_dir: {e}")))
}

pub fn read_dir(path: &str) -> RhaiResult {
    let entries = std::fs::read_dir(path)
        .map_err(|e| rhai_err(format!("read_dir: {e}")))?;
    let mut paths: rhai::Array = Vec::with_capacity(READ_DIR_CAPACITY);
    for entry in entries {
        let entry = entry.map_err(|e| rhai_err(format!("read_dir entry: {e}")))?;
        paths.push(Dynamic::from(entry.path().to_string_lossy().into_owned()));
    }
    Ok(Dynamic::from(paths))
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn file_size(path: &str) -> RhaiResult {
    std::fs::metadata(path)
        .map(|m| Dynamic::from(m.len() as i64))
        .map_err(|e| rhai_err(format!("file_size: {e}")))
}

pub fn register(engine: &mut rhai::Engine) {
    engine.register_fn("read_file",  read_file);
    engine.register_fn("write_file", write_file);
    engine.register_fn("append_file", append_file);
    engine.register_fn("remove_file", remove_file);
    engine.register_fn("copy_file",  copy_file);
    engine.register_fn("move_file",  move_file);
    engine.register_fn("make_dir",   make_dir);
    engine.register_fn("read_dir",   read_dir);
    engine.register_fn("path_exists", path_exists);
    engine.register_fn("is_file",    is_file);
    engine.register_fn("is_dir",     is_dir);
    engine.register_fn("file_size",  file_size);
}
